use core::ffi;

use zerocopy::{FromBytes, FromZeroes};

use crate::game::fd4::FD4BasicHashString;
use crate::util::singleton::DLRFLocatable;

/// Represents a managed resource.
/// The data it represents is immediately handed over to 
/// other systems and the ResCap serves as a token for unloading things.
/// One such example is gparams where the file associated with a FileCap is
/// parsed, ResCaps (multiple) are created from the FileCap, and the ResCaps
/// individually post the data they represent to associated sub-systems.
/// For GParamResCaps that means posting the such data to the gparam blending
/// system as well as a bunch of other GX structures
#[repr(C)]
#[derive(FromBytes, FromZeroes)]
pub struct FD4ResCap<'a, TRes> {
    pub header: FD4ResCapHeader<'a, TRes>,
    pub data: TRes,
}

#[repr(C)]
#[derive(FromBytes, FromZeroes)]
pub struct FD4ResCapHeader<'a, TRes> {
    pub vftable: usize,
    pub name: FD4BasicHashString,
    pub owning_repository: &'a FD4ResCapHolder<'a, TRes>,
    pub next_item: *const FD4ResCap<'a, TRes>,
    pub reference_count: u32,
    pad5c: [u8; 4], // TODO: Actually contains two bools
    pub debug_menu_item: usize,
}

/// Represents a collection of ResCaps/FileCaps.
/// The game relies heavily on traditional hashmaps for asset management.
/// The resources name gets turned in a u32 using some FNV variant. That hash
/// is then modulo'd by the repository's capacity to find the appropriate slot 
/// in the map's first layer.
/// In the case of collision on lookups it will start cycling through the 
/// linked list for the matched slot and compare the full resource name hashes.
///
/// This fnv hashing itself is actually facilitated by FD4BasicHashString.
/// In the case of a collision on insertion it will make the entry you are 
/// seeking to insert the new head.
/// 
/// Slot# = fnv(resource name) % holder capacity
///
/// ```
/// +----------------------------------------------------------------------....
/// |                        FD4ResCapHolder<R,T>'s map                    
/// +-------------------------------------------------------+--------------....
/// |  Slot 0          |  Slot 1          |  Slot 2         |  Slot 4
/// +------------------+------------------+-----------------+--------------....
/// |  FD4ResCap<T>    |  FD4ResCap<T>    |                 |  FD4ResCap<T>
/// |  FD4ResCap<T>    |                  |                 |  FD4ResCap<T>
/// |  FD4ResCap<T>    |                  |                 |
/// |                  |                  |                 |
/// |                  |                  |                 |
/// +------------------+------------------+-----------------+--------------....
/// ```
#[repr(C)]
#[derive(FromBytes, FromZeroes)]
pub struct FD4ResCapHolder<'a, TRes> {
    pub vftable: usize,
    pub allocator: usize,
    pub owning_repository: usize,
    pub unk18: u32,
    pub capacity: u32,
    pub map: &'a FD4ResCap<'a, TRes>,
}

// TODO: implement an actual hashmap type for this
impl<TRes> FD4ResCapHolder<'_, TRes> {
    pub fn iter(&self) -> ResIterator<TRes> {
        ResIterator {
            current_index: 0,
            last_item: None,
            repository: self,
        }
    }
}

pub struct ResIterator<'a, TRes> {
    // Will have to store lock for mutex in here such that we have guaranteed
    // exclusivity to the hashmap for as long as the iterator exists.
    current_index: u32,
    last_item: Option<&'a FD4ResCap<'a, TRes>>,
    repository: &'a FD4ResCapHolder<'a, TRes>,
}

impl<'a, T> Iterator for ResIterator<'a, T> {
    type Item = &'a FD4ResCap<'a, T>;

    // TODO: wtf is this supposed to be lmao
    fn next(&mut self) -> Option<Self::Item> {
        // // Continue in the linked list if we're in any
        // if let Some(last) = self.last_item {
        //     if let Some(item) = unsafe { last.header.next_item.as_ref() } {
        //         self.last_item = Some(item);
        //         return Some(item);
        //     }
        // }
        //
        // // Search for the next occupied slot and start from the head
        // for i in self.current_index..self.repository.capacity {
        //     self.current_index += 1;
        //
        //     let slot_ptr = unsafe {
        //         let slot_offset = 8 * i as usize;
        //         *((self.repository.map as *const FD4ResCap<'a, _> as usize + slot_offset) as *const usize)
        //     };
        //
        //     if slot_ptr != 0x0 {
        //         let res_cap = unsafe {
        //             std::mem::transmute::<usize, Self::Item>(slot_ptr)
        //         };
        //         self.last_item = Some(res_cap);
        //         return Some(res_cap);
        //     }
        // }

        None
    }
}

#[repr(C)]
pub struct FfxRepositoryImp<'a> {
    pub repository_res_cap: FD4ResCap<'a, [u8; 0x10]>,
    pub map: FD4ResCapHolder<'a, ()>,
}

impl DLRFLocatable for FfxRepositoryImp<'_> {
    const DLRF_NAME: &'static str = "FfxRepository";
}

#[repr(C)]
pub struct FlverRepository<'a> {
    pub repository_res_cap: FD4ResCap<'a, [u8; 0x8]>,
    pub map: FD4ResCapHolder<'a, ()>,
}

impl DLRFLocatable for FlverRepository<'_> {
    const DLRF_NAME: &'static str = "FlverRepository";
}
