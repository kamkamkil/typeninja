use std::env;

use iced::keyboard::{self, KeyCode};
use iced::widget::{Button, Column, Container, Row, Text};
use iced::{executor, Color, Command, Renderer, Settings, Subscription};
use iced::{subscription, Application, Element, Event};
use std::time::{Duration, Instant};

fn main() -> Result<(), iced::Error> {
    env::set_var("RUST_BACKTRACE", "full");
    App::run(Settings::default())
}

pub struct TextStorage {
    text: Vec<TextType>,
    toType: String
}
//TODO: dodać testy
//TODO: przenieść do modułu
impl TextStorage {
    fn new(text: String) -> TextStorage {
        TextStorage {
            toType: text,
            text: Vec::new(),
        }
    }
    
    fn type_letter(&mut self, letter: char) {
        let mut iter = self.text.iter_mut().rev();
        if let Some(part_of_text) = iter.next() {
            match part_of_text {
                TextType::ToType(typed) => {
                    if typed.starts_with(letter) {
                        typed.remove(0);
                        match iter.next() {
                            Some(TextType::Typed(t)) => t.push(letter),
                            Some(TextType::Error(_)) => self
                                .text
                                .insert(self.text.len() - 1, TextType::Typed(String::from(letter))),
                            Some(TextType::ToType(a)) => println!("pojawił się błąd w type_letter 1 {}",a),
                            None => self.text.insert(0, TextType::Typed(String::from(letter))),
                        }
                    }
                    else{
                        //tutaj trzeba przewidizeń sytuację kiedy pierwsza litera jest błędna 
                        match iter.next() {
                            Some(TextType::Error(t)) => t.push(letter),
                            Some(TextType::Typed(_)) => self
                                .text
                                .insert(self.text.len() - 1, TextType::Error(String::from(letter))),
                            Some(TextType::ToType(a)) => println!("pojawił się błąd w type_letter 14 {}",a),
                            None => self.text.insert(0, TextType::Typed(String::from(letter))),
                        }
                    }
                }
                TextType::Error(q) => println!("pojawił się błąd w type_letter 2 {}" ,q),
                TextType::Typed(q) => println!("pojawił się błąd w type_letter 3 {}" , q),
            }
        }
    }
// to musi nie tyklko usunąc ale też dodać do totype
    fn back_space(&mut self) {
        if let Some(end) = self.text.iter_mut().rev().nth(1) {
            match end {
                TextType::Typed(letter) | TextType::ToType(letter) => {
                    letter.pop();
                }
                TextType::Error(letter) => {
                    letter.pop();
                    if end.is_empty() {
                        self.text.pop();
                    };
                }
            };
        }
    }
}
enum TextType {
    Typed(String),
    ToType(String),
    Error(String),
}

impl TextType {
    fn is_empty(&self) -> bool {
        match self {
            TextType::Typed(text) | TextType::ToType(text) | TextType::Error(text) => {
                text.is_empty()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(Event),
    GoToResults,
}

struct TextStyle {
    typed_color: Color,
    error_color: Color,
    to_type_color: Color,
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
}

struct App {
    view: Views,
    text_style: TextStyle,
    text: TextStorage,
}

impl App {
    fn new() -> App {
        App {
            view: Views::TypingPage,
            text_style: TextStyle {
                typed_color: Color::from([0., 0., 0.]),
                error_color: Color::from([1., 0.2, 0.2]),
                to_type_color: Color::from([0.7, 0.7, 0.7]),
            },
            text: TextStorage::new(String::from("hello world")),
        }
    }
    //TODO: add | and mayby change some var names 
    fn render_text(&self) -> iced_native::widget::Row<'static, Message, Renderer> {
        let mut row = Row::new();
        for text in &self.text.text {
            match text {
                TextType::Typed(text) => {
                    row = row.push(Text::new(String::from(text)).style(self.text_style.typed_color))
                }
                TextType::ToType(text) => {
                    row =
                        row.push(Text::new(String::from(text)).style(self.text_style.to_type_color))
                }
                TextType::Error(text) => {
                    row = row.push(Text::new(String::from(text)).style(self.text_style.error_color))
                }
            }
        }
        row
    }

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
                            if !keyboard::Modifiers::shift(modifiers) {
                                self.text.type_letter(typed_char);
                            }else if keyboard::Modifiers::shift(modifiers) {
                                self.text.type_letter(typed_char.to_ascii_uppercase());
                                
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
            Views::Results => Container::new(Text::new("wow udałos się"))
                .center_x()
                .center_y()
                .width(iced::Length::Fill)
                .height(iced::Length::Fill),
        };
        layout.into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events().map(Message::Event)
    }
}
