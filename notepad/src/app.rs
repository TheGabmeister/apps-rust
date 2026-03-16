use eframe::egui;

use crate::editor::EditorState;
use crate::file_ops;
use crate::ui::editor_view;
use crate::ui::menu_bar::{self, MenuAction};
use crate::ui::status_bar;

enum PendingAction {
    None,
    New,
    Open,
    Exit,
}

pub struct NotepadApp {
    editor: EditorState,
    pending_action: PendingAction,
    show_unsaved_dialog: bool,
}

impl NotepadApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            editor: EditorState::new(),
            pending_action: PendingAction::None,
            show_unsaved_dialog: false,
        }
    }

    fn handle_action(&mut self, action: MenuAction, ctx: &egui::Context) {
        match action {
            MenuAction::New => {
                if self.editor.dirty {
                    self.pending_action = PendingAction::New;
                    self.show_unsaved_dialog = true;
                } else {
                    self.do_new();
                }
            }
            MenuAction::Open => {
                if self.editor.dirty {
                    self.pending_action = PendingAction::Open;
                    self.show_unsaved_dialog = true;
                } else {
                    self.do_open();
                }
            }
            MenuAction::Save => self.do_save(),
            MenuAction::SaveAs => self.do_save_as(),
            MenuAction::Exit => {
                if self.editor.dirty {
                    self.pending_action = PendingAction::Exit;
                    self.show_unsaved_dialog = true;
                } else {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        }
    }

    fn do_new(&mut self) {
        self.editor.reset();
    }

    fn do_open(&mut self) {
        if let Some((path, content)) = file_ops::open_file_dialog() {
            self.editor.load(path, content);
        }
    }

    fn do_save(&mut self) {
        if let Some(path) = self.editor.file_path.clone() {
            match file_ops::save_file(&path, &self.editor.content) {
                Ok(()) => self.editor.mark_saved(None),
                Err(e) => eprintln!("{}", e),
            }
        } else {
            self.do_save_as();
        }
    }

    fn do_save_as(&mut self) {
        if let Some(path) = file_ops::save_file_as_dialog(&self.editor.content) {
            self.editor.mark_saved(Some(path));
        }
    }

    fn render_unsaved_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_unsaved_dialog {
            return;
        }

        egui::Window::new("Unsaved Changes")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("You have unsaved changes. Do you want to save before continuing?");
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        self.do_save();
                        self.execute_pending_action(ctx);
                        self.show_unsaved_dialog = false;
                    }
                    if ui.button("Don't Save").clicked() {
                        self.execute_pending_action(ctx);
                        self.show_unsaved_dialog = false;
                    }
                    if ui.button("Cancel").clicked() {
                        self.pending_action = PendingAction::None;
                        self.show_unsaved_dialog = false;
                    }
                });
            });
    }

    fn execute_pending_action(&mut self, ctx: &egui::Context) {
        match std::mem::replace(&mut self.pending_action, PendingAction::None) {
            PendingAction::New => self.do_new(),
            PendingAction::Open => self.do_open(),
            PendingAction::Exit => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            PendingAction::None => {}
        }
    }
}

impl eframe::App for NotepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Intercept OS window close button
        if ctx.input(|i| i.viewport().close_requested()) && self.editor.dirty {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.pending_action = PendingAction::Exit;
            self.show_unsaved_dialog = true;
        }

        ctx.send_viewport_cmd(egui::ViewportCommand::Title(self.editor.window_title()));

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            if let Some(action) = menu_bar::render_menu_bar(ui, ctx) {
                self.handle_action(action, ctx);
            }
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            status_bar::render_status_bar(ui, &self.editor);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let changed = editor_view::render_editor(ui, &mut self.editor);
            if changed {
                self.editor.dirty = true;
            }
        });

        self.render_unsaved_dialog(ctx);
    }
}
