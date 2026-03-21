mod app;
mod menu_bar;
mod sidebar;
mod tabs;
mod theme;

fn main() -> iced::Result {
    iced::application(app::App::boot, app::App::update, app::App::view)
        .subscription(app::App::subscription)
        .theme(app::App::theme)
        .title("iced-fancy")
        .font(iced_aw::ICED_AW_FONT_BYTES)
        .run()
}
