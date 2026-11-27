// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::chain::{send_chain, ThreadState};
use crate::neo::error::MessageServiceError;
use crate::neo::model::{NeoCase, NeoContent};
use crate::neo::parse::parse_case;
use crate::neo::util::{convert_timeline, get_latest_thread_event};
use matrix_sdk::Client as MatrixClient;
use ruma::{EventId, RoomId};
use std::ops::Deref;

#[matrix_sdk_ffi_macros::export]
pub fn message_service(
    client: &crate::client::Client,
) -> crate::neo::message_service::MessageService {
    crate::neo::message_service::MessageService { client: client.inner.clone().deref().clone() }
}

#[derive(uniffi::Object)]
pub struct MessageService {
    pub client: MatrixClient,
}

#[matrix_sdk_ffi_macros::export]
impl MessageService {
    pub async fn load_case(&self, case_id: &str) -> Result<NeoCase, MessageServiceError> {
        let room_id = RoomId::parse(case_id)
            .map_err(|e| MessageServiceError::IdError { msg: e.to_string() })?;
        let room = self.client.get_room(&room_id).ok_or(MessageServiceError::RoomNotFoundError)?;

        let (event_cache, _) = room
            .event_cache()
            .await
            .map_err(|e| MessageServiceError::TimelineError { msg: e.to_string() })?;
        event_cache
            .pagination()
            .run_backwards_until(999)
            .await
            .map_err(|e| MessageServiceError::TimelineError { msg: e.to_string() })?;
        let items = event_cache
            .events()
            .await
            .map_err(|e| MessageServiceError::TimelineError { msg: e.to_string() })?;

        let items = parse_case(case_id.to_string(), convert_timeline(items));

        Ok(items)
    }

    pub async fn send_envelope(
        &self,
        case_id: &str,
        blocks: Vec<NeoContent>
    ) -> Result<String, MessageServiceError> {
        let room_id = RoomId::parse(case_id)
            .map_err(|e| MessageServiceError::IdError { msg: e.to_string() })?;
        let room = self.client.get_room(&room_id)
            .ok_or(MessageServiceError::RoomNotFoundError)?;

        let thread_state = send_chain(&room, blocks, None).await?;
        Ok(thread_state.root.to_string())
    }

    pub async fn send_answer(
        &self,
        case_id: &str,
        dialog_id: &str,
        blocks: Vec<NeoContent>
    ) -> Result<String, MessageServiceError> {
        let room_id = RoomId::parse(case_id)
            .map_err(|e| MessageServiceError::IdError { msg: e.to_string() })?;
        let room = self.client.get_room(&room_id)
            .ok_or(MessageServiceError::RoomNotFoundError)?;

        let thread_id = EventId::parse(dialog_id)
            .map_err(|e| MessageServiceError::ParseError { msg: format!("invalid thread_id: {}", e) })?;

        let previous_id = get_latest_thread_event(&room, &thread_id).await?
            .ok_or(MessageServiceError::TimelineError { msg: "could not find latest thread message".to_string() })?;

        let thread_state = send_chain(&room, blocks, Some(ThreadState { root: thread_id, last: previous_id } )).await?;
        Ok(thread_state.root.to_string())
    }
}
