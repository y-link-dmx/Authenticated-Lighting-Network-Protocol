"""Discovery helper that exchanges CBOR messages with devices."""

import os
import socket
from dataclasses import dataclass
from typing import Any, Dict, List, Optional

import cbor2

from alnp import build_discovery_request

from .errors import AlpineSdkError, DiscoveryTimeoutError


@dataclass
class DiscoveryClientOptions:
    remote_host: str
    remote_port: int
    local_host: str = "0.0.0.0"
    local_port: int = 0
    timeout: float = 3.0


class DiscoveryClient:
    """Stateless discovery helper for the Alpine SDK."""

    def __init__(self, options: DiscoveryClientOptions) -> None:
        self._remote = (options.remote_host, options.remote_port)
        self._socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self._socket.settimeout(options.timeout)
        self._socket.bind((options.local_host, options.local_port))

    def discover(
        self, requested: List[str], nonce: Optional[bytes] = None
    ) -> Dict[str, Any]:
        """Builds a discovery request, broadcasts it, and decodes the reply."""

        payload = build_discovery_request(
            requested, nonce or os.urandom(32)
        ).to_map()
        self._send(cbor2.dumps(payload))
        try:
            data, _ = self._socket.recvfrom(2048)
        except socket.timeout:
            raise DiscoveryTimeoutError()
        return cbor2.loads(data)

    def close(self) -> None:
        """Closes the discovery socket."""

        self._socket.close()

    def _send(self, data: bytes) -> None:
        """Sends the CBOR bytes to the remote peer."""

        try:
            self._socket.sendto(data, self._remote)
        except OSError as err:
            raise AlpineSdkError(str(err)) from err
