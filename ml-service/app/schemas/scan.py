from pydantic import BaseModel, Field
from typing import Optional
from enum import Enum


class ScanType(str, Enum):
    """Supported scan types."""

    INJECTION = "injection"
    PII = "pii"


class ScanRequest(BaseModel):
    """Incoming scan request from the Rust gateway."""

    text: str
    scan_types: list[ScanType] = Field(default=[ScanType.INJECTION, ScanType.PII])


class PiiEntity(BaseModel):
    """A single detected PII entity with position information."""

    entity_type: str
    value: str
    start: int
    end: int


class PiiResult(BaseModel):
    """Result of PII detection scan."""

    detected: bool = False
    entities: list[PiiEntity] = []
    masked_text: str = ""


class ThreatDetail(BaseModel):
    """Details about a single detected injection threat."""

    pattern_name: str
    matched_text: str
    severity: str = "medium"


class InjectionResult(BaseModel):
    """Result of prompt injection scan."""

    detected: bool = False
    threats: list[ThreatDetail] = []
    severity: Optional[str] = None


class ScanResponse(BaseModel):
    """Combined scan response returned to the gateway."""

    is_safe: bool = True
    injection: Optional[InjectionResult] = None
    pii: Optional[PiiResult] = None
