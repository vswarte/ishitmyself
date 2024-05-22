use hudhook::imgui::{TreeNodeFlags, Ui};

use crate::game::cs::{CSCam, CSSessionManager, ChrIns, ChrSet, OpenFieldChrSet, WorldChrMan};
use crate::game::fd4::FlverRepository;
use crate::util;
use crate::game::{cs::CSCamera, world_area_time::WorldAreaTime};

use super::singleton::DLRFLocatable;

pub trait DebugDisplay {
    fn render_debug(&self, ui: &&mut Ui);
}

impl DebugDisplay for CSCamera<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Pers cam 1", TreeNodeFlags::empty()) {
            self.pers_cam_1.render_debug(ui);
        }

        if ui.collapsing_header("Pers cam 2", TreeNodeFlags::empty()) {
            self.pers_cam_2.render_debug(ui);
        }

        if ui.collapsing_header("Pers cam 3", TreeNodeFlags::empty()) {
            self.pers_cam_3.render_debug(ui);
        }

        if ui.collapsing_header("Pers cam 4", TreeNodeFlags::empty()) {
            self.pers_cam_4.render_debug(ui);
        }

        ui.text(format!("Unk28: {}", self.unk28));
        ui.text(format!("Unk30: {}", self.unk30));
    }
}

impl DebugDisplay for CSCam {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("unk8: {}", self.unk8));
        ui.text(format!("unkc: {}", self.unkc));
        ui.text(format!("Aspect ratio: {}", self.aspect_ratio));
        ui.text(format!("Far plane: {}", self.far_plane));
        ui.text(format!("Near plane: {}", self.near_plane));
    }
}

impl DebugDisplay for WorldAreaTime {
    fn render_debug(&self, ui: &&mut Ui) {
        let year = self.clock.year();
        ui.text(format!("Year: {year}"));

        let month = self.clock.month();
        ui.text(format!("Month: {month}"));

        let day_of_week = self.clock.day_of_week();
        ui.text(format!("Day of week: {day_of_week}"));

        let day = self.clock.day();
        ui.text(format!("Day: {day}"));

        let hours = self.clock.hours();
        ui.text(format!("Hours: {hours}"));

        let minutes = self.clock.minutes();
        ui.text(format!("Minutes: {minutes}"));

        let seconds = self.clock.seconds();
        ui.text(format!("Seconds: {seconds}"));
    }
}

impl DebugDisplay for CSSessionManager {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("World state: {}", self.world_state));
        ui.text(format!("Protocol state: {}", self.protocol_state));
    }
}

impl DebugDisplay for WorldChrMan<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        let world_area_chr_list_count = self.world_area_chr_list_count;
        let world_block_chr_list_count = self.world_block_chr_list_count;
        let world_grid_area_chr_list_count = self.world_grid_area_chr_list_count;
        let world_area_list_count = self.world_area_list_count;

        ui.text(format!("World Area Chr List Count: {world_area_list_count}"));
        ui.text(format!("World Block Chr List Count: {world_block_chr_list_count}"));
        ui.text(format!("World Grid Area Chr List Count: {world_grid_area_chr_list_count}"));
        ui.text(format!("World Area List Count: {world_area_list_count}"));

        if ui.collapsing_header("ChrSet 1", TreeNodeFlags::empty()) {
            self.chr_set_1.render_debug(ui);
        }

        if ui.collapsing_header("ChrSet 2", TreeNodeFlags::empty()) {
            self.chr_set_2.render_debug(ui);
        }

        if ui.collapsing_header("ChrSet 3", TreeNodeFlags::empty()) {
            self.chr_set_3.render_debug(ui);
        }

        if ui.collapsing_header("ChrSet 4", TreeNodeFlags::empty()) {
            self.chr_set_4.render_debug(ui);
        }

        if ui.collapsing_header("OpenFieldChrSet", TreeNodeFlags::empty()) {
            self.open_field_chr_set.render_debug(ui);
        }
    }
}

impl DebugDisplay for ChrSet<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Character count: {}", self.count));
        ui.text(format!("Character capacity: {}", self.capacity));

        // for element in self.character_iter() {
        //     let chr_ins = unsafe {(element.chr_ins).as_ref().unwrap()};
        //
        //     let label = chr_ins.field_ins_handle.to_string();
        //     if ui.collapsing_header(label, TreeNodeFlags::empty()) {
        //         chr_ins.render_debug(ui);
        //     }
        // }
    }
}

impl DebugDisplay for ChrIns<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Map ID 1: {}", self.map_id_1));
        ui.text(format!("Map ID origin 1: {}", self.map_id_origin_1));
        ui.text(format!("Map ID 2: {}", self.map_id_2));
        ui.text(format!("Map ID origin 2: {}", self.map_id_origin_2));
    }
}


impl DebugDisplay for OpenFieldChrSet<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        self.base.render_debug(ui)
    }
}

impl DebugDisplay for FlverRepository<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Capacity: {}", self.map.capacity));

        for rescap in self.map.iter() {
            ui.text(rescap.header.name.string.to_string());
        }
    }
}

pub fn render_debug_singleton<T: DLRFLocatable + DebugDisplay + 'static>(ui: &&mut Ui) {
    let singleton = util::singleton::get_instance::<T>()
        .expect(&format!("Could not get reflection data for {}", T::DLRF_NAME));

    match singleton {
        Some(instance) => if ui.collapsing_header(T::DLRF_NAME, TreeNodeFlags::empty()) {
            instance.render_debug(&ui);
            ui.separator();
        },
        None => ui.text(format!("No instance of {} found", T::DLRF_NAME)),
    }
}
