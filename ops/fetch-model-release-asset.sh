#!/usr/bin/env bash
set -euo pipefail

REPO="${SAMARYN_MODEL_REPOSITORY:-${GITHUB_REPOSITORY:-Urdemonlord/samaryn}}"
DEST_PATH="${SAMARYN_MODEL_DEST_PATH:-ml-service/models/indobert-agentwa/model.onnx}"
ASSET_NAME="${SAMARYN_MODEL_ASSET_NAME:-model.onnx}"
CHECKSUM_ASSET_NAME="${SAMARYN_MODEL_CHECKSUM_ASSET_NAME:-${ASSET_NAME}.sha256}"
RELEASE_TAG="${SAMARYN_MODEL_RELEASE_TAG:-}"
TMP_DIR="$(mktemp -d)"

cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

require_cmd() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "Missing required command: $1" >&2
    exit 1
  fi
}

require_cmd gh
require_cmd sha256sum

if [ -z "${GH_TOKEN:-${GITHUB_TOKEN:-}}" ]; then
  echo "GH_TOKEN or GITHUB_TOKEN is required to download the model release asset" >&2
  exit 1
fi

mkdir -p "$(dirname "$DEST_PATH")"

download_asset() {
  local pattern="$1"
  if [ -n "$RELEASE_TAG" ]; then
    gh release download "$RELEASE_TAG" \
      --repo "$REPO" \
      --pattern "$pattern" \
      --dir "$TMP_DIR" \
      --clobber
  else
    gh release download \
      --repo "$REPO" \
      --pattern "$pattern" \
      --dir "$TMP_DIR" \
      --clobber
  fi
}

echo "Downloading model asset '$ASSET_NAME' from GitHub release (${RELEASE_TAG:-latest}) in $REPO"
download_asset "$ASSET_NAME"

MODEL_PATH="$TMP_DIR/$ASSET_NAME"
if [ ! -f "$MODEL_PATH" ]; then
  echo "Downloaded asset missing: $MODEL_PATH" >&2
  exit 1
fi

echo "Downloading checksum asset '$CHECKSUM_ASSET_NAME'"
download_asset "$CHECKSUM_ASSET_NAME"

CHECKSUM_PATH="$TMP_DIR/$CHECKSUM_ASSET_NAME"
if [ ! -f "$CHECKSUM_PATH" ]; then
  echo "Downloaded checksum asset missing: $CHECKSUM_PATH" >&2
  exit 1
fi

CHECKSUM_FILE_FOR_VERIFY="$TMP_DIR/checksums.txt"
if grep -Eq '^[0-9a-fA-F]{64}[[:space:]]+[* ]' "$CHECKSUM_PATH"; then
  cp "$CHECKSUM_PATH" "$CHECKSUM_FILE_FOR_VERIFY"
else
  CHECKSUM_VALUE="$(tr -d '[:space:]' < "$CHECKSUM_PATH")"
  if ! printf '%s' "$CHECKSUM_VALUE" | grep -Eq '^[0-9a-fA-F]{64}$'; then
    echo "Checksum file must contain a SHA256 hash or sha256sum-compatible line" >&2
    exit 1
  fi
  printf '%s  %s\n' "$CHECKSUM_VALUE" "$ASSET_NAME" > "$CHECKSUM_FILE_FOR_VERIFY"
fi

(
  cd "$TMP_DIR"
  sha256sum -c "$CHECKSUM_FILE_FOR_VERIFY"
)

install -m 0644 "$MODEL_PATH" "$DEST_PATH"

FILE_SIZE="$(stat -c '%s' "$DEST_PATH")"
FILE_SHA256="$(sha256sum "$DEST_PATH" | awk '{print $1}')"

echo "Model stored at: $DEST_PATH"
echo "Model size: $FILE_SIZE bytes"
echo "Model sha256: $FILE_SHA256"
