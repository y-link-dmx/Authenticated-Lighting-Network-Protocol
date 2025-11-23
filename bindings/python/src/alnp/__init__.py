from dataclasses import dataclass, asdict
import json
from typing import Any, Dict


@dataclass
class ControlHeader:
    seq: int
    nonce: bytes
    timestamp_ms: int


@dataclass
class ControlEnvelope:
    header: ControlHeader
    payload: Dict[str, Any]
    signature: bytes

    def to_json(self) -> str:
        return json.dumps(
            {
                "header": asdict(self.header),
                "payload": self.payload,
                "signature": self.signature.decode("latin1"),
            }
        )


def SetMode(mode: str) -> Dict[str, Any]:
    return {"type": "SetMode", "body": {"mode": mode}}

