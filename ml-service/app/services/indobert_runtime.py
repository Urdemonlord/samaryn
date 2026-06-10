from __future__ import annotations

import json
import logging
import re
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any

logger = logging.getLogger(__name__)

NON_WORD_RE = re.compile(r"[^\w\s]")
MULTI_SPACE_RE = re.compile(r"\s+")

LABEL_TO_ACTION = {
    "SAFE": "allow",
    "PROMPT_INJECTION": "block",
    "OUT_OF_DOMAIN": "escalate",
}

ID_TO_LABEL = {
    0: "SAFE",
    1: "PROMPT_INJECTION",
    2: "OUT_OF_DOMAIN",
}


@dataclass
class ClassificationResult:
    label: str
    confidence: float
    source: str
    action: str
    normalized_text: str

    def to_payload(self) -> dict[str, Any]:
        payload = asdict(self)
        payload["confidence"] = round(self.confidence, 4)
        return payload


def normalize_text(text: str) -> str:
    normalized = str(text or "").strip().lower()
    normalized = NON_WORD_RE.sub(" ", normalized)
    normalized = MULTI_SPACE_RE.sub(" ", normalized).strip()
    return normalized


def _load_training_max_length(model_dir: Path) -> int | None:
    metadata_path = model_dir / "training_metadata.json"
    if not metadata_path.exists():
        return None

    try:
        payload = json.loads(metadata_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError:
        logger.warning("Failed to parse training metadata at %s", metadata_path)
        return None

    max_length = payload.get("max_length")
    if isinstance(max_length, int) and max_length > 0:
        return max_length
    return None


class IndoBertOnnxClassifier:
    def __init__(self, model_dir: str | Path) -> None:
        self.model_dir = Path(model_dir)
        self.onnx_path = self.model_dir / "model.onnx"
        self._session = None
        self._tokenizer = None
        self._max_length = None
        self._load_error: str | None = None

    def is_available(self) -> bool:
        return self.onnx_path.exists()

    def _ensure_loaded(self) -> None:
        if self._load_error is not None:
            raise RuntimeError(self._load_error)
        if self._session is not None and self._tokenizer is not None and self._max_length is not None:
            return

        if not self.is_available():
            raise RuntimeError(f"ONNX model not found at {self.onnx_path}")

        try:
            import onnxruntime as ort
            from transformers import AutoTokenizer
        except ImportError as exc:
            message = "Missing ONNX runtime dependencies for IndoBERT classifier"
            self._load_error = message
            raise RuntimeError(message) from exc

        tokenizer = AutoTokenizer.from_pretrained(
            str(self.model_dir),
            local_files_only=True,
        )
        session = ort.InferenceSession(
            str(self.onnx_path),
            providers=["CPUExecutionProvider"],
        )

        model_limit = 512
        config_path = self.model_dir / "config.json"
        if config_path.exists():
            try:
                config_payload = json.loads(config_path.read_text(encoding="utf-8"))
                max_position_embeddings = config_payload.get("max_position_embeddings")
                if isinstance(max_position_embeddings, int) and max_position_embeddings > 0:
                    model_limit = max_position_embeddings
            except json.JSONDecodeError:
                logger.warning("Failed to parse config.json at %s", config_path)

        tokenizer_limit = getattr(tokenizer, "model_max_length", None)
        if not isinstance(tokenizer_limit, int) or tokenizer_limit <= 0 or tokenizer_limit >= 100000:
            tokenizer_limit = model_limit

        metadata_limit = _load_training_max_length(self.model_dir)
        max_length = min(model_limit, tokenizer_limit)
        if metadata_limit is not None:
            max_length = min(max_length, metadata_limit)

        self._tokenizer = tokenizer
        self._session = session
        self._max_length = max_length
        logger.info("Loaded IndoBERT ONNX model from %s", self.onnx_path)

    def predict(self, text: str) -> ClassificationResult:
        self._ensure_loaded()

        import numpy as np

        normalized = normalize_text(text)
        tokenized = self._tokenizer(
            normalized,
            truncation=True,
            max_length=self._max_length,
            padding=True,
            return_tensors="np",
        )

        input_names = {input_meta.name for input_meta in self._session.get_inputs()}
        ort_inputs = {}
        for key, value in tokenized.items():
            if key in input_names:
                ort_inputs[key] = np.ascontiguousarray(value.astype(np.int64, copy=False))

        if not ort_inputs:
            raise RuntimeError(f"No compatible ONNX inputs found for model {self.onnx_path}")

        outputs = self._session.run(None, ort_inputs)
        logits = np.asarray(outputs[0])[0]
        shifted_logits = logits - np.max(logits)
        exp_logits = np.exp(shifted_logits)
        probabilities = exp_logits / np.sum(exp_logits)
        predicted_id = int(np.argmax(probabilities).item())
        confidence = float(probabilities[predicted_id].item())
        label = ID_TO_LABEL[predicted_id]
        return ClassificationResult(
            label=label,
            confidence=confidence,
            source="indobert-onnx",
            action=LABEL_TO_ACTION[label],
            normalized_text=normalized,
        )
