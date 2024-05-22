use std::ffi;

use windows::Win32::System::Threading::CRITICAL_SECTION;
use zerocopy::{FromBytes, FromZeroes};

#[repr(C)]
#[derive(FromBytes, FromZeroes)]
pub struct DLPlainLightMutex {
    pub vftable: usize,
    pub critical_section: [u8; 0x28],
}
