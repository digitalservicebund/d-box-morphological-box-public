// SPDX-FileCopyrightText: 2026 FITKO (Föderale IT-Kooperation)
//
// SPDX-License-Identifier: EUPL-1.2

#[derive(Debug, thiserror::Error, uniffi::Error)]
#[uniffi(flat_error)]
pub enum MessageServiceError {
    #[error("error parsing id: {:?}", msg)]
    IdError { msg: String },
    #[error("error loading room timeline: {:?}", msg)]
    TimelineError { msg: String },
    #[error("error loading room.")]
    RoomNotFoundError,
    #[error("error during request: {:?}", msg)]
    RequestError { msg: String },
    #[error("error parsing message: {:?}", msg)]
    ParseError { msg: String },
    #[error("error sending message: cannot send empty message")]
    EmptyMessageError,
}
