pub struct TextStorage {
    pub(crate) typed: Vec<TextType>,
    pub(crate) toType: String,
}

//TODO: dodać testy
impl TextStorage {
    pub(crate) fn new(text: String) -> TextStorage {
        TextStorage {
            toType: text,
            typed: Vec::new(),
        }
    }

    pub(crate) fn type_letter(&mut self, letter: char) -> bool {
        if self.toType.is_empty() {
            return true;
        }
        if self.is_letter_correct(letter) {
            self.typed_correctly(letter);
        } else {
            self.typed_wrongly(letter);
        }
        false
    }

    fn is_letter_correct(&self, letter: char) -> bool {
        self.toType.starts_with(letter)
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
        self.toType = String::from(&self.toType[1..]);
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
                        self.toType.insert(0, letter);
                        if t.is_empty() {
                            self.typed.pop();
                        }
                    }
                }
            }
        }
    }
}

pub(crate) enum TextType {
    Typed(String),
    Error(String),
}
