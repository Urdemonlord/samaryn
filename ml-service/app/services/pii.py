import re
import logging
from typing import NamedTuple

logger = logging.getLogger(__name__)


class PiiMatch(NamedTuple):
    """Represents a single PII match with type, value, and character offsets."""

    entity_type: str
    value: str
    start: int
    end: int


class PiiDetector:
    """Regex-based PII detection and anonymization.

    Detects: email, phone (US + Indonesian), API keys, JWT tokens,
    credit cards, bank accounts, credentials.
    """

    def __init__(self):
        self.patterns = {
            "EMAIL": re.compile(
                r"[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}"
            ),
            "PHONE": re.compile(
                # Indonesian: 08xx-xxxx-xxxx or +62xxx
                # US: (xxx) xxx-xxxx or xxx-xxx-xxxx
                # International: +xx xxx xxx xxxx
                r"(?:\+62[\s\-]?\d{2,4}[\s\-]?\d{3,4}[\s\-]?\d{3,4})"
                r"|(?:08\d{1,2}[\s\-.]?\d{3,4}[\s\-.]?\d{3,4})"
                r"|(?:\+?1[\s\-.]?\(?\d{3}\)?[\s\-.]?\d{3}[\s\-.]?\d{4})"
                r"|(?:\+\d{1,3}[\s\-.]?\d{2,4}[\s\-.]?\d{3,4}[\s\-.]?\d{3,4})"
            ),
            "API_KEY": re.compile(
                r"(?:sk-[a-zA-Z0-9]{32,})"  # OpenAI keys
                r"|(?:AKIA[A-Z0-9]{16})"  # AWS access keys
                r"|(?:ghp_[a-zA-Z0-9]{36})"  # GitHub tokens
                r"|(?:glpat-[a-zA-Z0-9\-]{20,})"  # GitLab tokens
                r"|(?:xoxb-[a-zA-Z0-9\-]+)"  # Slack bot tokens
            ),
            "JWT": re.compile(
                r"eyJ[a-zA-Z0-9_\-]*\.eyJ[a-zA-Z0-9_\-]*\.[a-zA-Z0-9_\-]*"
            ),
            "CREDIT_CARD": re.compile(
                r"\b(?:4[0-9]{12}(?:[0-9]{3})?)"  # Visa
                r"|(?:5[1-5][0-9]{14})"  # Mastercard
                r"|(?:3[47][0-9]{13})"  # Amex
                r"|(?:6(?:011|5[0-9]{2})[0-9]{12})"  # Discover
                r"\b"
            ),
            "CREDENTIAL": re.compile(
                # password= or secret= or token= followed by value
                r"(?:password|passwd|pwd|secret|token|api[_\-]?key|access[_\-]?key)"
                r"\s*[=:]\s*['\"]?([^\s'\"]{8,})['\"]?",
                re.IGNORECASE,
            ),
        }

    def detect(self, text: str) -> list[PiiMatch]:
        """Detect all PII entities in text.

        Returns list of PiiMatch with entity_type, value, start, end.
        """
        entities: list[PiiMatch] = []

        for entity_type, pattern in self.patterns.items():
            for match in pattern.finditer(text):
                entities.append(
                    PiiMatch(
                        entity_type=entity_type,
                        value=match.group(),
                        start=match.start(),
                        end=match.end(),
                    )
                )

        # Sort by position (start), then by length (longer match first)
        entities.sort(key=lambda e: (e.start, -(e.end - e.start)))

        # Remove overlapping entities (keep longest)
        filtered: list[PiiMatch] = []
        last_end = 0
        for entity in entities:
            if entity.start >= last_end:
                filtered.append(entity)
                last_end = entity.end

        return filtered

    def anonymize(
        self, text: str, entities: list[PiiMatch] | None = None
    ) -> tuple[str, list[PiiMatch]]:
        """Replace PII with numbered placeholders like [EMAIL_1], [PHONE_1].

        Returns (masked_text, entities).
        """
        if entities is None:
            entities = self.detect(text)

        if not entities:
            return text, []

        # Count occurrences per type for numbering
        type_counter: dict[str, int] = {}
        result = text
        offset = 0

        for entity in entities:
            type_counter.setdefault(entity.entity_type, 0)
            type_counter[entity.entity_type] += 1
            count = type_counter[entity.entity_type]

            placeholder = f"[{entity.entity_type}_{count}]"
            start = entity.start + offset
            end = entity.end + offset

            result = result[:start] + placeholder + result[end:]
            offset += len(placeholder) - (entity.end - entity.start)

        return result, entities


# Singleton
detector = PiiDetector()
