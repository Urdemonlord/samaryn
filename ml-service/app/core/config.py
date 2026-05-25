from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    """Application settings loaded from environment variables.

    All settings can be overridden via environment variables prefixed
    with ``ML_SERVICE_`` (e.g. ``ML_SERVICE_PORT=9000``).
    """

    app_name: str = "samaryn-ml-service"
    version: str = "0.1.0"
    host: str = "0.0.0.0"
    port: int = 8000
    log_level: str = "info"

    model_config = {"env_prefix": "ML_SERVICE_"}


settings = Settings()
