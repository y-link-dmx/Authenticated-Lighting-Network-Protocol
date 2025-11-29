"""Errors emitted by the Python SDK."""


class AlpineSdkError(Exception):
    """Base exception raised by the Alpine Python SDK."""


class DiscoveryTimeoutError(AlpineSdkError):
    """Indicates that discovery did not receive a reply in time."""
