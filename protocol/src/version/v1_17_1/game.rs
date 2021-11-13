use crate::data::chat::Message;
use crate::decoder::Decoder;
use crate::error::DecodeError;
use minecraft_protocol_derive::Decoder;
use minecraft_protocol_derive::Encoder;
use nbt::CompoundTag;
use std::io::Read;

// Re-export Minecraft 1.14.4 types
pub use super::super::v1_14_4::game::{
    BossBar, ChunkData, ClientBoundChatMessage, ClientBoundKeepAlive, EntityAction, GameDisconnect,
    ServerBoundAbilities, ServerBoundChatMessage, ServerBoundKeepAlive,
};

pub enum GameServerBoundPacket {
    ServerBoundChatMessage(ServerBoundChatMessage),
    ServerBoundKeepAlive(ServerBoundKeepAlive),
    ServerBoundAbilities(ServerBoundAbilities),
}

pub enum GameClientBoundPacket {
    ClientBoundChatMessage(ClientBoundChatMessage),
    JoinGame(JoinGame),
    ClientBoundKeepAlive(ClientBoundKeepAlive),
    ChunkData(ChunkData),
    GameDisconnect(GameDisconnect),
    BossBar(BossBar),
    EntityAction(EntityAction),

    Respawn(Respawn),
    PlayerPositionAndLook(PlayerPositionAndLook),
    SpawnPosition(SpawnPosition),
    SetTitleSubtitle(SetTitleSubtitle),
    SetTitleText(SetTitleText),
    TimeUpdate(TimeUpdate),
    SetTitleTimes(SetTitleTimes),
}

impl GameServerBoundPacket {
    pub fn get_type_id(&self) -> u8 {
        match self {
            GameServerBoundPacket::ServerBoundChatMessage(_) => 0x03,
            GameServerBoundPacket::ServerBoundKeepAlive(_) => 0x0F,
            GameServerBoundPacket::ServerBoundAbilities(_) => 0x19,
        }
    }

    pub fn decode<R: Read>(type_id: u8, reader: &mut R) -> Result<Self, DecodeError> {
        match type_id {
            0x03 => {
                let chat_message = ServerBoundChatMessage::decode(reader)?;

                Ok(GameServerBoundPacket::ServerBoundChatMessage(chat_message))
            }
            0x0F => {
                let keep_alive = ServerBoundKeepAlive::decode(reader)?;

                Ok(GameServerBoundPacket::ServerBoundKeepAlive(keep_alive))
            }
            _ => Err(DecodeError::UnknownPacketType { type_id }),
        }
    }
}

impl GameClientBoundPacket {
    pub fn get_type_id(&self) -> u8 {
        match self {
            GameClientBoundPacket::ClientBoundChatMessage(_) => 0x0E,
            GameClientBoundPacket::GameDisconnect(_) => 0x1A,
            GameClientBoundPacket::ClientBoundKeepAlive(_) => 0x20,
            GameClientBoundPacket::ChunkData(_) => 0x21,
            GameClientBoundPacket::JoinGame(_) => 0x25,
            GameClientBoundPacket::BossBar(_) => 0x0D,
            GameClientBoundPacket::EntityAction(_) => 0x1B,
            GameClientBoundPacket::PlayerPositionAndLook(_) => 0x38,
            GameClientBoundPacket::Respawn(_) => 0x3D,
            GameClientBoundPacket::SpawnPosition(_) => 0x4B,
            GameClientBoundPacket::SetTitleSubtitle(_) => 0x57,
            GameClientBoundPacket::TimeUpdate(_) => 0x58,
            GameClientBoundPacket::SetTitleText(_) => 0x59,
            GameClientBoundPacket::SetTitleTimes(_) => 0x5A,
        }
    }

