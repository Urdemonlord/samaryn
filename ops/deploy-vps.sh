#!/usr/bin/env bash
set -euo pipefail

ARTIFACT_PATH="${1:-/tmp/samaryn-release.tgz}"
TMP_DIR="$(mktemp -d /tmp/samaryn-deploy.XXXXXX)"
NETWORK_NAME="samaryn_default"
GATEWAY_CONTAINER="samaryn-gateway"
ML_CONTAINER="samaryn-ml-service"
GATEWAY_IMAGE_NEW="samaryn-src-gateway:latest"
ML_IMAGE_NEW="samaryn-ml-service:latest"

cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

get_current_env_value() {
  local container="$1"
  local key="$2"
  docker inspect --format '{{range .Config.Env}}{{println .}}{{end}}' "$container" 2>/dev/null | awk -F= -v k="$key" '$1==k {sub($1"=", ""); print; exit}'
}

wait_container_health() {
  local container="$1"
  local attempts="${2:-30}"
  local sleep_seconds="${3:-2}"
  local status
  for _ in $(seq 1 "$attempts"); do
    status="$(docker inspect --format '{{if .State.Health}}{{.State.Health.Status}}{{else}}{{.State.Status}}{{end}}' "$container" 2>/dev/null || true)"
    if [ "$status" = "healthy" ] || [ "$status" = "running" ]; then
      return 0
    fi
    sleep "$sleep_seconds"
  done
  docker logs --tail 100 "$container" 2>/dev/null || true
  return 1
}

wait_gateway_http_health() {
  local attempts="${1:-20}"
  local sleep_seconds="${2:-2}"
  for _ in $(seq 1 "$attempts"); do
    if curl -fsS http://127.0.0.1:8080/health >/dev/null 2>&1; then
      curl -fsS http://127.0.0.1:8080/health
      return 0
    fi
    sleep "$sleep_seconds"
  done
  return 1
}

ensure_network() {
  docker network inspect "$NETWORK_NAME" >/dev/null 2>&1 || docker network create "$NETWORK_NAME" >/dev/null
}

run_ml_service() {
  local image_ref="$1"
  docker rm -f "$ML_CONTAINER" >/dev/null 2>&1 || true
  docker run -d \
    --name "$ML_CONTAINER" \
    --network "$NETWORK_NAME" \
    --restart unless-stopped \
    -e ML_SERVICE_LOG_LEVEL=info \
    -e ML_SERVICE_MODEL_DIR=/app/models/indobert-agentwa \
    "$image_ref" >/dev/null
}

run_gateway() {
  local image_ref="$1"
  local meowlabs_key openai_key anthropic_key gemini_key openrouter_key samaryn_auth_key
  meowlabs_key="$(get_current_env_value "$GATEWAY_CONTAINER" MEOWLABS_API_KEY || true)"
  openai_key="$(get_current_env_value "$GATEWAY_CONTAINER" OPENAI_API_KEY || true)"
  anthropic_key="$(get_current_env_value "$GATEWAY_CONTAINER" ANTHROPIC_API_KEY || true)"
  gemini_key="$(get_current_env_value "$GATEWAY_CONTAINER" GEMINI_API_KEY || true)"
  openrouter_key="$(get_current_env_value "$GATEWAY_CONTAINER" OPENROUTER_API_KEY || true)"
  samaryn_auth_key="$(get_current_env_value "$GATEWAY_CONTAINER" SAMARYN__AUTH_KEYS || true)"

  [ -z "$meowlabs_key" ] && meowlabs_key="${MEOWLABS_API_KEY:-}"
  [ -z "$openai_key" ] && openai_key="${OPENAI_API_KEY:-}"
  [ -z "$anthropic_key" ] && anthropic_key="${ANTHROPIC_API_KEY:-}"
  [ -z "$gemini_key" ] && gemini_key="${GEMINI_API_KEY:-}"
  [ -z "$openrouter_key" ] && openrouter_key="${OPENROUTER_API_KEY:-}"
  [ -z "$samaryn_auth_key" ] && samaryn_auth_key="${SAMARYN__AUTH_KEYS:-}"

  docker rm -f "$GATEWAY_CONTAINER" >/dev/null 2>&1 || true

  local cmd=(docker run -d
    --name "$GATEWAY_CONTAINER"
    --network "$NETWORK_NAME"
    --restart unless-stopped
    -p 8080:8080
    -e SAMARYN__ML_SERVICE__URL=http://samaryn-ml-service:8000
    -e SAMARYN__SERVER__HOST=0.0.0.0
    -e SAMARYN__SERVER__PORT=8080
    -e RUST_LOG=info,tower_http=debug)

  [ -n "$meowlabs_key" ] && cmd+=(-e "MEOWLABS_API_KEY=$meowlabs_key")
  [ -n "$openai_key" ] && cmd+=(-e "OPENAI_API_KEY=$openai_key")
  [ -n "$anthropic_key" ] && cmd+=(-e "ANTHROPIC_API_KEY=$anthropic_key")
  [ -n "$gemini_key" ] && cmd+=(-e "GEMINI_API_KEY=$gemini_key")
  [ -n "$openrouter_key" ] && cmd+=(-e "OPENROUTER_API_KEY=$openrouter_key")
  [ -n "$samaryn_auth_key" ] && cmd+=(-e "SAMARYN__AUTH_KEYS=$samaryn_auth_key")

  cmd+=("$image_ref")
  "${cmd[@]}" >/dev/null
}

