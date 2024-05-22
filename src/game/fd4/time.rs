use std::ffi;

#[repr(C)]
pub struct FD4Time {
    pub vftable: *const ffi::c_void,
    pub time: f32,
    _padc: u32,
}
