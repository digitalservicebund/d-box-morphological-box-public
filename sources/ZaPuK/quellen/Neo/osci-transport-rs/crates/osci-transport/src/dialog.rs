// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::error::{Error, Result};

/// `soap:Header/osci:ControlBlock` (spec 3.3, 4.5).
///
/// ConversationId identifies the dialog, SequenceNumber is the sender's own
/// message counter, Challenge is a fresh nonce, Response echoes the peer's
/// previous Challenge.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ControlBlock {
    pub id: Option<String>,
    pub conversation_id: Option<String>,
    pub challenge: Option<String>,
    pub response: Option<String>,
    pub sequence_number: Option<u32>,
}

/// Supplier-side state of one dialog.
///
/// The Supplier assigns the ConversationId, validates the
/// `{ConversationId, SequenceNumber, Response}` triple on every incoming request,
/// and (in an explicit dialog) sends a fresh Challenge that the client must echo
/// next.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DialogState {
    pub conversation_id: String,
    pub explicit: bool,
    // highest SequenceNumber seen from the client, None before the first request
    pub peer_sequence: Option<u32>,
    // our own response counter
    pub own_sequence: u32,
    // Challenge we last sent; the client's next request must echo it as Response
    pub expected_response: Option<String>,
    // Challenge the client last sent; we echo it as Response
    pub peer_challenge: Option<String>,
}

impl DialogState {
    pub fn new(conversation_id: impl Into<String>, explicit: bool) -> Self {
        DialogState {
            conversation_id: conversation_id.into(),
            explicit,
            peer_sequence: None,
            own_sequence: 0,
            expected_response: None,
            peer_challenge: None,
        }
    }

    /// Validate an incoming request's ControlBlock and advance the dialog.
    pub fn accept_request(&mut self, cb: &ControlBlock) -> Result<()> {
        if cb.conversation_id.as_deref() != Some(self.conversation_id.as_str()) {
            return Err(Error::msg("ControlBlock: wrong ConversationId"));
        }
        let seq = cb
            .sequence_number
            .ok_or_else(|| Error::msg("ControlBlock: no SequenceNumber"))?;
        let expected = self
            .peer_sequence
            .map_or(Some(0), |n| n.checked_add(1))
            .ok_or_else(|| Error::msg("ControlBlock: SequenceNumber exhausted"))?;
        if seq != expected {
            return Err(Error::msg(format!(
                "ControlBlock: SequenceNumber {seq}, expected {expected}"
            )));
        }
        if self.explicit
            && self.expected_response.is_some()
            && cb.response.as_deref() != self.expected_response.as_deref()
        {
            return Err(Error::msg("ControlBlock: bad Response"));
        }
        self.peer_sequence = Some(seq);
        self.peer_challenge = cb.challenge.clone();
        Ok(())
    }

    /// Build the ControlBlock for our response. `challenge` is the fresh nonce we
    /// send (used only in an explicit dialog); the caller generates it.
    pub fn response_control_block(&mut self, challenge: impl Into<String>) -> ControlBlock {
        let challenge = challenge.into();
        let cb = ControlBlock {
            id: Some("controlblock".into()),
            conversation_id: Some(self.conversation_id.clone()),
            challenge: self.explicit.then(|| challenge.clone()),
            response: self.peer_challenge.clone(),
            sequence_number: Some(self.own_sequence),
        };
        self.own_sequence = self.own_sequence.saturating_add(1);
        if self.explicit {
            self.expected_response = Some(challenge);
        }
        cb
    }
}

/// Client-side state of one dialog.
///
/// Mirror of [`DialogState`]: the client picks its own SequenceNumber and
/// Challenge, expects the Supplier to echo the Challenge as Response, and echoes
/// the Supplier's Challenge in its next request.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ClientDialog {
    // assigned by the Supplier in the initDialog response; None outside an
    // explicit dialog and before it is opened
    pub conversation_id: Option<String>,
    pub explicit: bool,
    pub own_sequence: u32,
    pub expected_response: Option<String>,
    pub peer_challenge: Option<String>,
}

impl ClientDialog {
    pub fn implicit() -> Self {
        ClientDialog::default()
    }

    pub fn explicit() -> Self {
        ClientDialog {
            explicit: true,
            ..ClientDialog::default()
        }
    }

    /// Build the ControlBlock for our next request. `challenge` is a fresh nonce
    /// the caller generates.
    pub fn request_control_block(&mut self, challenge: impl Into<String>) -> ControlBlock {
        let challenge = challenge.into();
        let cb = ControlBlock {
            id: Some("controlblock".into()),
            conversation_id: self.conversation_id.clone(),
            challenge: Some(challenge.clone()),
            response: self.peer_challenge.clone(),
            sequence_number: Some(self.own_sequence),
        };
        self.own_sequence = self.own_sequence.saturating_add(1);
        self.expected_response = Some(challenge);
        cb
    }

