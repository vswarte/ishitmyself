use std::ffi;

use zerocopy::{FromBytes, FromZeroes};

use crate::game::dl::DLWString;

#[repr(C)]
#[derive(FromZeroes, FromBytes)]
pub struct FD4BasicHashString {
    pub vftable: usize,
    pub allocator: usize,
    pub string: DLWString,
    pub unk1: usize,
    pub hash: u32,
    pub needs_hashing: u8,
    pub pad: [u8; 7],
}
