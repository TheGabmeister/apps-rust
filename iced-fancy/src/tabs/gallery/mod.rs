mod buttons;
mod sliders;
mod text_inputs;
mod togglers;
mod checkboxes;
mod radios;
mod pick_lists;
mod progress_bars;
mod tooltips;
mod cards;
mod badges;
mod color_picker;
mod date_picker;
mod time_picker;
mod number_input;
mod spinner;
mod context_menu;

use iced::widget::{column, container, responsive, row, rule, scrollable, text, Space};
use iced::{Color, Element, Fill};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadioChoice {
    A,
    B,
    C,
}

#[derive(Debug)]
pub struct State {
    pub click_count: u32,
    pub slider_value: f32,
    pub text_value: String,
    pub password_value: String,
    pub toggler_value: bool,
    pub check1: bool,
    pub check2: bool,
    pub check3: bool,
    pub radio_selected: Option<RadioChoice>,
    pub pick_selected: Option<String>,
    pub progress_value: f32,
    pub card_closed: bool,
    pub show_color_picker: bool,
    pub color: Color,
    pub show_date_picker: bool,
    pub date: iced_aw::date_picker::Date,
    pub date_label: String,
    pub show_time_picker: bool,
    pub time: iced_aw::time_picker::Time,
    pub time_label: String,
    pub number_value: f32,
    pub context_action: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            click_count: 0,
            slider_value: 50.0,
            text_value: String::new(),
            password_value: String::new(),
            toggler_value: false,
            check1: false,
            check2: true,
            check3: false,
            radio_selected: Some(RadioChoice::B),
            pick_selected: None,
            progress_value: 45.0,
            card_closed: false,
            show_color_picker: false,
            color: Color::from_rgb(0.2, 0.6, 1.0),
            show_date_picker: false,
            date: iced_aw::date_picker::Date::default(),
            date_label: "No date selected".into(),
            show_time_picker: false,
            time: iced_aw::time_picker::Time::default(),
            time_label: "No time selected".into(),
            number_value: 42.0,
            context_action: "None".into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonClicked,
    SliderChanged(f32),
    TextChanged(String),
    PasswordChanged(String),
    TogglerToggled(bool),
    Check1Toggled(bool),
    Check2Toggled(bool),
    Check3Toggled(bool),
    RadioSelected(RadioChoice),
    PickSelected(String),
    ProgressChanged(f32),
    CardClosed,
    CardReopen,
    ToggleColorPicker,
    ColorSubmit(Color),
    ColorCancel,
    ToggleDatePicker,
    DateSubmit(iced_aw::date_picker::Date),
    DateCancel,
    ToggleTimePicker,
    TimeSubmit(iced_aw::time_picker::Time),
    TimeCancel,
    NumberChanged(f32),
    ContextAction(String),
}

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::ButtonClicked => state.click_count += 1,
        Message::SliderChanged(v) => state.slider_value = v,
        Message::TextChanged(v) => state.text_value = v,
        Message::PasswordChanged(v) => state.password_value = v,
        Message::TogglerToggled(v) => state.toggler_value = v,
        Message::Check1Toggled(v) => state.check1 = v,
        Message::Check2Toggled(v) => state.check2 = v,
        Message::Check3Toggled(v) => state.check3 = v,
        Message::RadioSelected(v) => state.radio_selected = Some(v),
        Message::PickSelected(v) => state.pick_selected = Some(v),
        Message::ProgressChanged(v) => state.progress_value = v,
        Message::CardClosed => state.card_closed = true,
        Message::CardReopen => state.card_closed = false,
        Message::ToggleColorPicker => state.show_color_picker = !state.show_color_picker,
        Message::ColorSubmit(c) => {
            state.color = c;
            state.show_color_picker = false;
        }
        Message::ColorCancel => state.show_color_picker = false,
        Message::ToggleDatePicker => state.show_date_picker = !state.show_date_picker,
        Message::DateSubmit(d) => {
            state.date_label = format!("{:?}", d);
            state.date = d;
            state.show_date_picker = false;
        }
        Message::DateCancel => state.show_date_picker = false,
        Message::ToggleTimePicker => state.show_time_picker = !state.show_time_picker,
        Message::TimeSubmit(t) => {
            state.time_label = format!("{:?}", t);
            state.time = t;
            state.show_time_picker = false;
        }
        Message::TimeCancel => state.show_time_picker = false,
        Message::NumberChanged(v) => state.number_value = v,
        Message::ContextAction(a) => state.context_action = a,
    }
}

pub fn view(state: &State) -> Element<'_, Message> {
    responsive(|size| {
        let cols = if size.width < 500.0 {
            1
        } else if size.width < 800.0 {
            2
        } else if size.width < 1100.0 {
            3
        } else {
            4
        };

        let cards: Vec<Element<Message>> = vec![
            buttons::view(state),
            sliders::view(state),
            text_inputs::view(state),
            togglers::view(state),
            checkboxes::view(state),
            radios::view(state),
            pick_lists::view(state),
            progress_bars::view(state),
            tooltips::view(state),
            cards::view(state),
            badges::view(state),
            color_picker::view(state),
            date_picker::view(state),
            time_picker::view(state),
            number_input::view(state),
            spinner::view(state),
            context_menu::view(state),
        ];

        let mut grid = column![].spacing(12);
        let mut cards_iter = cards.into_iter();

        loop {
            let mut current_row = row![].spacing(12);
            let mut count = 0;
            for _ in 0..cols {
                if let Some(card) = cards_iter.next() {
                    current_row = current_row.push(container(card).width(Fill));
                    count += 1;
                }
            }
            if count == 0 {
                break;
            }
            for _ in count..cols {
                current_row = current_row.push(Space::new().width(Fill));
            }
            grid = grid.push(current_row);
        }

        scrollable(container(grid).padding(16).width(Fill))
            .height(Fill)
            .into()
    })
    .into()
}

pub fn demo_card<'a>(
    title: &str,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(
        column![
            text(title.to_string()).size(16),
            rule::horizontal(1),
            content.into(),
        ]
        .spacing(8)
        .padding(12),
    )
    .style(container::bordered_box)
    .width(Fill)
    .into()
}
