// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

//! `osci:Feedback/osci:Entry/osci:Code` values (spec chapter 5). Four digits:
//! the first is the kind (0 success/info, 3 warning, 9 error), the second is the
//! processing step (1 receive .. 8 execute, 9 chunked transfer).

/// `true` for a `0xxx` success or info code.
pub fn is_success(code: &str) -> bool {
    code.starts_with('0')
}

/// `true` for a `3xxx` warning code.
pub fn is_warning(code: &str) -> bool {
    code.starts_with('3')
}

/// `true` for a `9xxx` error code.
pub fn is_error(code: &str) -> bool {
    code.starts_with('9')
}

/// Named constants for the feedback codes this crate uses.
pub mod code {
    // success
    pub const OK_DIALOG_CLOSED: &str = "0800";
    pub const OK_DIALOG_OPEN: &str = "0801";

    // warnings
    pub const MORE_DELIVERIES_WAITING: &str = "3800";
    pub const MORE_PROCESS_CARDS: &str = "3801";
    pub const RECIPIENT_RESPONSE_UNENCRYPTED: &str = "3803";
    pub const BIG_MESSAGE_WARNING: &str = "3950";
    pub const BIG_MESSAGE_STILL_PROCESSING: &str = "3952";

    // dialog / control block
    pub const BAD_CONTROL_BLOCK: &str = "9400";

    // client cipher certificate (step 5)
    pub const CLIENT_CIPHER_CERT_MISSING: &str = "9500";
    pub const CLIENT_CIPHER_CERT_SIG_BAD: &str = "9501";
    pub const CLIENT_CIPHER_CERT_REVOKED: &str = "9502";

    // order signature (step 6)
    pub const ORDER_NOT_SIGNED: &str = "9600";
    pub const ORDER_SIGNATURE_BAD: &str = "9601";
    pub const ORDER_SIGNATURE_INCOMPLETE: &str = "9602";

    // order execution (step 8)
    pub const NO_MESSAGE_ID: &str = "9800";
    pub const ILLEGAL_MESSAGE_ID: &str = "9801";
    pub const NOT_IN_EXPLICIT_DIALOG: &str = "9802";
    pub const NO_DELIVERY_MATCHES: &str = "9803";
    pub const NO_PROCESS_CARD_MATCHES: &str = "9804";
    pub const SUPPLIER_INTERNAL_ERROR: &str = "9811";

    // chunked transfer (step 9)
    pub const CHUNK_ALREADY_RECEIVED: &str = "9900";
    pub const CHUNK_NUMBER_OUT_OF_RANGE: &str = "9901";
    pub const CHUNK_TOO_LARGE: &str = "9902";
    pub const CHUNK_TOO_SMALL: &str = "9903";
    pub const TOTAL_MESSAGE_TOO_LARGE: &str = "9904";

    // soap:Fault / message level
    pub const MSG_NOT_VALID_OSCI: &str = "9100";
    pub const MSG_NO_PRIVATE_KEY: &str = "9200";
    pub const MSG_CIPHER_CERT_REVOKED: &str = "9201";
    pub const MSG_DECRYPT_FAILED: &str = "9202";
    pub const MSG_NOT_VALID_ORDER: &str = "9300";
    pub const MSG_BAD_CONVERSATION: &str = "9400";
    pub const MSG_SUPPLIER_INTERNAL_ERROR: &str = "9000";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_classification() {
        assert!(is_success(code::OK_DIALOG_OPEN));
        assert!(is_warning(code::MORE_DELIVERIES_WAITING));
        assert!(is_error(code::ORDER_SIGNATURE_BAD));
    }
}
