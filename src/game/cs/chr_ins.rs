use std::ffi;

use zerocopy::{FromBytes, FromZeroes};

use crate::game::cs::ChrSetEntry;

#[repr(C)]
#[derive(FromBytes, FromZeroes)]
pub struct ChrIns<'a> {
    pub vftable: usize,
    pub field_ins_handle: u32,
    pub padc: u32,
    chr_set_entry: usize,
    pub unk18: usize,
    pub unk20: u32,
    pub unk24: u32,
    pub chr_res: usize,
    pub map_id_1: u32,
    pub map_id_origin_1: u32,
    pub map_id_2: u32,
    pub map_id_origin_2: u32,
    pub unk40: u32,
    pub unk44: u32,
    pub unk48: usize,
    pub chr_model: usize,
    pub chr_ctrl: &'a mut ChrCtrl<'a>,
}

#[repr(C)]
pub struct ChrCtrl<'a> {
    pub vftable: usize,
    unk8: u64,
    pub owner: &'a ChrIns<'a>,
    pub manipulator: usize,
    unk20: usize,
    ragdoll_ins: usize,
    chr_collision: usize,
    unk38: [u8; 240],
    pub chr_ragdoll_state: u8,
}
