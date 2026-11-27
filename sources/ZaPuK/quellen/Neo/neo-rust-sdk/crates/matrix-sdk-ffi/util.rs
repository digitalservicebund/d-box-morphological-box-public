// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

use crate::neo::error::MessageServiceError;
use crate::neo::events::SerializableEventContent;
use matrix_sdk::Room;
use matrix_sdk_common::deserialized_responses::TimelineEvent;
use ruma::OwnedEventId;

pub fn convert_timeline(items: Vec<TimelineEvent>) -> Vec<SerializableEventContent> {
    items.into_iter().filter_map(|item| {
        let item = item.into_raw().deserialize_as_unchecked()
            .map_err(|e| MessageServiceError::ParseError { msg: format!("parse error: {}", e.to_string())});

        if let Err(e) = &item {
            println!("error: {}", e);
        }

        item.ok()
    }).collect()
}

pub async fn get_latest_thread_event(
    room: &Room,
    thread_id: &OwnedEventId
) -> Result<Option<OwnedEventId>, MessageServiceError> {
    let (event_cache, _) = room
        .event_cache()
        .await
        .map_err(|e| MessageServiceError::TimelineError { msg: e.to_string() })?;

    let thread = event_cache.thread_pagination(thread_id.clone()).await
        .map_err(|e| MessageServiceError::RequestError { msg: format!("could not load dialog: {}", e.to_string())})?;
    let pagination = thread.run_backwards_once(1).await
        .map_err(|e| MessageServiceError::RequestError { msg: format!("could not load dialog content: {}", e.to_string())})?;
    let event = pagination.events.last();

    Ok(event.and_then(|e| { e.event_id() }))
}
