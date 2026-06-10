import logging

from fastapi import APIRouter

from ..schemas.scan import (
    ClassificationResult,
    InjectionResult,
    PiiEntity,
    PiiResult,
    ScanRequest,
    ScanResponse,
    ScanType,
)
from ..services.injection import detector as injection_detector
from ..services.pii import detector as pii_detector

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/api/v1", tags=["scan"])


@router.post("/scan", response_model=ScanResponse)
async def scan_text(request: ScanRequest) -> ScanResponse:
    """Scan text for security threats and PII.

    Accepts a text payload and optional list of scan types. Returns
    a combined result indicating whether the text is safe, along with
    detailed injection and PII findings.
    """
    logger.info(
        f"Scanning text ({len(request.text)} chars), types: {request.scan_types}"
    )

    injection_result = None
    pii_result = None
    classification_result = None
    is_safe = True

    if ScanType.INJECTION in request.scan_types:
        result = injection_detector.scan(request.text)
        classification_payload = result.get("classification")
        if classification_payload is not None:
            classification_result = ClassificationResult(**classification_payload)
        injection_result = InjectionResult(
            detected=result["detected"],
            threats=result["threats"],
            severity=result.get("severity"),
        )
        if classification_result is not None:
            is_safe = classification_result.action == "allow"
        elif result["detected"]:
            is_safe = False

    if ScanType.PII in request.scan_types:
        entities = pii_detector.detect(request.text)
        masked_text, _ = pii_detector.anonymize(request.text, entities)
        pii_result = PiiResult(
            detected=len(entities) > 0,
            entities=[
                PiiEntity(
                    entity_type=e.entity_type,
                    value=e.value,
                    start=e.start,
                    end=e.end,
                )
                for e in entities
            ],
            masked_text=masked_text,
        )

    return ScanResponse(
        is_safe=is_safe,
        injection=injection_result,
        pii=pii_result,
        classification=classification_result,
    )
