use iced::widget::{button, column, container, pick_list, row, slider, text, toggler};
use iced::{Color, Element, Fill, Length};

use super::channel::{Channel, WaveType};
use super::Message;

/// Build the control strip for a single channel.
pub fn channel_strip(index: usize, channel: &Channel) -> Element<'_, Message> {
    let header = row![
        text(&channel.name).size(16),
        toggler(channel.enabled)
            .label(if channel.enabled { "On" } else { "Off" })
            .on_toggle(move |v| Message::SetEnabled(index, v))
            .size(16),
    ]
    .spacing(12)
    .align_y(iced::Alignment::Center);

    let wave_pick = row![
        text("Wave:").size(12),
        pick_list(
            WaveType::ALL,
            Some(channel.wave_type),
            move |w| Message::SetWaveType(index, w),
        )
        .text_size(12),
    ]
    .spacing(6)
    .align_y(iced::Alignment::Center);

    let volume = labeled_slider(
        "Volume",
        channel.volume,
        0.0..=1.0,
        0.01,
        move |v| Message::SetVolume(index, v),
    );

    let frequency = labeled_slider(
        "Freq",
        channel.frequency,
        0.5..=10.0,
        0.1,
        move |v| Message::SetFrequency(index, v),
    );

    let amplitude = labeled_slider(
        "Amp",
        channel.amplitude,
        0.0..=1.0,
        0.01,
        move |v| Message::SetAmplitude(index, v),
    );

    let eq_bass = labeled_slider_signed(
        "Bass",
        channel.eq_bass,
        -1.0..=1.0,
        0.05,
        move |v| Message::SetEqBass(index, v),
    );

    let eq_treble = labeled_slider_signed(
        "Treble",
        channel.eq_treble,
        -1.0..=1.0,
        0.05,
        move |v| Message::SetEqTreble(index, v),
    );

    let color_btn = button(
        container(text("").size(12))
            .padding([4, 12])
            .style(move |_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Background::Color(channel.color)),
                    border: iced::Border {
                        radius: 4.0.into(),
                        width: 1.0,
                        color: Color::from_rgba(1.0, 1.0, 1.0, 0.3),
                    },
                    ..Default::default()
                }
            }),
    )
    .on_press(Message::ToggleColorPicker(index))
    .padding(0);

    let color_row = row![
        text("Color:").size(12),
        color_btn,
    ]
    .spacing(6)
    .align_y(iced::Alignment::Center);

    let mut col = column![
        header,
        wave_pick,
        volume,
        frequency,
        amplitude,
        eq_bass,
        eq_treble,
        color_row,
    ]
    .spacing(6)
    .padding(10)
    .width(Fill);

    if channel.show_color_picker {
        let picker = iced_aw::ColorPicker::new(
            true,
            channel.color,
            button(text("Pick Color").size(12)).on_press(Message::ToggleColorPicker(index)),
            Message::CancelColorPicker(index),
            move |c| Message::SetColor(index, c),
        );
        col = col.push(picker);
    }

    container(col)
        .style(container::bordered_box)
        .width(Fill)
        .into()
}

fn labeled_slider<'a>(
    label: &str,
    value: f32,
    range: std::ops::RangeInclusive<f32>,
    step: f32,
    on_change: impl Fn(f32) -> Message + 'a,
) -> Element<'a, Message> {
    row![
        text(format!("{}: {:.2}", label, value)).size(12).width(Length::Fixed(90.0)),
        slider(range, value, on_change).step(step),
    ]
    .spacing(6)
    .align_y(iced::Alignment::Center)
    .into()
}

fn labeled_slider_signed<'a>(
    label: &str,
    value: f32,
    range: std::ops::RangeInclusive<f32>,
    step: f32,
    on_change: impl Fn(f32) -> Message + 'a,
) -> Element<'a, Message> {
    let sign = if value >= 0.0 { "+" } else { "" };
    row![
        text(format!("{}: {}{:.2}", label, sign, value)).size(12).width(Length::Fixed(90.0)),
        slider(range, value, on_change).step(step),
    ]
    .spacing(6)
    .align_y(iced::Alignment::Center)
    .into()
}
