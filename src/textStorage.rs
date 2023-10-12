use crate::time;

pub struct TextStorage {
    pub(crate) typed: Vec<TextType>,
    pub(crate) to_type: String,
    typed_with_time: Vec<TypedLetter>,
}

//TODO: add tests
impl TextStorage {
    pub(crate) fn new(text: String) -> TextStorage {
        TextStorage {
            to_type: text,
            typed: Vec::new(),
            typed_with_time: Vec::new(),
        }
    }

    pub(crate) fn type_letter(&mut self, letter: char, time: time::Timer) -> bool {
        if self.to_type.is_empty() {
            return true;
        }
        if self.is_letter_correct(letter) {
            self.typed_with_time.push(TypedLetter::new(
                TextType::Typed(letter.to_string()),
                time.time_elapsed,
            ));
            self.typed_correctly(letter);
        } else {
            self.typed_with_time.push(TypedLetter::new(
                TextType::Error(letter.to_string()),
                time.time_elapsed,
            ));
            self.typed_wrongly(letter);
        }
        false
    }

    fn is_letter_correct(&self, letter: char) -> bool {
        self.to_type.starts_with(letter)
    }

    fn typed_wrongly(&mut self, letter: char) {
        if let Some(part_of_text) = self.typed.iter_mut().last() {
            match part_of_text {
                TextType::Typed(_) => self.typed.push(TextType::Error(String::from(letter))),
                TextType::Error(t) => t.push(letter),
            }
        } else {
            self.typed.push(TextType::Error(String::from(letter)))
        }
    }

    fn typed_correctly(&mut self, letter: char) {
        if let Some(part_of_text) = self.typed.iter_mut().last() {
            match part_of_text {
                TextType::Error(_) => self.typed.push(TextType::Typed(String::from(letter))),
                TextType::Typed(t) => t.push(letter),
            }
        } else {
            self.typed.push(TextType::Typed(String::from(letter)))
        }
        self.to_type = String::from(&self.to_type[1..]);
    }

    pub(crate) fn back_space(&mut self) {
        if let Some(part_of_text) = self.typed.iter_mut().last() {
            match part_of_text {
                TextType::Error(t) => {
                    t.pop();
                    if t.is_empty() {
                        self.typed.pop();
                    }
                }
                TextType::Typed(t) => {
                    if let Some(letter) = t.pop() {
                        self.to_type.insert(0, letter);
                        if t.is_empty() {
                            self.typed.pop();
                        }
                    }
                }
            }
        }
    }
    pub(crate) fn print(&mut self) {
        for letter in &self.typed_with_time {
            print!("time: m:{} s:{} ms:{} ",letter.time.minute,letter.time.second,letter.time.miliseconds);
            match &letter.text_type{
                TextType::Typed(l) => println!("ok {}",l),
                TextType::Error(l) => println!("error {}",l),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum TextType {
    Typed(String),
    Error(String),
}
#[derive(Debug, Clone)]
struct TypedLetter {
    text_type: TextType,
    time: time::Time,
}
impl TypedLetter {
    fn new(text_type: TextType, time: time::Time) -> Self {
        Self { text_type, time }
    }
}
