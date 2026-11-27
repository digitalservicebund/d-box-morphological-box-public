// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

//! Small browser bindings over the unmodified upstream Matrix Rust SDK.
use matrix_sdk::{
    Client, Room, RoomState,
    authentication::matrix::MatrixSession,
    config::{RequestConfig, SyncSettings},
    media::{MediaFormat, MediaRequestParameters},
    room::MessagesOptions,
    ruma::{
        EventId, OwnedTransactionId, RoomId, UserId,
        api::client::{
            discovery::get_capabilities,
            profile::{AvatarUrl, DisplayName, ProfileFieldName},
            room::create_room::{self, v3::RoomPreset},
        },
        events::room::{EncryptedFile, MediaSource, create::RoomCreateEventContent},
        serde::Raw,
    },
};
use serde_json::{Value, json};
use std::{cell::RefCell, collections::HashMap, io::Cursor, time::Duration};
use wasm_bindgen::prelude::*;

fn error(value: impl std::fmt::Display) -> JsValue {
    js_sys_error(&value.to_string())
}

#[wasm_bindgen(inline_js = r#"
export function js_sys_error(message) { return new Error(message); }
export function session_changed(session) {
    globalThis.postMessage({ kind: 'session', session: JSON.parse(session) });
}
"#)]
extern "C" {
    fn js_sys_error(message: &str) -> JsValue;
    fn session_changed(session: &str);
}

fn encode(value: &impl serde::Serialize) -> Result<String, JsValue> {
    serde_json::to_string(value).map_err(error)
}

fn can_set_profile_field(
    capabilities: Option<&get_capabilities::v3::Capabilities>,
    field: ProfileFieldName,
    legacy: &str,
) -> bool {
    let Some(capabilities) = capabilities else {
        return true;
    };
    if let Some(fields) = &capabilities.profile_fields {
        return fields.can_set_field(&field);
    }
    capabilities
        .get(legacy)
        .and_then(|value| value.get("enabled").and_then(Value::as_bool))
        .unwrap_or(true)
}

#[wasm_bindgen]
pub struct MatrixClient {
    client: Client,
    last_updates: RefCell<HashMap<String, u64>>,
}

impl MatrixClient {
    fn room(&self, id: &str) -> Result<Room, JsValue> {
        let room_id = RoomId::parse(id).map_err(error)?;
        self.client
            .get_room(&room_id)
            .ok_or_else(|| error("Case is not available in this session"))
    }
}

#[wasm_bindgen]
impl MatrixClient {
    pub async fn open(
        homeserver: String,
        database: String,
        session: Option<String>,
    ) -> Result<MatrixClient, JsValue> {
        let client = Client::builder()
            .homeserver_url(homeserver)
            .indexeddb_store(&database, None)
            .handle_refresh_tokens()
            .request_config(
                RequestConfig::new()
                    .timeout(Duration::from_secs(40))
                    .retry_limit(2),
            )
            .build()
            .await
            .map_err(error)?;
        client
            .set_session_callbacks(
                Box::new(|client| {
                    client
                        .session_tokens()
                        .ok_or_else(|| std::io::Error::other("No active session").into())
                }),
                Box::new(|client| {
                    let session = client
                        .matrix_auth()
                        .session()
                        .ok_or_else(|| std::io::Error::other("No active session"))?;
                    session_changed(&serde_json::to_string(&session)?);
                    Ok(())
                }),
            )
            .map_err(error)?;
        if let Some(session) = session {
            let session: MatrixSession = serde_json::from_str(&session).map_err(error)?;
            client.restore_session(session).await.map_err(error)?;
        }
        Ok(Self {
            client,
            last_updates: RefCell::new(HashMap::new()),
        })
    }

    pub async fn login(&self, username: String, password: String) -> Result<String, JsValue> {
        self.client
            .matrix_auth()
            .login_username(&username, &password)
            .initial_device_display_name("Neo Agency Dev Tool")
            .request_refresh_token()
            .send()
            .await
            .map_err(error)?;
        self.session()
    }

    pub fn session(&self) -> Result<String, JsValue> {
        encode(
            &self
                .client
                .matrix_auth()
                .session()
                .ok_or_else(|| error("Not logged in"))?,
        )
    }

    pub fn homeserver(&self) -> String {
        self.client.homeserver().to_string()
    }

    pub async fn profile(&self) -> Result<String, JsValue> {
        let name = self
            .client
            .account()
            .get_display_name()
            .await
            .map_err(error)?;
        let avatar_url = self
            .client
            .account()
            .get_avatar_url()
            .await
            .map_err(error)?;
        let capabilities = self
            .client
            .send(get_capabilities::v3::Request::new())
            .await
            .ok()
            .map(|response| response.capabilities);
        encode(
            &json!({ "mxid": self.client.user_id(), "displayName": name, "avatarUrl": avatar_url,
                "canSetDisplayName": can_set_profile_field(capabilities.as_ref(), ProfileFieldName::DisplayName, "m.set_displayname"),
                "canSetAvatar": can_set_profile_field(capabilities.as_ref(), ProfileFieldName::AvatarUrl, "m.set_avatar_url") }),
        )
    }

