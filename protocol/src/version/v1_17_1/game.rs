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

    PlayerPositionAndLook(PlayerPositionAndLook),
    TimeUpdate(TimeUpdate),
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
            GameClientBoundPacket::TimeUpdate(_) => 0x58,
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
            // TODO: 0x25 => {
            // TODO:     let join_game = JoinGame::decode(reader)?;

            // TODO:     Ok(GameClientBoundPacket::JoinGame(join_game))
            // TODO: }
            // TODO: 0x38 => {
            // TODO:     let player_position = PlayerPositionAndLook::decode(reader)?;

            // TODO:     Ok(GameClientBoundPacket::PlayerPositionAndLook(
            // TODO:         player_position,
            // TODO:     ))
            // TODO: }
            // TODO: 0x58 => {
            // TODO:     let time_update = TimeUpdate::decode(reader)?;

            // TODO:     Ok(GameClientBoundPacket::TimeUpdate(time_update))
            // TODO: }
            _ => Err(DecodeError::UnknownPacketType { type_id }),
        }
    }
}

// TODO: implement decoder
// TODO: implement new()
#[derive(Encoder, Debug)]
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

// TODO: implement decoder
// TODO: implement new()
#[derive(Encoder, Debug)]
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

// TODO: implement decoder
// TODO: implement new()
#[derive(Encoder, Debug)]
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

// TODO: implement decoder
// TODO: implement new()
// #[derive(Encoder, Debug)]
// pub struct ChunkData {
//     pub x: i32,
//     pub z: i32,
//     // TODO: should be BitSet type!
//     pub primary_mask: Vec<i64>,
//     pub heightmaps: CompoundTag,

//     #[data_type(with = "var_int_array")]
//     pub biomes: Vec<i32>,

//     // TODO: extract this from block_entities with sized datatype
//     #[data_type(with = "var_int")]
//     pub data_size: i32,
//     pub data: Vec<u8>,

//     // TODO: extract this from block_entities with sized datatype
//     #[data_type(with = "var_int")]
//     pub block_entities_size: i32,
//     pub block_entities: Vec<u8>,
// }
