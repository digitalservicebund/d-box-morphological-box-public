// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::error::{Error, Result};
use crate::xml::{QName, ns};

/// An OSCI-Transport request or response type.
///
/// OSCI 1.2 fixes this set. Profiles such as XJustiz constrain existing messages
/// and define content, they do not add SOAP message types, so this stays a
/// closed enum.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive]
pub enum MessageType {
    InitDialog,
    ResponseToInitDialog,
    ExitDialog,
    ResponseToExitDialog,
    GetMessageId,
    ResponseToGetMessageId,
    StoreDelivery,
    ResponseToStoreDelivery,
    FetchDelivery,
    ResponseToFetchDelivery,
    FetchProcessCard,
    ResponseToFetchProcessCard,
    ForwardDelivery,
    ResponseToForwardDelivery,
    AcceptDelivery,
    ResponseToAcceptDelivery,
    MediateDelivery,
    ResponseToMediateDelivery,
    ProcessDelivery,
    ResponseToProcessDelivery,
    PartialStoreDelivery,
    ResponseToPartialStoreDelivery,
    PartialFetchDelivery,
    ResponseToPartialFetchDelivery,
}

impl MessageType {
    /// Every variant, in a stable order.
    pub const ALL: &'static [MessageType] = &[
        MessageType::InitDialog,
        MessageType::ResponseToInitDialog,
        MessageType::ExitDialog,
        MessageType::ResponseToExitDialog,
        MessageType::GetMessageId,
        MessageType::ResponseToGetMessageId,
        MessageType::StoreDelivery,
        MessageType::ResponseToStoreDelivery,
        MessageType::FetchDelivery,
        MessageType::ResponseToFetchDelivery,
        MessageType::FetchProcessCard,
        MessageType::ResponseToFetchProcessCard,
        MessageType::ForwardDelivery,
        MessageType::ResponseToForwardDelivery,
        MessageType::AcceptDelivery,
        MessageType::ResponseToAcceptDelivery,
        MessageType::MediateDelivery,
        MessageType::ResponseToMediateDelivery,
        MessageType::ProcessDelivery,
        MessageType::ResponseToProcessDelivery,
        MessageType::PartialStoreDelivery,
        MessageType::ResponseToPartialStoreDelivery,
        MessageType::PartialFetchDelivery,
        MessageType::ResponseToPartialFetchDelivery,
    ];

    /// The local name of the message-type XML element, e.g. `"storeDelivery"`.
    pub fn local_name(self) -> &'static str {
        use MessageType::*;
        match self {
            InitDialog => "initDialog",
            ResponseToInitDialog => "responseToInitDialog",
            ExitDialog => "exitDialog",
            ResponseToExitDialog => "responseToExitDialog",
            GetMessageId => "getMessageId",
            ResponseToGetMessageId => "responseToGetMessageId",
            StoreDelivery => "storeDelivery",
            ResponseToStoreDelivery => "responseToStoreDelivery",
            FetchDelivery => "fetchDelivery",
            ResponseToFetchDelivery => "responseToFetchDelivery",
            FetchProcessCard => "fetchProcessCard",
            ResponseToFetchProcessCard => "responseToFetchProcessCard",
            ForwardDelivery => "forwardDelivery",
            ResponseToForwardDelivery => "responseToForwardDelivery",
            AcceptDelivery => "acceptDelivery",
            ResponseToAcceptDelivery => "responseToAcceptDelivery",
            MediateDelivery => "mediateDelivery",
            ResponseToMediateDelivery => "responseToMediateDelivery",
            ProcessDelivery => "processDelivery",
            ResponseToProcessDelivery => "responseToProcessDelivery",
            PartialStoreDelivery => "partialStoreDelivery",
            ResponseToPartialStoreDelivery => "responseToPartialStoreDelivery",
            PartialFetchDelivery => "partialFetchDelivery",
            ResponseToPartialFetchDelivery => "responseToPartialFetchDelivery",
        }
    }

    /// The namespaced element name. The `partial*` messages use the osci12/7
    /// namespace, everything else the base OSCI namespace.
    pub fn element(self) -> QName {
        use MessageType::*;
        let uri = match self {
            PartialStoreDelivery
            | ResponseToPartialStoreDelivery
            | PartialFetchDelivery
            | ResponseToPartialFetchDelivery => ns::OSCI2017,
            _ => ns::OSCI,
        };
        QName::new(uri, self.local_name())
    }

    /// Look up a variant by its namespaced element name.
    pub fn from_element(q: &QName) -> Result<Self> {
        MessageType::ALL
            .iter()
            .copied()
            .find(|m| &m.element() == q)
            .ok_or_else(|| Error::UnknownMessageType(q.clone()))
    }

    /// `true` for a request, `false` for a `responseTo*`.
    pub fn is_request(self) -> bool {
        !self.local_name().starts_with("responseTo")
    }

    /// Whether the message-type element sits in `soap:Body` rather than
    /// `soap:Header`. True for the message types that carry no header-worthy
    /// parameters of their own (nothing for an `osci:Osci*` header block):
    /// the official Java osci-bibliothek's `compose()` places a bare
    /// `body = new Body("<...>...</...>")` marker for each of these instead
    /// of adding an `osciH` header part. Every other request type
    /// (StoreDelivery, FetchDelivery, ForwardDelivery, ...) keeps its
    /// message-type element in the header, alongside its real parameters.
    pub fn element_in_body(self) -> bool {
        matches!(
            self,
            MessageType::InitDialog
                | MessageType::ResponseToInitDialog
                | MessageType::ExitDialog
                | MessageType::ResponseToExitDialog
                | MessageType::GetMessageId
                | MessageType::ResponseToGetMessageId
                | MessageType::FetchProcessCard
                | MessageType::ResponseToFetchProcessCard
        )
    }

    /// The `soap:Envelope` schema file name for `xsi:schemaLocation`.
    pub fn soap_schema(self) -> String {
        let local = self.local_name();
        let mut c = local.chars();
        let first = c.next().unwrap().to_ascii_uppercase();
        format!("soap{first}{}.xsd", c.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_roundtrip() {
        for m in MessageType::ALL {
            assert_eq!(MessageType::from_element(&m.element()).unwrap(), *m);
        }
    }

    #[test]
    fn partial_namespace() {
        assert_eq!(MessageType::PartialStoreDelivery.element().ns, ns::OSCI2017);
        assert_eq!(MessageType::StoreDelivery.element().ns, ns::OSCI);
    }

    #[test]
    fn request_flag() {
        assert!(MessageType::StoreDelivery.is_request());
        assert!(!MessageType::ResponseToStoreDelivery.is_request());
    }
}
