use zerocopy::{FromBytes, FromZeroes};

use crate::util::singleton::DLRFLocatable;

#[repr(C)]
#[derive(FromBytes, FromZeroes)]
pub struct WorldAreaTime {
    pub unk0: u64,
    pub clock: WorldAreaTimeClock,
    // TODO: rest
}

impl DLRFLocatable for WorldAreaTime {
    const DLRF_NAME: &'static str = "WorldAreaTime";
}

#[derive(FromBytes, FromZeroes)]
pub struct WorldAreaTimeClock(u64);

impl WorldAreaTimeClock {
    pub fn year(&self) -> u64 {
        (self.0 >> 00) & 0b111111111111
    }

    pub fn milliseconds(&self) -> u64 {
        (self.0 >> 12) & 0b1111111111
    }

    pub fn month(&self) -> u64 {
        (self.0 >> 22) & 0b1111
    }

    pub fn day_of_week(&self) -> u64 {
        (self.0 >> 26) & 0b111
    }

    pub fn day(&self) -> u64 {
        (self.0 >> 29) & 0b11111
    }

    pub fn hours(&self) -> u64 {
        (self.0 >> 34) & 0b11111
    }

    pub fn minutes(&self) -> u64 {
        (self.0 >> 39) & 0b111111
    }

    pub fn seconds(&self) -> u64 {
        (self.0 >> 45) & 0b111111
    }
}
