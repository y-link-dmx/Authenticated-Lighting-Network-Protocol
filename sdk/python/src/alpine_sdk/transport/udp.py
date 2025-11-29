"""UDP transport helper used by the Alpine SDK clients."""

import socket
from typing import Optional, Tuple


class UdpTransport:
    """Lightweight UDP socket wrapper."""

    def __init__(self, remote: Tuple[str, int], local_host: str, local_port: int) -> None:
        self._remote = remote
        self._socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self._socket.bind((local_host, local_port))

    def send(self, payload: bytes, destination: Optional[Tuple[str, int]] = None) -> None:
        """Sends raw bytes to the provided destination or the configured peer."""

        target = destination or self._remote
        self._socket.sendto(payload, target)

    def close(self) -> None:
        """Closes the underlying socket."""

        self._socket.close()
