// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

/// The `osci:DesiredLanguages` header block: the languages the sender wants
/// feedback text in, most preferred first.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DesiredLanguages {
    /// The `Id` attribute, if any.
    pub id: Option<String>,
    /// Language tags in preference order.
    pub languages: Vec<String>,
}

impl DesiredLanguages {
    /// Build from an iterator of language tags.
    pub fn new<I, S>(langs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        DesiredLanguages {
            id: None,
            languages: langs.into_iter().map(Into::into).collect(),
        }
    }

    /// The space-separated `LanguagesList` attribute value.
    pub fn languages_list(&self) -> String {
        self.languages.join(" ")
    }
}

/// The `osci:QualityOfTimestamp` header block: requests a timestamp of the
/// given `service` ("creation" or "reception") at the given `quality`
/// ("plain" for a local wall-clock read, "cryptographic" for one from a
/// qualified timestamping authority). `StoreDelivery` sends one of each
/// service.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QualityOfTimestamp {
    /// The `Id` attribute.
    pub id: String,
    /// The `Service` attribute: "creation" or "reception".
    pub service: String,
    /// The `Quality` attribute.
    pub quality: String,
}

impl QualityOfTimestamp {
    /// A "plain" quality timestamp for `service` ("creation"/"reception").
    pub fn plain(service: impl Into<String>, id: impl Into<String>) -> Self {
        QualityOfTimestamp {
            id: id.into(),
            service: service.into(),
            quality: "plain".into(),
        }
    }
}