    pub async fn sender_profile(&self, sender: String) -> Result<String, JsValue> {
        let user_id = UserId::parse(sender).map_err(error)?;
        let profile = self
            .client
            .account()
            .fetch_user_profile_of(&user_id)
            .await
            .map_err(error)?;
        let name = profile.get_static::<DisplayName>().unwrap_or(None);
        let avatar = profile.get_static::<AvatarUrl>().unwrap_or(None);
        encode(&json!({ "mxid": user_id, "displayName": name, "avatarUrl": avatar }))
    }

    pub async fn set_display_name(&self, name: String) -> Result<(), JsValue> {
        let name = name.trim();
        self.client
            .account()
            // An empty string also uses the stable profile PUT on MAS-backed Synapse.
            .set_display_name(Some(name))
            .await
            .map_err(error)
    }

    pub async fn set_avatar(&self, bytes: Vec<u8>, mimetype: String) -> Result<(), JsValue> {
        let content_type: mime::Mime = mimetype.parse().map_err(error)?;
        if content_type.type_() != mime::IMAGE {
            return Err(error("Select an image for the avatar"));
        }
        self.client
            .account()
            .upload_avatar(&content_type, bytes)
            .await
            .map_err(error)?;
        Ok(())
    }

    pub async fn logout(&self) -> Result<(), JsValue> {
        self.client.logout().await.map_err(error)
    }

    pub async fn sync(&self, since: Option<String>) -> Result<String, JsValue> {
        let mut settings = SyncSettings::default().timeout(Duration::from_secs(25));
        if let Some(token) = since {
            settings = settings.token(token);
        }
        let response = self.client.sync_once(settings).await.map_err(error)?;
        let rooms: Vec<Value> = response
            .rooms
            .joined
            .into_iter()
            .map(|(id, update)| {
                if let Some(timestamp) = update
                    .timeline
                    .events
                    .iter()
                    .filter_map(|event| event.timestamp())
                    .map(|timestamp| u64::from(timestamp.0))
                    .max()
                {
                    self.last_updates
                        .borrow_mut()
                        .entry(id.to_string())
                        .and_modify(|current| *current = (*current).max(timestamp))
                        .or_insert(timestamp);
                }
                let events: Vec<_> = update
                    .timeline
                    .events
                    .iter()
                    .map(|event| event.raw())
                    .collect();
                json!({ "roomId": id, "events": events, "limited": update.timeline.limited })
            })
            .collect();
        encode(&json!({ "nextBatch": response.next_batch, "rooms": rooms,
            "keysChanged": !response.to_device.is_empty() }))
    }

    pub async fn rooms(&self) -> Result<String, JsValue> {
        let mut rooms = Vec::new();
        for room in self.client.rooms() {
            if room.is_space() || !matches!(room.state(), RoomState::Joined | RoomState::Invited) {
                continue;
            }
            let name = room.display_name().await.map_err(error)?.to_string();
            let created_at = room
                .get_state_event_static::<RoomCreateEventContent>()
                .await
                .ok()
                .flatten()
                .and_then(|event| event.deserialize().ok())
                .and_then(|event| event.origin_server_ts())
                .map(|timestamp| u64::from(timestamp.0));
            let observed_update = self
                .last_updates
                .borrow()
                .get(room.room_id().as_str())
                .copied();
            let latest_update = room
                .latest_event_timestamp()
                .map(|timestamp| u64::from(timestamp.0));
            let updated_at = [observed_update, latest_update, created_at]
                .into_iter()
                .flatten()
                .max();
            let citizen = room
                .members(matrix_sdk::RoomMemberships::ACTIVE)
                .await
                .ok()
                .and_then(|members| {
                    let mut others = members
                        .into_iter()
                        .filter(|member| Some(member.user_id()) != self.client.user_id());
                    let citizen = others.next()?.user_id().to_string();
                    others.next().is_none().then_some(citizen)
                });
            rooms.push(
                json!({ "id": room.room_id(), "name": name, "topic": room.topic(),
                "encrypted": room.encryption_state().is_encrypted(),
                "invited": room.state() == RoomState::Invited, "createdAt": created_at, "updatedAt": updated_at,
                "citizen": citizen }),
            );
        }
        encode(&rooms)
    }

    pub async fn join(&self, room_id: String) -> Result<(), JsValue> {
        self.room(&room_id)?.join().await.map_err(error)?;
        Ok(())
    }

    pub async fn invite(&self, room_id: String, recipient: String) -> Result<(), JsValue> {
        let user_id = UserId::parse(recipient).map_err(error)?;
        self.room(&room_id)?
            .invite_user_by_id(&user_id)
            .await
            .map_err(error)?;
        Ok(())
    }

