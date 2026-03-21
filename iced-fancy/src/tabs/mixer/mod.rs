use iced::widget::{center, text};
use iced::Element;

#[derive(Debug, Default)]
pub struct State;

#[derive(Debug, Clone)]
pub enum Message {}

pub fn update(_state: &mut State, _message: Message) {}

pub fn view(_state: &State) -> Element<'_, Message> {
    center(text("Audio Mixer").size(24)).into()
}
