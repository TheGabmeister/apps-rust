use std::time::{Duration, Instant};

use iced::{Element, Subscription, Task, Theme};

use crate::sidebar::TabId;
use crate::tabs;

#[derive(Debug)]
pub struct App {
    pub active_tab: TabId,
    pub is_dark: bool,
    pub gallery: tabs::gallery::State,
    pub layouts: tabs::layouts::State,
    pub pane_grid: tabs::pane_grid::State,
    pub canvas: tabs::canvas::State,
    pub mixer: tabs::mixer::State,
}

impl Default for App {
    fn default() -> Self {
        Self {
            active_tab: TabId::Gallery,
            is_dark: true,
            gallery: tabs::gallery::State::default(),
            layouts: tabs::layouts::State::default(),
            pane_grid: tabs::pane_grid::State::default(),
            canvas: tabs::canvas::State::default(),
            mixer: tabs::mixer::State::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(TabId),
    ToggleTheme(bool),
    Gallery(tabs::gallery::Message),
    Layouts(tabs::layouts::Message),
    PaneGrid(tabs::pane_grid::Message),
    Canvas(tabs::canvas::Message),
    Mixer(tabs::mixer::Message),
    Tick(Instant),
}

impl App {
    pub fn boot() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TabSelected(tab) => self.active_tab = tab,
            Message::ToggleTheme(dark) => self.is_dark = dark,
            Message::Gallery(msg) => tabs::gallery::update(&mut self.gallery, msg),
            Message::Layouts(msg) => tabs::layouts::update(&mut self.layouts, msg),
            Message::PaneGrid(msg) => tabs::pane_grid::update(&mut self.pane_grid, msg),
            Message::Canvas(msg) => tabs::canvas::update(&mut self.canvas, msg),
            Message::Mixer(msg) => tabs::mixer::update(&mut self.mixer, msg),
            Message::Tick(_now) => {
                let dt = 1.0 / 60.0_f32; // ~16ms per tick
                if self.active_tab == TabId::Canvas || self.active_tab == TabId::Mixer {
                    tabs::canvas::update(&mut self.canvas, tabs::canvas::Message::Tick(dt));
                }
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let sidebar = crate::sidebar::view(self.active_tab, self.is_dark);

        let content: Element<Message> = match self.active_tab {
            TabId::Gallery => tabs::gallery::view(&self.gallery).map(Message::Gallery),
            TabId::Layouts => tabs::layouts::view(&self.layouts).map(Message::Layouts),
            TabId::PaneGrid => tabs::pane_grid::view(&self.pane_grid).map(Message::PaneGrid),
            TabId::Canvas => tabs::canvas::view(&self.canvas).map(Message::Canvas),
            TabId::Mixer => tabs::mixer::view(&self.mixer).map(Message::Mixer),
        };

        iced::widget::row![sidebar, content]
            .height(iced::Fill)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match self.active_tab {
            TabId::Canvas | TabId::Mixer => {
                iced::time::every(Duration::from_millis(16)).map(Message::Tick)
            }
            _ => Subscription::none(),
        }
    }

    pub fn theme(&self) -> Theme {
        if self.is_dark {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}