    /// Build the ControlBlock for an `InitDialog` request: just a fresh
    /// Challenge, nothing else. `InitDialog` is the message that establishes
    /// dialog state, so it can't carry a ConversationId, SequenceNumber or
    /// Response yet - the official Java osci-bibliothek's `InitDialog` constructor
    /// explicitly clears all three for the same reason.
    pub fn init_control_block(&mut self, challenge: impl Into<String>) -> ControlBlock {
        let challenge = challenge.into();
        let cb = ControlBlock {
            id: Some("controlblock".into()),
            challenge: Some(challenge.clone()),
            ..Default::default()
        };
        self.expected_response = Some(challenge);
        cb
    }

    /// Validate the Supplier's response ControlBlock and advance the dialog.
    pub fn accept_response(&mut self, cb: &ControlBlock) -> Result<()> {
        if self.expected_response.is_some()
            && cb.response.as_deref() != self.expected_response.as_deref()
        {
            return Err(Error::msg("response ControlBlock: bad Response"));
        }
        if self.conversation_id.is_none() {
            self.conversation_id = cb.conversation_id.clone();
        } else if cb.conversation_id.is_some()
            && cb.conversation_id.as_deref() != self.conversation_id.as_deref()
        {
            return Err(Error::msg("response ControlBlock: wrong ConversationId"));
        }
        self.peer_challenge = cb.challenge.clone();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cb(cid: &str, seq: u32, challenge: &str, response: Option<&str>) -> ControlBlock {
        ControlBlock {
            id: None,
            conversation_id: Some(cid.into()),
            challenge: Some(challenge.into()),
            response: response.map(Into::into),
            sequence_number: Some(seq),
        }
    }

    #[test]
    fn explicit_dialog_challenge_response() {
        let mut d = DialogState::new("c1", true);

        // initDialog request: seq 0, no response expected yet
        d.accept_request(&cb("c1", 0, "chA", None)).unwrap();
        let r0 = d.response_control_block("srvB");
        assert_eq!(r0.response.as_deref(), Some("chA"));
        assert_eq!(r0.challenge.as_deref(), Some("srvB"));
        assert_eq!(r0.sequence_number, Some(0));

        // next request must echo "srvB" and use seq 1
        assert!(
            d.accept_request(&cb("c1", 1, "chC", Some("wrong")))
                .is_err()
        );
        d.accept_request(&cb("c1", 1, "chC", Some("srvB"))).unwrap();
        let r1 = d.response_control_block("srvD");
        assert_eq!(r1.response.as_deref(), Some("chC"));
        assert_eq!(r1.sequence_number, Some(1));
    }

    #[test]
    fn rejects_wrong_conversation_and_sequence() {
        let mut d = DialogState::new("c1", false);
        assert!(d.accept_request(&cb("other", 0, "x", None)).is_err());
        assert!(d.accept_request(&cb("c1", 5, "x", None)).is_err());
        assert!(d.accept_request(&cb("c1", 0, "x", None)).is_ok());
    }

    #[test]
    fn client_dialog_challenge_response() {
        let mut c = ClientDialog::explicit();

        let r0 = c.request_control_block("cliA");
        assert_eq!(r0.challenge.as_deref(), Some("cliA"));
        assert_eq!(r0.sequence_number, Some(0));

        // supplier response: echoes cliA, assigns ConversationId, sends its Challenge
        let resp0 = ControlBlock {
            conversation_id: Some("cv1".into()),
            response: Some("cliA".into()),
            challenge: Some("srvB".into()),
            sequence_number: Some(0),
            ..Default::default()
        };
        c.accept_response(&resp0).unwrap();
        assert_eq!(c.conversation_id.as_deref(), Some("cv1"));

        // next request echoes srvB, uses ConversationId, seq 1
        let r1 = c.request_control_block("cliC");
        assert_eq!(r1.response.as_deref(), Some("srvB"));
        assert_eq!(r1.conversation_id.as_deref(), Some("cv1"));
        assert_eq!(r1.sequence_number, Some(1));

        // a response with the wrong echo is rejected
        let bad = ControlBlock {
            response: Some("nope".into()),
            ..Default::default()
        };
        assert!(c.accept_response(&bad).is_err());
    }

    #[test]
    fn sequence_number_at_the_limit_is_an_error_not_a_panic() {
        let mut d = DialogState::new("c1", false);
        d.peer_sequence = Some(u32::MAX);
        assert!(d.accept_request(&cb("c1", 0, "x", None)).is_err());
    }
}
