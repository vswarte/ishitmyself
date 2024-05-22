use zerocopy::{FromBytes, FromZeroes};

use crate::util::singleton::DLRFLocatable;

#[repr(u32)]
pub enum WorldState {
    // Nothing is happening
    Offline = 0x0,
    // Creating a lobby 
    CreatingLobby = 0x1,
    Unk2 = 0x2,
    // Whenever the player is hosting a lobby, this includes quickmatch pooling
    // lobbies.
    Host = 0x3,
    // Hit when trying to join another player in coop or while waiting in a 
    // quickmatch lobby as a non-host.
    AwaitingJoin = 0x4,
    Unk5 = 0x5,
    // Whenever the player is in someone elses world
    Client = 0x6,
    // Whenever the hosts lobby closes
    ClosingLobby = 0x7,
}

#[repr(u32)]
pub enum ProtocolState {
    // Nothing is happening
    Inactive = 0x0,
    AwaitingWorldData = 0x1,
    Unk2 = 0x2,
    Unk3 = 0x3,
    Unk4 = 0x4,
    Unk5 = 0x5,
    InWorld = 0x6,
    Unk7 = 0x7,
}

#[repr(C)]
#[derive(FromZeroes, FromBytes)]
pub struct CSSessionManager {
    pub vftable: usize,
    pub unk0x8: u32,
    pub world_state: u32,
    pub protocol_state: u32,
}

impl DLRFLocatable for CSSessionManager {
    const DLRF_NAME: &'static str = "CSSessionManager";
}
