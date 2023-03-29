use iced::keyboard::KeyCode;
use iced::pure::widget::{Button, Column, Container, Row, Text};
use iced::pure::{Application, Element};
use iced::{executor, Command, Settings};

fn main() -> Result<(), iced::Error> {
    App::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub struct Message {
    nextKey: char,
}

struct App;

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(flags: Self::Flags) -> (App, iced::Command<Self::Message>) {
        (App, Command::none())
    }

    fn title(&self) -> String {
        String::from("Type ninja ")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        todo!()
    }

    fn view(&self) -> iced::pure::Element<'_, Self::Message> {
        let a = Text::new("a");
        let b = Text::new("b");
        let c = Text::new("c");
        let row = Row::new().push(a).push(b).push(c);
        let counter_layout = Container::new(row)
            .center_x()
            .center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill);
        counter_layout.into()
    }
}


// struct Counter {
//     count: i32,
//     current_view: Views,
//     main_page: MainPage,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum Views {
//     Counter,
//     Main,
// }

// #[derive(Debug, Clone, Copy)]
// pub enum CounterMessage {
//     Increment,
//     Decrement,
//     ChangePage(Views),
// }

// impl Application  for Counter {
//     type Message = CounterMessage;

//     fn new() -> Self {
//         Counter {
//             count: 0,
//             current_view: Views::Counter,
//             main_page: MainPage::new(),
//         }
//     }

//     fn title(&self) -> String {
//         String::from("Counter app")
//     }

//     fn update(&mut self, message: Self::Message) {
//         match message {
//             CounterMessage::Increment => self.count += 1,
//             CounterMessage::Decrement => self.count -= 1,
//             CounterMessage::ChangePage(view) => self.current_view = view,
//         }
//     }

//     fn view(&self) -> iced::pure::Element<Self::Message> {
//         let a = Text::new("a");
//         let b = Text::new("b");
//         let c = Text::new("c");
//         let row = Row::new().push(a).push(b).push(c);
//         let counter_layout = Container::new(row)
//             .center_x()
//             .center_y()
//             .width(iced::Length::Fill)
//             .height(iced::Length::Fill);
//         match self.current_view {
//             Views::Counter => counter_layout.into(),
//             Views::Main => self.main_page.view(),
//         }
//     }
// }
