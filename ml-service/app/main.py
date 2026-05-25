import logging
from contextlib import asynccontextmanager

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from .core.config import settings
from .routers import health, scan

# Configure logging
logging.basicConfig(
    level=getattr(logging, settings.log_level.upper()),
    format="%(asctime)s - %(name)s - %(levelname)s - %(message)s",
)
logger = logging.getLogger(__name__)


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Application lifespan — initialize and cleanup resources."""
    logger.info(f"Starting {settings.app_name} v{settings.version}")
    logger.info("Prompt injection detector: loaded")
    logger.info("PII detector: loaded")
    yield
    logger.info("Shutting down ML service")


app = FastAPI(
    title=settings.app_name,
    version=settings.version,
    description="Samaryn ML Security Service — prompt injection detection & PII masking",
    lifespan=lifespan,
)

# CORS (only gateway should call this, but permissive for dev)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include routers
app.include_router(health.router)
app.include_router(scan.router)

if __name__ == "__main__":
    import uvicorn

    uvicorn.run(
        "app.main:app",
        host=settings.host,
        port=settings.port,
        reload=True,
    )
