use async_trait::async_trait;
use uuid::Uuid;

use super::{
    new_nonce, ChallengeAuthenticator, HandshakeContext, HandshakeError, HandshakeMessage,
    HandshakeParticipant, HandshakeTransport,
};
use crate::crypto::KeyExchange;
use crate::messages::{
    CapabilitySet, ChallengeRequest, ChallengeResponse, DeviceIdentity, KeyExchangeProposal,
    NodeHello, ProtocolVersion, SessionEstablished, SignatureScheme,
};

/// Node-side handshake driver.
pub struct ServerHandshake<A, K>
where
    A: ChallengeAuthenticator + Send + Sync,
    K: KeyExchange + Send + Sync,
{
    pub identity: DeviceIdentity,
    pub capabilities: CapabilitySet,
    pub protocol_version: ProtocolVersion,
    pub authenticator: A,
    pub key_exchange: K,
    pub context: HandshakeContext,
}

#[async_trait]
impl<A, K> HandshakeParticipant for ServerHandshake<A, K>
where
    A: ChallengeAuthenticator + Send + Sync,
    K: KeyExchange + Send + Sync,
{
    async fn run<T: HandshakeTransport + Send>(
        &self,
        transport: &mut T,
    ) -> Result<SessionEstablished, HandshakeError> {
        let controller_hello = match transport.recv().await? {
            HandshakeMessage::ControllerHello(msg) => msg,
            other => {
                return Err(HandshakeError::Protocol(format!(
                    "expected ControllerHello, got {:?}",
                    other
                )))
            }
        };

        if let Some(expected) = &self.context.expected_controller {
            if &controller_hello.controller.cid.to_string() != expected {
                return Err(HandshakeError::Authentication(
                    "controller identity not authorized".into(),
                ));
            }
        }

        if let Some(required_fw) = &self.context.required_firmware_rev {
            if &controller_hello.controller.firmware_rev != required_fw {
                return Err(HandshakeError::Authentication(
                    "controller firmware revision rejected".into(),
                ));
            }
        }

        let expected_algorithm = format!("{:?}", self.context.key_algorithm);
        if controller_hello.key_exchange.algorithm != expected_algorithm {
            return Err(HandshakeError::Capability(format!(
                "controller key algorithm {} not supported",
                controller_hello.key_exchange.algorithm
            )));
        }

        let node_hello = NodeHello {
            node: self.identity.clone(),
            supported_version: self.protocol_version.clone(),
            capabilities: self.capabilities.clone(),
            key_exchange: KeyExchangeProposal {
                algorithm: format!("{:?}", self.context.key_algorithm),
                public_key: self.key_exchange.public_key(),
            },
            auth_required: true,
        };

        transport
            .send(HandshakeMessage::NodeHello(node_hello))
            .await?;

        let nonce = new_nonce().to_vec();
        let challenge = ChallengeRequest {
            nonce: nonce.clone(),
            controller_expected: controller_hello.controller.cid,
            signature_scheme: SignatureScheme::Ed25519,
        };

        transport
            .send(HandshakeMessage::ChallengeRequest(challenge.clone()))
            .await?;

        let response = match transport.recv().await? {
            HandshakeMessage::ChallengeResponse(resp) => resp,
            other => {
                return Err(HandshakeError::Protocol(format!(
                    "expected ChallengeResponse, got {:?}",
                    other
                )))
            }
        };

        validate_response(
            &challenge,
            &response,
            &self.authenticator,
            &controller_hello.controller.cid,
        )?;

        let shared = self
            .key_exchange
            .derive_shared(&controller_hello.key_exchange.public_key);

        let session = SessionEstablished {
            session_id: Uuid::new_v4(),
            agreed_version: controller_hello.requested_version,
            stream_key: Some(shared.shared_secret),
            expires_at_epoch_ms: None,
        };

        transport
            .send(HandshakeMessage::SessionEstablished(session.clone()))
            .await?;

        Ok(session)
    }
}

fn validate_response<A: ChallengeAuthenticator>(
    challenge: &ChallengeRequest,
    response: &ChallengeResponse,
    authenticator: &A,
    controller_cid: &Uuid,
) -> Result<(), HandshakeError> {
    if response.nonce != challenge.nonce {
        return Err(HandshakeError::Protocol(
            "nonce in response did not match challenge".into(),
        ));
    }

    let verified = authenticator.verify_challenge(&response.nonce, &response.signature);
    if !verified {
        return Err(HandshakeError::Authentication(
            "challenge signature validation failed".into(),
        ));
    }

    if response
        .key_confirmation
        .as_ref()
        .map(|k| k.is_empty())
        .unwrap_or(true)
    {
        return Err(HandshakeError::Authentication(format!(
            "controller {} did not confirm key material",
            controller_cid
        )));
    }

    Ok(())
}
