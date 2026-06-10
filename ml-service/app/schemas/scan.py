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
    entities: list[PiiEntity] = Field(default_factory=list)
    masked_text: str = ""


class InjectionResult(BaseModel):
    """Result of prompt injection scan."""

    detected: bool = False
    threats: list[str] = Field(default_factory=list)
    severity: Optional[str] = None


class ClassificationResult(BaseModel):
    """Three-way classifier output used by the Samaryn gateway."""

    label: str
    confidence: float
    source: str
    action: str
    normalized_text: Optional[str] = None
    matched_category: Optional[str] = None
    matched_pattern: Optional[str] = None


class ScanResponse(BaseModel):
    """Combined scan response returned to the gateway."""

    is_safe: bool = True
    injection: Optional[InjectionResult] = None
    pii: Optional[PiiResult] = None
    classification: Optional[ClassificationResult] = None
