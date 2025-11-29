"""High-level client orchestrating the discovery → streaming lifecycle."""

from dataclasses import dataclass
from typing import Optional

from alnp import (
    ControlEnvelope,
    FrameEnvelope,
    StreamProfile,
    encode_control,
    encode_frame,
)

from .errors import AlpineSdkError
from .transport import UdpTransport


@dataclass
class AlpineClientOptions:
    remote_host: str
    remote_port: int
    local_host: str = "0.0.0.0"
    local_port: int = 0


class AlpineClient:
    """SDK client that establishes streaming sessions over UDP."""

    def __init__(self, options: AlpineClientOptions) -> None:
        self._remote = (options.remote_host, options.remote_port)
        self._transport = UdpTransport(self._remote, options.local_host, options.local_port)
        self._config_id: Optional[str] = None
        self._streaming = False

    @classmethod
    def connect(cls, options: AlpineClientOptions) -> "AlpineClient":
        """Creates a new Alpine client bound to the requested sockets."""

        return cls(options)

    def start_stream(self, profile: Optional[StreamProfile] = None) -> str:
        """Starts the stream, records the config id, and returns it."""

        if self._streaming:
            raise AlpineSdkError("stream already started")
        profile = profile or StreamProfile.auto()
        compiled = profile.compile()
        self._config_id = compiled.config_id
        self._streaming = True
        return self._config_id

    def send_frame(self, frame: FrameEnvelope) -> None:
        """Sends a frame envelope over the transport."""

        self._ensure_streaming()
        payload = encode_frame(frame)
        self._transport.send(payload)

    def send_control(self, envelope: ControlEnvelope) -> None:
        """Sends a control envelope to the remote peer."""

        payload = encode_control(envelope)
        self._transport.send(payload)

    def close(self) -> None:
        """Closes the UDP socket."""

        self._transport.close()

    def _ensure_streaming(self) -> None:
        if not self._streaming or not self._config_id:
            raise AlpineSdkError("stream must be started before sending frames")