rollback() {
  local old_gateway_image_id="$1"
  local old_ml_image_id="$2"
  echo "[rollback] Restoring previous Samaryn containers"
  ensure_network
  if [ -n "$old_ml_image_id" ]; then
    run_ml_service "$old_ml_image_id"
    wait_container_health "$ML_CONTAINER" 30 2 || true
  fi
  if [ -n "$old_gateway_image_id" ]; then
    run_gateway "$old_gateway_image_id"
    wait_container_health "$GATEWAY_CONTAINER" 30 2 || true
    wait_gateway_http_health 20 2 || true
  fi
}

if [ ! -f "$ARTIFACT_PATH" ]; then
  echo "Artifact not found: $ARTIFACT_PATH" >&2
  exit 1
fi

OLD_GATEWAY_IMAGE_ID="$(docker inspect --format '{{.Image}}' "$GATEWAY_CONTAINER" 2>/dev/null || true)"
OLD_ML_IMAGE_ID="$(docker inspect --format '{{.Image}}' "$ML_CONTAINER" 2>/dev/null || true)"

echo "[1/7] Extract artifact"
tar -xzf "$ARTIFACT_PATH" -C "$TMP_DIR"

echo "[2/7] Load Docker images"
gunzip -c "$TMP_DIR/samaryn-gateway-image.tar.gz" | docker load >/dev/null
gunzip -c "$TMP_DIR/samaryn-ml-service-image.tar.gz" | docker load >/dev/null

echo "[3/7] Ensure Docker network"
ensure_network

echo "[4/7] Restart ML service"
if ! run_ml_service "$ML_IMAGE_NEW" || ! wait_container_health "$ML_CONTAINER" 30 2; then
  echo "ML service failed to become healthy" >&2
  rollback "$OLD_GATEWAY_IMAGE_ID" "$OLD_ML_IMAGE_ID"
  exit 7
fi

echo "[5/7] Restart gateway"
if ! run_gateway "$GATEWAY_IMAGE_NEW" || ! wait_container_health "$GATEWAY_CONTAINER" 30 2; then
  echo "Gateway container failed to become healthy" >&2
  rollback "$OLD_GATEWAY_IMAGE_ID" "$OLD_ML_IMAGE_ID"
  exit 8
fi

echo "[6/7] HTTP health check"
if ! wait_gateway_http_health 20 2; then
  echo "Gateway HTTP health check failed" >&2
  rollback "$OLD_GATEWAY_IMAGE_ID" "$OLD_ML_IMAGE_ID"
  exit 9
fi

echo "[7/7] Done"
rm -f "$ARTIFACT_PATH"
