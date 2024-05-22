use zerocopy::{FromBytes, FromZeroes};

use crate::game::matrix::Matrix4X4;
use crate::util::singleton::DLRFLocatable;

#[repr(C)]
#[derive(FromBytes, FromZeroes)]
pub struct CSCamera<'a> {
    pub vftable: usize,
    pub pers_cam_1: &'a mut CSPersCam,
    pub pers_cam_2: &'a mut CSPersCam,
    pub pers_cam_3: &'a mut CSPersCam,
    pub pers_cam_4: &'a mut CSPersCam,
    pub unk28: usize,
    pub unk30: usize,
}

impl DLRFLocatable for CSCamera<'_> {
    const DLRF_NAME: &'static str = "CSCamera";
}

#[repr(C)]
#[derive(FromBytes, FromZeroes)]
pub struct CSCam {
    pub vftable: usize,
    pub unk8: u32,
    pub unkc: u32,
    pub view_matrix: Matrix4X4,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

pub type CSPersCam = CSCam;
