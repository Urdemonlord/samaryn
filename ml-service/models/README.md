This folder stores the local IndoBERT ONNX bundle used by the Samaryn ML service.

Required files:
- `indobert-agentwa/model.onnx`
- `indobert-agentwa/config.json`
- `indobert-agentwa/tokenizer.json`
- `indobert-agentwa/tokenizer_config.json`
- `indobert-agentwa/training_metadata.json`

Source bundle:
- ONNX graph copied from `C:\Users\click\indobert-agentwa\model\model.onnx`
- tokenizer and metadata copied from `C:\Users\click\indobert-agentwa\artifacts\models\indobert-best`

Deployment note:
- `model.onnx` is intentionally excluded from git and must be provided by the release pipeline.
- Samaryn CI downloads `model.onnx` plus `model.onnx.sha256` from a GitHub Release before building `samaryn-ml-service`.
- The remaining files in this directory stay tracked in git and are copied into the ML image together with the downloaded ONNX artifact.
