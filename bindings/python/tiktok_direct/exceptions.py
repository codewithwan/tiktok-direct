class TikTokDirectError(Exception):
    """Base exception for tiktok-direct."""
    pass

class InvalidURLError(TikTokDirectError):
    """Raised when URL format is invalid."""
    pass

class ChallengeError(TikTokDirectError):
    """Raised when challenge solving fails."""
    pass

class DownloadError(TikTokDirectError):
    """Raised when media download fails."""
    pass
