use std::ffi;
use zerocopy::{FromBytes, FromZeroes};

use crate::game::fd4::FD4BasicHashString;
use crate::util::singleton::DLRFLocatable;

#[repr(C)]
#[derive(FromZeroes, FromBytes)]
pub struct CSTaskGroup<'a> {
    pub vftable: usize,
    pub task_groups: [&'a CSTimeLineTaskGroupIns; 169],
}

impl DLRFLocatable for CSTaskGroup<'_> {
    const DLRF_NAME: &'static str = "CSTaskGroup";
}

#[repr(C)]
#[derive(FromZeroes, FromBytes)]
pub struct CSTaskGroupIns {
    pub vftable: usize,
    pub name: FD4BasicHashString,
    unk48: [u8; 0x10],
}

#[repr(C)]
#[derive(FromZeroes, FromBytes)]
pub struct CSTimeLineTaskGroupIns {
    pub base: CSTaskGroupIns,
    pub step_impl: usize,
    unk60: [u8; 0x20],
}
