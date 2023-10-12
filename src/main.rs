use std::env;

use chrono::prelude::*;
use iced::keyboard::{self, KeyCode};
use iced::widget::canvas::{Cache, Path};
use iced::widget::{canvas, Button, Canvas, Container, Row, Text};
use iced::{executor, Color, Command, Renderer, Settings, Subscription};
use iced::{subscription, Application, Event};
use iced_native::Point;
fn main() -> Result<(), iced::Error> {
    env::set_var("RUST_BACKTRACE", "full");
    App::run(Settings::default())
}

mod MainMenu;
mod style;
pub(crate) mod textStorage;
mod time;

struct MainPage {}

impl MainPage {
    fn create() {}
}

struct Graph {}

impl canvas::Program<Message> for Graph {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        theme: &iced::Theme,
        bounds: iced_native::Rectangle,
        cursor: canvas::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut  result: Vec<canvas::Geometry> = vec![];
        let g: Cache = canvas::Cache::default();
        let p = g.draw(bounds.size(), |frame| {
            let t =  Path::circle( frame.center(),10.0);
            frame.fill(&t, Color::from_rgb8(0xF9, 0xD7, 0x1C));

        });
        result.push(p);
        result
    }
}

impl Graph {
    fn new() -> Graph {
        Graph {}
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(Event),
    GoToResults,
    Tick(chrono::DateTime<Utc>),
}

enum KeyType {
    Letter(char),
    BackSpace,
    Other,
    Escape,
}

pub enum Views {
    TypingPage,
    Results,
    MainPage,
}

struct App {
    view: Views,
    text_style: style::TextStyle,
    text: textStorage::TextStorage,
    timer: time::Timer,
}

impl App {
    fn new() -> App {
        App {
            view: Views::TypingPage,
            text_style: style::TextStyle {
                typed_color: Color::from([0., 0., 0.]),
                error_color: Color::from([1., 0.2, 0.2]),
                to_type_color: Color::from([0.7, 0.7, 0.7]),
                couros_color: Color::from([0., 0., 0.]),
            },
            text: textStorage::TextStorage::new(String::from("h")),
            timer: time::Timer::new(),
        }
    }
    //TODO: add | and mayby change some var names
    fn render_text(&self) -> iced_native::widget::Row<'static, Message, Renderer> {
        let mut row = Row::new();
        // println!(
        //     "time elapsed : {}",
        //     self.timer.time_elapsed.second.to_string()
        // ); // add here some way to anable logging
        row = row.push(
            Text::new(self.timer.time_elapsed.second.to_string())
                .style(self.text_style.typed_color),
        );
        for text in &self.text.typed {
            match text {
                textStorage::TextType::Typed(text) => {
                    row = row.push(Text::new(String::from(text)).style(self.text_style.typed_color))
                }
                textStorage::TextType::Error(text) => {
                    row = row.push(Text::new(String::from(text)).style(self.text_style.error_color))
                }
            }
        }
        row = row.push(Text::new(String::from('|')).style(self.text_style.couros_color));

        row = row
            .push(Text::new(String::from(&self.text.to_type)).style(self.text_style.to_type_color));
        row
    }
    //todo add to textStorage
    fn get_key_code(key_code: keyboard::KeyCode) -> KeyType {
        match key_code {
            KeyCode::A => KeyType::Letter('a'),
            KeyCode::B => KeyType::Letter('b'),
            KeyCode::C => KeyType::Letter('c'),
            KeyCode::D => KeyType::Letter('d'),
            KeyCode::E => KeyType::Letter('e'),
            KeyCode::F => KeyType::Letter('f'),
            KeyCode::G => KeyType::Letter('g'),
            KeyCode::H => KeyType::Letter('h'),
            KeyCode::I => KeyType::Letter('i'),
            KeyCode::J => KeyType::Letter('j'),
            KeyCode::K => KeyType::Letter('k'),
            KeyCode::L => KeyType::Letter('l'),
            KeyCode::M => KeyType::Letter('m'),
            KeyCode::N => KeyType::Letter('n'),
            KeyCode::O => KeyType::Letter('o'),
            KeyCode::U => KeyType::Letter('u'),
            KeyCode::R => KeyType::Letter('r'),
            KeyCode::P => KeyType::Letter('p'),
            KeyCode::S => KeyType::Letter('s'),
            KeyCode::Q => KeyType::Letter('q'),
            KeyCode::T => KeyType::Letter('t'),
            KeyCode::V => KeyType::Letter('v'),
            KeyCode::W => KeyType::Letter('w'),
            KeyCode::X => KeyType::Letter('x'),
            KeyCode::Y => KeyType::Letter('y'),
            KeyCode::Z => KeyType::Letter('z'),
            KeyCode::Space => KeyType::Letter(' '), //TODO: think if this should be handeled better
            KeyCode::Backspace => KeyType::BackSpace,
            KeyCode::Escape => KeyType::Escape,
            _ => KeyType::Other,
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = iced::Theme;

    fn new(_flags: Self::Flags) -> (App, iced::Command<Self::Message>) {
        (App::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Type ninja ")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code,
                    modifiers,
                }) => {
                    // TODO: move to new func
                    let typed_key = App::get_key_code(key_code);
                    match typed_key {
                        KeyType::Letter(typed_char) => {
                            let text_done = if !keyboard::Modifiers::shift(modifiers) {
                                self.text.type_letter(typed_char, self.timer)
                            } else if keyboard::Modifiers::shift(modifiers) {
                                self.text
                                    .type_letter(typed_char.to_ascii_uppercase(), self.timer)
                            } else {
                                false
                            };
                            if text_done {
                                self.text.print();
                                self.view = Views::Results;
                            }
                        }
                        KeyType::BackSpace => {
                            self.text.back_space();
                        }
                        KeyType::Escape => std::process::exit(0),
                        KeyType::Other => (),
                    };
                }
                _ => (),
            },
            Message::GoToResults => self.view = Views::Results,
            Message::Tick(time) => {
                self.timer.update_timer(time);
            }
        };
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let layout = match self.view {
            Views::TypingPage => Container::new(self.render_text())
                .center_x()
                .center_y()
                .width(iced::Length::Fill)
                .height(iced::Length::Fill),
            Views::Results => {
                Container::new(iced::widget::row![Canvas::new(Graph {})]) //todo refractoring
                    .center_x()
                    .center_y()
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill)
            }
            // Views::Results => Container::new(Text::new("wow udałos się"))
            //     .center_x()
            //     .center_y()
            //     .width(iced::Length::Fill)
            //     .height(iced::Length::Fill),
            Views::MainPage => todo!(),
        };
        layout.into()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::Subscription::batch(vec![
            iced::time::every(std::time::Duration::from_millis(500))
                .map(|_| Message::Tick(Utc::now())),
            subscription::events().map(Message::Event),
        ])
    }
}
