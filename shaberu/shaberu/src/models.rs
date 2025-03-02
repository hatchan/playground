use std::fmt::Display;
use thiserror::Error;
use tracing::trace;

pub struct GetRoomRequest {
    pub room_id: RoomId,
}

impl GetRoomRequest {
    pub fn new(room_id: impl Into<RoomId>) -> Self {
        Self {
            room_id: room_id.into(),
        }
    }
}

impl From<RoomId> for GetRoomRequest {
    fn from(room_id: RoomId) -> Self {
        GetRoomRequest { room_id }
    }
}

pub struct GetRoomResponse {
    pub room: Room,
}

#[derive(Error, Debug)]
pub enum GetRoomError {
    #[error("Invalid room id was provided")]
    InvalidRoomId,

    #[error("Room specified with id {0} was not found")]
    RoomNotFound(RoomId),
}

#[derive(Debug)]
pub struct Room {
    pub room_id: RoomId,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RoomId([u8; 16]);

impl RoomId {
    pub fn new(raw_room_id: impl AsRef<[u8]>) -> Result<Self, InvalidRoomId> {
        let mut room_id = [0; 16];

        hex::decode_to_slice(raw_room_id, &mut room_id).map_err(|_err| InvalidRoomId)?;

        Ok(RoomId(room_id))
    }
}

impl TryInto<RoomId> for String {
    type Error = InvalidRoomId;

    fn try_into(self) -> Result<RoomId, Self::Error> {
        RoomId::new(self)
    }
}

impl Display for RoomId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        hex::encode(self.0).fmt(f)
    }
}

#[derive(Debug, Error)]
#[error("Invalid room id provided")]
pub struct InvalidRoomId;
