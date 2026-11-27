// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

/// DER-encoded X.509 certificate bytes. The crypto crate turns these into keys.
pub type CertDer = Vec<u8>;

/// The signature and encryption certificate slots of one role.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Certificates {
    /// The certificate whose key verifies this role's signatures.
    pub signature: Option<CertDer>,
    /// The certificate whose key wraps content or transport keys for this role.
    pub encryption: Option<CertDer>,
}

/// The OSCI roles a party can hold in a message.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RoleKind {
    /// Request-level sender.
    Originator,
    /// Request-level recipient.
    Addressee,
    /// Content-level author (end-to-end encrypted).
    Author,
    /// Content-level reader (end-to-end encrypted).
    Reader,
    /// The neutral post office.
    Intermediary,
}

impl RoleKind {
    /// Every role kind, for lookups that need to search across all of them.
    pub const ALL: &'static [RoleKind] = &[
        RoleKind::Originator,
        RoleKind::Addressee,
        RoleKind::Author,
        RoleKind::Reader,
        RoleKind::Intermediary,
    ];

    /// The `osci:{Signature,Cipher}Certificate<suffix>` element suffix this
    /// role's cert slots use, e.g. `Originator` -> `"Originator"`,
    /// `Author`/`Reader` -> `"OtherAuthor"`/`"OtherReader"` (content-level
    /// roles are always "other" relative to the request-level Originator/
    /// Addressee, per the OSCI 1.2 schema).
    pub fn cert_slot_suffix(self) -> &'static str {
        match self {
            RoleKind::Originator => "Originator",
            RoleKind::Addressee => "Addressee",
            RoleKind::Intermediary => "Intermediary",
            RoleKind::Author => "OtherAuthor",
            RoleKind::Reader => "OtherReader",
        }
    }
}

/// One party in a message: a role kind, an optional id, and its cert slots.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Role {
    /// Which role this party holds.
    pub kind: RoleKind,
    /// The role's id, if the message carries one.
    pub id: Option<String>,
    /// The role's certificate slots.
    pub certs: Certificates,
}

impl Role {
    /// A role of the given kind with no id and empty cert slots.
    pub fn new(kind: RoleKind) -> Self {
        Role {
            kind,
            id: None,
            certs: Certificates::default(),
        }
    }
}

macro_rules! role_ctor {
    ($name:ident, $kind:ident) => {
        #[doc = concat!("A fresh [`RoleKind::", stringify!($kind), "`] role.")]
        pub fn $name() -> Self {
            Role::new(RoleKind::$kind)
        }
    };
}

impl Role {
    role_ctor!(originator, Originator);
    role_ctor!(addressee, Addressee);
    role_ctor!(author, Author);
    role_ctor!(reader, Reader);
    role_ctor!(intermediary, Intermediary);
}
