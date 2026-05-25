import re
import unicodedata
import base64
import logging

logger = logging.getLogger(__name__)


class PromptInjectionDetector:
    """Rule-based prompt injection detection engine.

    Detects common prompt injection patterns including:
    - Instruction override attempts
    - System prompt reveal attempts
    - Persona/role override attempts
    - Hidden unicode attacks
    - Base64/hex encoded attacks
    - Malicious tool/command requests
    """

    def __init__(self):
        self.patterns = {
            "ignore_instructions": {
                "pattern": re.compile(
                    r"(?:ignore|forget|disregard|override|bypass|skip)\s+"
                    r"(?:all\s+)?(?:previous|prior|above|earlier|your|the|system)\s+"
                    r"(?:instructions?|rules?|directives?|guidelines?|prompts?|constraints?)",
                    re.IGNORECASE,
                ),
                "severity": "critical",
                "description": "Attempt to override system instructions",
            },
            "reveal_prompt": {
                "pattern": re.compile(
                    r"(?:reveal|show|tell\s+me|display|print|output|repeat|echo)\s+"
                    r"(?:your\s+)?(?:the\s+)?(?:system\s+)?(?:original\s+)?"
                    r"(?:prompt|instructions?|configuration|rules|directive|system\s+message)",
                    re.IGNORECASE,
                ),
                "severity": "high",
                "description": "Attempt to reveal system prompt",
            },
            "override_persona": {
                "pattern": re.compile(
                    r"(?:act\s+as|pretend\s+(?:to\s+be|you(?:'re|\s+are))|behave\s+(?:as|like)|"
                    r"you\s+are\s+now|switch\s+to|enter)\s+"
                    r"(?:a\s+)?(?:jailbr[oe]ken|unrestricted|unfiltered|evil|DAN|"
                    r"hacker|malicious|unlimited)",
                    re.IGNORECASE,
                ),
                "severity": "critical",
                "description": "Attempt to override AI persona",
            },
            "role_override": {
                "pattern": re.compile(
                    r"(?:you\s+are\s+no\s+longer|stop\s+being|don'?t\s+be|"
                    r"new\s+(?:role|persona|identity|mode)\s*[:\-]|"
                    r"from\s+now\s+on\s+you\s+(?:are|will|must|should))",
                    re.IGNORECASE,
                ),
                "severity": "high",
                "description": "Attempt to change AI role",
            },
            "system_prompt_injection": {
                "pattern": re.compile(
                    r"(?:\[SYSTEM\]|\[INST\]|<<SYS>>|<\|system\|>|"
                    r"###\s*(?:System|Instruction)|"
                    r"SYSTEM\s*:|"
                    r"<system>|</system>)",
                    re.IGNORECASE,
                ),
                "severity": "high",
                "description": "Embedded system prompt markers",
            },
            "malicious_tool": {
                "pattern": re.compile(
                    r"(?:execute|run|perform|invoke)\s+"
                    r"(?:a\s+)?(?:shell|bash|cmd|command|terminal|system|code|script|sql|query)",
                    re.IGNORECASE,
                ),
                "severity": "medium",
                "description": "Attempt to execute system commands",
            },
            "delimiter_attack": {
                "pattern": re.compile(
                    r"(?:---+|===+|~~~+|\*\*\*+|###\s*END)\s*"
                    r"(?:ignore|new\s+instructions?|real\s+prompt|actual\s+task)",
                    re.IGNORECASE,
                ),
                "severity": "high",
                "description": "Delimiter-based injection attempt",
            },
        }

        # Zero-width and invisible unicode characters
        self.invisible_chars = re.compile(
            r"[\u200b\u200c\u200d\u200e\u200f\u2060\u2061\u2062\u2063\u2064"
            r"\ufeff\u00ad\u034f\u061c\u17b4\u17b5\u180e\u2000-\u200a"
            r"\u2028\u2029\u202a-\u202e\u2066-\u2069\ufff9-\ufffb]"
        )

    def _preprocess(self, text: str) -> str:
        """Normalize text for pattern matching."""
        # Unicode normalize
        normalized = unicodedata.normalize("NFKC", text)
        # Collapse whitespace
        normalized = re.sub(r"\s+", " ", normalized).strip()
        return normalized

    def _check_hidden_unicode(self, text: str) -> list[dict]:
        """Detect suspicious invisible unicode characters."""
        matches = list(self.invisible_chars.finditer(text))
        if len(matches) > 3:  # Threshold: more than 3 invisible chars is suspicious
            return [
                {
                    "pattern_name": "hidden_unicode",
                    "matched_text": f"Found {len(matches)} invisible unicode characters",
                    "severity": "medium",
                }
            ]
        return []

    def _check_encoded_attack(self, text: str) -> list[dict]:
        """Detect base64 or hex encoded injection attempts."""
        threats = []
        # Look for base64-encoded strings
        b64_pattern = re.compile(r"[A-Za-z0-9+/]{20,}={0,2}")
        for match in b64_pattern.finditer(text):
            try:
                decoded = base64.b64decode(match.group()).decode(
                    "utf-8", errors="ignore"
                ).lower()
                # Check if decoded content contains injection patterns
                suspicious_keywords = [
                    "ignore",
                    "system prompt",
                    "instructions",
                    "jailbreak",
                    "override",
                ]
                if any(kw in decoded for kw in suspicious_keywords):
                    threats.append(
                        {
                            "pattern_name": "encoded_attack",
                            "matched_text": (
                                f"Base64 encoded suspicious content: "
                                f"{match.group()[:50]}..."
                            ),
                            "severity": "high",
                        }
                    )
            except Exception:
                pass
        return threats

    def scan(self, text: str) -> dict:
        """Scan text for prompt injection attempts.

        Returns:
            dict with keys: detected (bool), threats (list), severity (str|None)
        """
        preprocessed = self._preprocess(text)
        threats: list[dict] = []
        max_severity: str | None = None
        severity_order = {"critical": 3, "high": 2, "medium": 1, "low": 0}

        # Check regex patterns
        for name, config in self.patterns.items():
            match = config["pattern"].search(preprocessed)
            if match:
                threats.append(
                    {
                        "pattern_name": name,
                        "matched_text": match.group()[:100],
                        "severity": config["severity"],
                    }
                )
                sev = config["severity"]
                if max_severity is None or severity_order.get(
                    sev, 0
                ) > severity_order.get(max_severity, 0):
                    max_severity = sev

        # Check hidden unicode (on original text, not preprocessed)
        unicode_threats = self._check_hidden_unicode(text)
        threats.extend(unicode_threats)

        # Check encoded attacks
        encoded_threats = self._check_encoded_attack(text)
        threats.extend(encoded_threats)

        if threats and max_severity is None:
            max_severity = max(
                (t.get("severity", "low") for t in threats),
                key=lambda s: severity_order.get(s, 0),
            )

        return {
            "detected": len(threats) > 0,
            "threats": threats,
            "severity": max_severity,
        }


# Singleton
detector = PromptInjectionDetector()
