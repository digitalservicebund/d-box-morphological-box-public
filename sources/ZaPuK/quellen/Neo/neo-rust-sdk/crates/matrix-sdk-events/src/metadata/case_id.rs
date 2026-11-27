// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use matrix_sdk::ruma::{
    OwnedEventId,
    events::{EmptyStateKey, macros::EventContent},
};
use serde::{Deserialize, Serialize};

/// Represents a case ID in the NEO system referencing an externals system
///
/// A single entry referencing a case in an external system. To reference
/// several external cases to this entry, multiple CaseIds with different `app` and `relation`
/// fields should be used.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CaseId {
    /// The ID of the case as it is known by the external system
    pub id: String,
    /// An unique ID of the app that owns the case
    pub app: String,
    /// Whether the case is visible to the user or only to the backend
    #[serde(default)]
    pub visible: bool,
    /// The relation of the case within the system
    #[serde(default)]
    pub relation: Option<String>,
}

/// The full set of case IDs in the NEO system referencing external systems
///
/// As this event overwrites the previous state, the `cases` field should be treated as the
/// complete set of case IDs for the event's room.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent, PartialEq)]
#[ruma_event(type = "dev.fitko.neo.meta.case_id.v1", kind = MessageLike)]
pub struct CaseIdMetadataEventContent {
    #[serde(rename = "dev.fitko.neo.meta.case_id.v1")]
    cases: Vec<CaseId>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LatestCaseIdContent {
    pub latest: OwnedEventId,
}

/// The latest list of known case IDs for the event's room/case
#[derive(Clone, Debug, Deserialize, Serialize, EventContent, PartialEq)]
#[ruma_event(type = "dev.fitko.neo.state.meta.case.v1", kind = State, state_key_type = EmptyStateKey)]
pub struct CaseIdStateEventContent {
    /// The event ID of the latest `CaseId
    /// MetadataEventContent` event for this room for easier lookup
    #[serde(rename = "dev.fitko.neo.state.meta.case.v1")]
    pub content: LatestCaseIdContent,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_helpers::*;
    use matrix_sdk::ruma::event_id;

    #[test]
    fn test_case_id_metadata_event_content_serialization() -> Result<(), Box<dyn std::error::Error>>
    {
        let content = CaseIdMetadataEventContent {
            cases: vec![CaseId {
                id: "case-1".to_string(),
                app: "app-1".to_string(),
                visible: true,
                relation: None,
            }],
        };
        assert_valid(&content)?;
        assert_serialization_roundtrip(content)?;
        Ok(())
    }

    #[test]
    fn test_case_id_metadata_example() -> Result<(), Box<dyn std::error::Error>> {
        let example_serialized = get_example("dev.fitko.neo.meta.case_id.v1")?;
        // ensure the example is valid
        assert_valid(&example_serialized)?;

        let example: CaseIdMetadataEventContent =
            serde_json::from_value(example_serialized.get("content").unwrap().clone())?;
        assert_eq!(example.cases.len(), 2);
        assert_eq!(
            example.cases[0],
            CaseId {
                id: "refId-1".to_string(),
                app: "drachenbeantragungsverfahrenssystem".to_string(),
                visible: false,
                relation: None,
            }
        );
        assert_eq!(
            example.cases[1],
            CaseId {
                id: "MSCDas-123q312".to_string(),
                app: "system-2".to_string(),
                visible: true,
                relation: Some("parent".to_string()),
            }
        );
        Ok(())
    }

    #[test]
    fn test_case_id_metadata_state_event_content_serialization()
    -> Result<(), Box<dyn std::error::Error>> {
        let content = CaseIdStateEventContent {
            content: LatestCaseIdContent {
                latest: "$refId-1".parse().unwrap(),
            },
        };
        assert_valid(&content)?;
        assert_serialization_roundtrip(content)?;
        Ok(())
    }

    #[test]
    fn test_case_id_state_event_content_example() -> Result<(), Box<dyn std::error::Error>> {
        let example_serialized = get_example("dev.fitko.neo.state.meta.case.v1")?;
        assert_valid(&example_serialized)?;
        let example: OriginalCaseIdStateEvent = serde_json::from_value(example_serialized)?;
        assert_eq!(example.state_key, EmptyStateKey);
        assert_eq!(example.content.content.latest, event_id!("$refId-1"));
        Ok(())
    }
}
