use std::env;

use iced::keyboard::{self, KeyCode};
use iced::widget::{Button, Column, Container, Row, Text};
use iced::{executor, Color, Command, Renderer, Settings, Subscription};
use iced::{subscription, Application, Element, Event};

fn main() -> Result<(), iced::Error> {
    env::set_var("RUST_BACKTRACE", "full");
    App::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(Event),
}

struct App {
    text: String,
    current_letter: usize,
}

impl App {
    fn new() -> App {
        App {
            text: String::from("HellO worlddd"),
            current_letter: 0,
        }
    }

    fn render_text(&self) -> iced_native::widget::Row<'static, Message, Renderer> {
        let mut row = Row::new();
        let (typed, to_type) = self.text.split_at(self.current_letter);

        let row = row.push(Text::new(String::from(typed)).style(Color::from([1., 0.5, 0.5])));
        let row = row.push(Text::new(String::from('|')).style(Color::from([0., 0., 0.])));
        let row = row.push(Text::new(String::from(to_type)).style(Color::from([0., 1., 0.5])));
        row
    }

    fn get_key_code(key_code: keyboard::KeyCode) -> char {
        match key_code {
            KeyCode::A => 'a',
            KeyCode::B => 'b',
            KeyCode::C => 'c',
            KeyCode::D => 'd',
            KeyCode::E => 'e',
            KeyCode::F => 'f',
            KeyCode::G => 'g',
            KeyCode::H => 'h',
            KeyCode::I => 'i',
            KeyCode::J => 'j',
            KeyCode::K => 'k',
            KeyCode::L => 'l',
            KeyCode::M => 'm',
            KeyCode::N => 'n',
            KeyCode::O => 'o',
            KeyCode::U => 'u',
            KeyCode::R => 'r',
            KeyCode::P => 'p',
            KeyCode::S => 's',
            KeyCode::Q => 'q',
            KeyCode::T => 't',
            KeyCode::V => 'v',
            KeyCode::W => 'w',
            KeyCode::X => 'x',
            KeyCode::Y => 'y',
            KeyCode::Z => 'z',
            KeyCode::Space => ' ',

            _ => '?',
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = iced::Theme;

    fn new(flags: Self::Flags) -> (App, iced::Command<Self::Message>) {
        (App::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Type ninja ")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Event(event) => match event {
                Event::Keyboard(keyboard_event) => match keyboard_event {
                    keyboard::Event::KeyPressed {
                        key_code,
                        modifiers,
                    } => {
                        let letter = self.text.chars().nth(self.current_letter).unwrap();
                        if App::get_key_code(key_code) == letter
                            && !keyboard::Modifiers::shift(modifiers)
                        {
                            self.current_letter += 1;
                        } else if letter.is_uppercase()
                            && keyboard::Modifiers::shift(modifiers)
                            && App::get_key_code(key_code).to_ascii_uppercase() == letter
                        {
                            assert!(letter.is_uppercase());
                            self.current_letter += 1;
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let counter_layout = Container::new(self.render_text())
            .center_x()
            .center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill);
        counter_layout.into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events().map(Message::Event)
    }
}
