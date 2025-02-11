use serde::Serialize;
use std::fmt::{self, Display};
use strum::IntoStaticStr;

use super::types::WSCommand;

#[derive(IntoStaticStr)]
pub enum WSErrorType {
    UserNotFound(String),

    TransportConnectionFailure,

    ProducerFailure,
    ProducerNotFound(String),

    ConsumerFailure,
    ConsumerNotFound(String),
}

impl Display for WSErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WSErrorType::UserNotFound(id) => write!(f, "User with ID {} doesn't exist", id),
            WSErrorType::TransportConnectionFailure => {
                write!(f, "An error occured while trying to connect transport")
            }

            WSErrorType::ProducerFailure => write!(
                f,
                "An unknown error occured while setting up an RTC producer"
            ),
            WSErrorType::ProducerNotFound(id) => write!(f, "Producer with ID {} doesn't exist", id),

            WSErrorType::ConsumerFailure => write!(
                f,
                "An unknown error occured while setting up an RTC consumer"
            ),
            WSErrorType::ConsumerNotFound(id) => write!(f, "Consumer with ID {} doesn't exist", id),
        }
    }
}

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum WSCloseType {
    /// Sent when the received data is unparseable
    InvalidData = 1003,
    /// Sent when a client tries to send a command in the wrong state
    InvalidState = 1002,
    Unauthorized = 4001,
    Kicked = 4003,
    RoomClosed = 4004,
    ServerError = 1011,
}

impl Display for WSCloseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WSCloseType::InvalidData => write!(f, "Unable to parse data"),
            WSCloseType::InvalidState => write!(f, "Command executed in invalid state"),
            WSCloseType::Unauthorized => write!(f, "Invalid token"),
            WSCloseType::Kicked => write!(f, "You have been kicked!"),
            WSCloseType::RoomClosed => write!(f, "Room has been closed"),
            WSCloseType::ServerError => write!(f, "Internal Server Error"),
        }
    }
}

impl From<serde_json::Error> for WSCloseType {
    fn from(_: serde_json::Error) -> WSCloseType {
        WSCloseType::InvalidData
    }
}

impl From<warp::Error> for WSCloseType {
    fn from(_: warp::Error) -> WSCloseType {
        WSCloseType::ServerError
    }
}

#[derive(Serialize)]
pub struct WSError<'a> {
    id: Option<String>,
    #[serde(rename = "type")]
    command_type: &'a str,
    error: &'static str,
    message: String,
}

impl<'a> WSError<'a> {
    pub fn new(id: Option<String>, command_type: &'a str, error: WSErrorType) -> Self {
        WSError {
            id,
            command_type,
            message: error.to_string(),
            error: error.into(),
        }
    }

    pub fn from_command(command: WSCommand, error: WSErrorType) -> Self {
        let id = command.id;
        let command_type: &'static str = command.command_type.into();
        WSError {
            id,
            command_type,
            message: error.to_string(),
            error: error.into(),
        }
    }
}
