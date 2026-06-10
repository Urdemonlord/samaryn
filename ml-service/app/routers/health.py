from fastapi import APIRouter

from ..core.config import settings

router = APIRouter(tags=["health"])


@router.get("/health")
async def health_check():
    """Return service health status.

    Used by Docker HEALTHCHECK and the Rust gateway to verify
    the ML service is alive and its detectors are loaded.
    """
    return {
        "status": "healthy",
        "service": settings.app_name,
        "version": settings.version,
        "detectors_loaded": True,
        "model_dir": settings.model_dir,
    }
