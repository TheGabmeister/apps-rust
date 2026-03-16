use eframe::egui;

use crate::fs::entries::FileEntry;
use crate::ui::file_list::ListAction;

pub fn show_for_entry(ui: &mut egui::Ui, action: &mut Option<ListAction>, entry: &FileEntry) {
    if entry.is_dir {
        if ui.button("\u{1F4C2} Open").clicked() {
            *action = Some(ListAction::Navigate(entry.path.clone()));
            ui.close();
        }
    } else if ui.button("\u{1F4C4} Open").clicked() {
        *action = Some(ListAction::OpenFile(entry.path.clone()));
        ui.close();
    }

    ui.separator();

    if ui.button("\u{270F} Rename...").clicked() {
        *action = Some(ListAction::Rename(entry.name.clone()));
        ui.close();
    }

    if ui.button("\u{1F5D1} Delete").clicked() {
        *action = Some(ListAction::Delete(entry.name.clone(), entry.path.clone()));
        ui.close();
    }

    ui.separator();

    if ui.button("\u{1F4C1} New Folder...").clicked() {
        *action = Some(ListAction::NewFolder);
        ui.close();
    }
}

pub fn show_for_background(ui: &mut egui::Ui, action: &mut Option<ListAction>) {
    if ui.button("\u{1F4C1} New Folder...").clicked() {
        *action = Some(ListAction::NewFolder);
        ui.close();
    }
}
