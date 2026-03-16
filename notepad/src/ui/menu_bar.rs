use eframe::egui::{self, Context, Key, KeyboardShortcut, Modifiers, Ui};

pub enum MenuAction {
    New,
    Open,
    Save,
    SaveAs,
    Exit,
}

const SHORTCUT_NEW: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::N);
const SHORTCUT_OPEN: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::O);
const SHORTCUT_SAVE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::S);
const SHORTCUT_SAVE_AS: KeyboardShortcut = KeyboardShortcut::new(
    Modifiers {
        alt: false,
        ctrl: true,
        shift: true,
        mac_cmd: false,
        command: false,
    },
    Key::S,
);

pub fn render_menu_bar(ui: &mut Ui, ctx: &Context) -> Option<MenuAction> {
    let mut action = None;

    ctx.input_mut(|input| {
        if input.consume_shortcut(&SHORTCUT_NEW) {
            action = Some(MenuAction::New);
        } else if input.consume_shortcut(&SHORTCUT_OPEN) {
            action = Some(MenuAction::Open);
        } else if input.consume_shortcut(&SHORTCUT_SAVE_AS) {
            action = Some(MenuAction::SaveAs);
        } else if input.consume_shortcut(&SHORTCUT_SAVE) {
            action = Some(MenuAction::Save);
        }
    });

    egui::MenuBar::new().ui(ui, |ui: &mut Ui| {
        ui.menu_button("File", |ui| {
            if ui
                .add(
                    egui::Button::new("New")
                        .shortcut_text(ctx.format_shortcut(&SHORTCUT_NEW)),
                )
                .clicked()
            {
                action = Some(MenuAction::New);
                ui.close();
            }
            if ui
                .add(
                    egui::Button::new("Open...")
                        .shortcut_text(ctx.format_shortcut(&SHORTCUT_OPEN)),
                )
                .clicked()
            {
                action = Some(MenuAction::Open);
                ui.close();
            }
            ui.separator();
            if ui
                .add(
                    egui::Button::new("Save")
                        .shortcut_text(ctx.format_shortcut(&SHORTCUT_SAVE)),
                )
                .clicked()
            {
                action = Some(MenuAction::Save);
                ui.close();
            }
            if ui
                .add(
                    egui::Button::new("Save As...")
                        .shortcut_text(ctx.format_shortcut(&SHORTCUT_SAVE_AS)),
                )
                .clicked()
            {
                action = Some(MenuAction::SaveAs);
                ui.close();
            }
            ui.separator();
            if ui.button("Exit").clicked() {
                action = Some(MenuAction::Exit);
                ui.close();
            }
        });
    });

    action
}