    pub fn decode<R: Read>(type_id: u8, reader: &mut R) -> Result<Self, DecodeError> {
        match type_id {
            0x0E => {
                let chat_message = ClientBoundChatMessage::decode(reader)?;

                Ok(GameClientBoundPacket::ClientBoundChatMessage(chat_message))
            }
            0x1A => {
                let game_disconnect = GameDisconnect::decode(reader)?;

                Ok(GameClientBoundPacket::GameDisconnect(game_disconnect))
            }
            0x20 => {
                let keep_alive = ClientBoundKeepAlive::decode(reader)?;

                Ok(GameClientBoundPacket::ClientBoundKeepAlive(keep_alive))
            }
            0x21 => {
                let chunk_data = ChunkData::decode(reader)?;

                Ok(GameClientBoundPacket::ChunkData(chunk_data))
            }
            0x25 => {
                let join_game = JoinGame::decode(reader)?;

                Ok(GameClientBoundPacket::JoinGame(join_game))
            }
            0x3D => {
                let respawn = Respawn::decode(reader)?;

                Ok(GameClientBoundPacket::Respawn(respawn))
            }
            0x38 => {
                let player_position = PlayerPositionAndLook::decode(reader)?;

                Ok(GameClientBoundPacket::PlayerPositionAndLook(
                    player_position,
                ))
            }
            0x4B => {
                let spawn_position = SpawnPosition::decode(reader)?;

                Ok(GameClientBoundPacket::SpawnPosition(spawn_position))
            }
            0x57 => {
                let title_subtitle = SetTitleSubtitle::decode(reader)?;

                Ok(GameClientBoundPacket::SetTitleSubtitle(title_subtitle))
            }
            0x58 => {
                let time_update = TimeUpdate::decode(reader)?;

                Ok(GameClientBoundPacket::TimeUpdate(time_update))
            }
            0x59 => {
                let title_text = SetTitleText::decode(reader)?;

                Ok(GameClientBoundPacket::SetTitleText(title_text))
            }
            0x5A => {
                let title_times = SetTitleTimes::decode(reader)?;

                Ok(GameClientBoundPacket::SetTitleTimes(title_times))
            }
            _ => Err(DecodeError::UnknownPacketType { type_id }),
        }
    }
}

// TODO(timvisee): implement new()
// TODO(timvisee): remove clone?
#[derive(Clone, Encoder, Decoder, Debug)]
pub struct JoinGame {
    pub entity_id: u32,
    pub hardcore: bool,
    pub game_mode: u8,
    pub previous_game_mode: u8,
    pub world_names: Vec<String>,
    pub dimension_codec: CompoundTag,
    pub dimension: CompoundTag,
    pub world_name: String,
    pub hashed_seed: i64,
    #[data_type(with = "var_int")]
    pub max_players: i32,
    #[data_type(with = "var_int")]
    pub view_distance: i32,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
}

// TODO(timvisee): implement new()
#[derive(Encoder, Decoder, Debug)]
pub struct Respawn {
    pub dimension: CompoundTag,
    pub world_name: String,
    pub hashed_seed: i64,
    pub game_mode: u8,
    pub previous_game_mode: u8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub copy_metadata: bool,
}

// TODO(timvisee): implement new()
#[derive(Encoder, Decoder, Debug)]
pub struct PlayerPositionAndLook {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
    #[data_type(with = "var_int")]
    pub teleport_id: i32,
    pub dismount_vehicle: bool,
}

// TODO(timvisee): implement new()
#[derive(Encoder, Decoder, Debug)]
pub struct TimeUpdate {
    pub world_age: i64,
    pub time_of_day: i64,
}

#[derive(Encoder, Decoder, Debug)]
pub struct SetTitleText {
    pub text: Message,
}

#[derive(Encoder, Decoder, Debug)]
pub struct SetTitleSubtitle {
    pub text: Message,
}

#[derive(Encoder, Decoder, Debug)]
pub struct SetTitleTimes {
    pub fade_in: i32,
    pub stay: i32,
    pub fade_out: i32,
}

#[derive(Encoder, Decoder, Debug)]
pub struct SpawnPosition {
    pub position: u64,
    pub angle: f32,
}
