use broadsword::dll;

use game::cs::CSCamera;
use game::cs::CSSessionManager;
use game::cs::WorldState;
use game::cs::{ChrIns, WorldChrMan, WorldChrManDbg};
use game::fd4::FlverRepository;
use game::world_area_time::WorldAreaTime;
use hudhook::eject;
use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::imgui;
use hudhook::imgui::*;
use hudhook::windows::Win32::Foundation::HINSTANCE;
use hudhook::Hudhook;
use hudhook::ImguiRenderLoop;
use util::debug_display::render_debug_singleton;
use util::debug_display::DebugDisplay;
use util::singleton::DLRFLocatable;

#[cfg(test)]
pub mod test;

mod game;
mod util;
mod export;

use std::ffi;
use std::mem;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::sync::OnceLock;
use std::sync::RwLock;
use std::time::Instant;
use windows::core::PCWSTR;
use crate::game::cs::CSFile;
use crate::game::dl::DLWString;

#[dll::entrypoint]
pub fn entry(hmodule: usize) -> bool {
    std::thread::spawn(move || {
        if let Err(e) = Hudhook::builder()
            .with::<ImguiDx12Hooks>(FsTestsHud::new())
            .with_hmodule(HINSTANCE(hmodule as isize))
            .build()
            .apply()
        {
            tracing::error!("Couldn't apply hooks: {e:?}");
            eject();
        }
    });

    // std::thread::spawn(|| {
    //     std::thread::sleep(std::time::Duration::from_secs(30));
    //
    //     let cs_file = unsafe { &**(0x143cd1f48 as **const CSFile) };
    //     log::info!("CSFile acquired! {:?}", cs_file);
    //
    //     let file_repository = unsafe { &*cs_file.file_repository_1 };
    //     log::info!("First repository acquired! {:?}", file_repository);
    //
    //     let holder = &file_repository.holder1;
    //     log::info!("Holder acquired: {:?}", holder);
    //
    //     std::thread::sleep(std::time::Duration::from_secs(5));
    //     holder.iter()
    //         .for_each(|file| log::info!("File cap: {}", file.header.name.string.to_string()))
    // });

    true
}

struct FsTestsHud;

impl FsTestsHud {
    fn new() -> Self {
        Self {}
    }
}

const LOG: OnceLock<RwLock<Vec<String>>> = OnceLock::new();
const LAST_WORLD_STATE: AtomicU32 = AtomicU32::new(0);

impl ImguiRenderLoop for FsTestsHud {
    fn render(&mut self, ui: &mut Ui) {
        ui.window("Elden Ring Debug")
            .position([0., 0.], imgui::Condition::FirstUseEver)
            .size([800., 600.], imgui::Condition::FirstUseEver)
            .build(|| {
                render_debug_singleton::<WorldChrMan>(&ui);
                render_debug_singleton::<CSSessionManager>(&ui);
                render_debug_singleton::<WorldAreaTime>(&ui);
                render_debug_singleton::<CSCamera>(&ui);
                render_debug_singleton::<FlverRepository>(&ui);
            });
    }
}

// unsafe fn update_spectate() {
//     static mut SPECTATE_ENABLED: bool = false;
//     static mut SPECTATE_SLOT: isize = 0;
//
//     let world_chr_man_dbg =
//         util::singleton::get_instance::<WorldChrManDbg>()
//             .expect("Could not find static for WorldChrManDbg");
//
//     let world_chr_man_dbg = match world_chr_man_dbg {
//         Some(w) => &mut *w,
//         None => return,
//     };
//
//     if input::is_key_pressed(0x24) {
//         SPECTATE_ENABLED = !SPECTATE_ENABLED;
//
//         if !SPECTATE_ENABLED {
//             world_chr_man_dbg.cam_override_chr_ins = 0 as *mut ChrIns;  
//         } else {
//             SPECTATE_SLOT = 0;
//         }
//     }
//
//     if SPECTATE_ENABLED {
//         let world_chr_man = 
//             util::singleton::get_instance::<WorldChrMan>()
//                 .expect("Could not find static for WorldChrMan");
//
//         let world_chr_man = match world_chr_man {
//             Some(w) => &mut *w,
//             None => return,
//         };
//
//         let chrs = world_chr_man.open_field_chr_set.base.iter()
//             .collect::<Vec<*mut ChrIns>>();
//
//         let change: isize = if input::is_key_pressed(0x21) {
//             -1
//         } else if input::is_key_pressed(0x22) {
//             1
//         } else {
//             0
//         };
//
//         SPECTATE_SLOT += change;
//         if SPECTATE_SLOT < 0 {
//             SPECTATE_SLOT = (chrs.len() - 1) as isize;
//         } else if SPECTATE_SLOT > chrs.len() as isize - 1 {
//             SPECTATE_SLOT = 0;
//         }
//
//         let selected_chr = chrs[SPECTATE_SLOT as usize];
//         if change != 0 {
//             world_chr_man_dbg.cam_override_chr_ins = selected_chr;
//         }
//
//         if input::is_key_pressed(0x2d) {
//             let state = (*(*selected_chr).chr_ctrl).chr_ragdoll_state;
//
//             (*(*selected_chr).chr_ctrl).chr_ragdoll_state = if state == 0 {
//                 2
//             } else {
//                 0
//             };
//         }
//     }
// }
