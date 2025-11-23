use async_trait::async_trait;
use uuid::Uuid;

use super::{
    HandshakeContext, HandshakeError, HandshakeMessage, HandshakeParticipant, HandshakeTransport,
};
use crate::crypto::KeyExchange;
use crate::messages::{
    CapabilitySet, ChallengeRequest, ChallengeResponse, ControllerHello, DeviceIdentity,
    ProtocolVersion, SessionEstablished,
};

/// Controller-side handshake driver.
pub struct ClientHandshake<A, K>
where
    A: super::ChallengeAuthenticator + Send + Sync,
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
impl<A, K> HandshakeParticipant for ClientHandshake<A, K>
where
    A: super::ChallengeAuthenticator + Send + Sync,
    K: KeyExchange + Send + Sync,
{
    async fn run<T: HandshakeTransport + Send>(
        &self,
        transport: &mut T,
    ) -> Result<SessionEstablished, HandshakeError> {
        let controller_hello = ControllerHello {
            controller: self.identity.clone(),
            requested_version: self.protocol_version.clone(),
            capabilities: self.capabilities.clone(),
            key_exchange: crate::messages::KeyExchangeProposal {
                algorithm: format!("{:?}", self.context.key_algorithm),
                public_key: self.key_exchange.public_key(),
            },
        };

        transport
            .send(HandshakeMessage::ControllerHello(controller_hello))
            .await?;

        let node_hello = match transport.recv().await? {
            HandshakeMessage::NodeHello(hello) => hello,
            other => {
                return Err(HandshakeError::Protocol(format!(
                    "expected NodeHello, got {:?}",
                    other
                )))
            }
        };

        let expected_algorithm = format!("{:?}", self.context.key_algorithm);
        if node_hello.key_exchange.algorithm != expected_algorithm {
            return Err(HandshakeError::Capability(format!(
                "node key algorithm {} not supported",
                node_hello.key_exchange.algorithm
            )));
        }

        if node_hello.supported_version.major != self.protocol_version.major {
            return Err(HandshakeError::Protocol(format!(
                "version mismatch: node supports {}.{}.{}",
                node_hello.supported_version.major,
                node_hello.supported_version.minor,
                node_hello.supported_version.patch
            )));
        }

        let challenge = match transport.recv().await? {
            HandshakeMessage::ChallengeRequest(ch) => ch,
            other => {
                return Err(HandshakeError::Protocol(format!(
                    "expected ChallengeRequest, got {:?}",
                    other
                )))
            }
        };

        validate_challenge(&challenge, &self.identity.cid, &self.context)?;

        let signature = self.authenticator.sign_challenge(&challenge.nonce);
        let shared = self
            .key_exchange
            .derive_shared(&node_hello.key_exchange.public_key);

        let response = ChallengeResponse {
            nonce: challenge.nonce.clone(),
            signature,
            key_confirmation: Some(shared.shared_secret.clone()),
        };

        transport
            .send(HandshakeMessage::ChallengeResponse(response))
            .await?;

        let established = match transport.recv().await? {
            HandshakeMessage::SessionEstablished(session) => session,
            other => {
                return Err(HandshakeError::Protocol(format!(
                    "expected SessionEstablished, got {:?}",
                    other
                )))
            }
        };

        Ok(established)
    }
}

fn validate_challenge(
    challenge: &ChallengeRequest,
    controller_cid: &Uuid,
    context: &HandshakeContext,
) -> Result<(), HandshakeError> {
    if challenge.controller_expected != *controller_cid {
        return Err(HandshakeError::Authentication(
            "challenge addressed to different controller".into(),
        ));
    }

    if let Some(expected) = &context.expected_controller {
        if &challenge.controller_expected.to_string() != expected {
            return Err(HandshakeError::Authentication(
                "controller identity rejected".into(),
            ));
        }
    }

    Ok(())
}