    pub async fn update_case_metadata(
        &self,
        room_id: String,
        name: String,
        topic: String,
    ) -> Result<(), JsValue> {
        let room = self.room(&room_id)?;
        room.set_name(name).await.map_err(error)?;
        room.set_room_topic(&topic).await.map_err(|cause| {
            error(format!(
                "Case title saved, but agency name could not be updated: {cause}"
            ))
        })?;
        Ok(())
    }

    pub async fn remove_case(&self, room_id: String) -> Result<String, JsValue> {
        let room = self.room(&room_id)?;
        if !matches!(room.state(), RoomState::Left | RoomState::Banned) {
            room.leave().await.map_err(error)?;
        }
        // Leaving succeeded even if the separate forget request fails.
        let warning = room.forget().await.err().map(|cause| cause.to_string());
        self.last_updates.borrow_mut().remove(&room_id);
        encode(&json!({ "forgotten": warning.is_none(), "warning": warning }))
    }

    pub async fn participants(&self, room_id: String) -> Result<String, JsValue> {
        let room = self.room(&room_id)?;
        let members = room
            .members(matrix_sdk::RoomMemberships::ACTIVE)
            .await
            .map_err(error)?;
        let members: Vec<_> = members
            .iter()
            .filter(|member| Some(member.user_id()) != self.client.user_id())
            .map(|member| member.user_id().to_string())
            .collect();
        encode(&members)
    }

    pub async fn create_case(
        &self,
        recipient: String,
        name: String,
        topic: String,
        encrypted: bool,
    ) -> Result<String, JsValue> {
        let mut request = create_room::v3::Request::new();
        request.name = Some(name);
        request.topic = Some(topic);
        request.preset = Some(RoomPreset::PrivateChat);
        request.invite = vec![UserId::parse(recipient).map_err(error)?];
        let mut state = vec![
            json!({ "type": "m.room.history_visibility", "state_key": "",
            "content": { "history_visibility": "shared" } }),
        ];
        if encrypted {
            state.push(json!({ "type": "m.room.encryption", "state_key": "",
            "content": { "algorithm": "m.megolm.v1.aes-sha2" } }));
        }
        for event in state {
            request.initial_state.push(Raw::from_json(
                serde_json::value::to_raw_value(&event).map_err(error)?,
            ));
        }
        let room = self.client.create_room(request).await.map_err(error)?;
        Ok(room.room_id().to_string())
    }

    pub async fn history(&self, room_id: String, from: Option<String>) -> Result<String, JsValue> {
        let mut options = MessagesOptions::backward();
        options.from = from;
        options.limit = 100u32.into();
        let response = self
            .room(&room_id)?
            .messages(options)
            .await
            .map_err(error)?;
        let events: Vec<_> = response.chunk.iter().map(|event| event.raw()).collect();
        encode(&json!({ "events": events, "next": response.end }))
    }

    pub async fn event(&self, room_id: String, event_id: String) -> Result<String, JsValue> {
        let id = EventId::parse(event_id).map_err(error)?;
        let event = self.room(&room_id)?.event(&id, None).await.map_err(error)?;
        encode(event.raw())
    }

    pub async fn send(
        &self,
        room_id: String,
        event_type: String,
        content: String,
        transaction: String,
    ) -> Result<String, JsValue> {
        let content: Value = serde_json::from_str(&content).map_err(error)?;
        let room = self.room(&room_id)?;
        let transaction: OwnedTransactionId = transaction.into();
        let response = room
            .send_raw(&event_type, content)
            .with_transaction_id(&transaction)
            .await
            .map_err(error)?;
        Ok(response.response.event_id.to_string())
    }

    pub async fn upload(
        &self,
        room_id: String,
        bytes: Vec<u8>,
        mimetype: String,
    ) -> Result<String, JsValue> {
        let room = self.room(&room_id)?;
        if room
            .latest_encryption_state()
            .await
            .map_err(error)?
            .is_encrypted()
        {
            let mut cursor = Cursor::new(bytes);
            let file = self
                .client
                .upload_encrypted_file(&mut cursor)
                .await
                .map_err(error)?;
            encode(&file)
        } else {
            let mime: mime::Mime = mimetype.parse().map_err(error)?;
            let response = self
                .client
                .media()
                .upload(&mime, bytes, None)
                .await
                .map_err(error)?;
            encode(&json!({ "url": response.content_uri }))
        }
    }

    pub async fn download(&self, descriptor: String) -> Result<Vec<u8>, JsValue> {
        let value: Value = serde_json::from_str(&descriptor).map_err(error)?;
        let source = if value.get("key").is_some() {
            MediaSource::Encrypted(Box::new(
                serde_json::from_value::<EncryptedFile>(value).map_err(error)?,
            ))
        } else {
            let uri = value
                .get("url")
                .and_then(Value::as_str)
                .ok_or_else(|| error("Missing media URL"))?;
            MediaSource::Plain(uri.into())
        };
        self.client
            .media()
            .get_media_content(
                &MediaRequestParameters {
                    source,
                    format: MediaFormat::File,
                },
                true,
            )
            .await
            .map_err(error)
    }
}
