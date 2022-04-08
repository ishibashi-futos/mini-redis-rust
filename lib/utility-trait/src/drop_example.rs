use std::ops::{Deref, DerefMut};

pub struct Application<'a> {
    pub name: String,
    pub nicknames: Vec<String>,
    writer: &'a mut dyn Writer,
}

impl<'a> Drop for Application<'a> {
    fn drop(&mut self) {
        self.writer.write(format!("Dropping {}", self.name));
        if !self.nicknames.is_empty() {
            self.writer
                .write(format!("(AKA {})", self.nicknames.join(", ")));
        }
    }
}

trait Writer {
    fn write(&mut self, message: String);
}

pub struct WriterImpl {
    messages: Vec<String>,
}

impl WriterImpl {
    pub fn new() -> Self {
        WriterImpl {
            messages: Vec::new(),
        }
    }

    pub fn get_all_message(&self) -> &Vec<String> {
        &self.messages
    }
}

impl Writer for WriterImpl {
    fn write(&mut self, message: String) {
        self.messages.push(message);
    }
}

pub struct Selector<T> {
    pub elements: Vec<T>,
    pub current: usize,
}

impl<T> Selector<T> {
    pub fn next(&mut self) {
        if self.elements.len() - 1 > self.current {
            self.current += 1;
        }
    }
}

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

impl<T> Default for Selector<T> {
    fn default() -> Self {
        Selector {
            elements: Vec::new(),
            current: 0usize,
        }
    }
}

pub enum Fruits {
    Apple,
    Banana,
    Grape,
}

impl Default for Fruits {
    fn default() -> Self {
        Fruits::Apple
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect::expect;

    #[test]
    fn writer() {
        let mut w = WriterImpl::new();
        {
            let _a = Application {
                name: "John".to_owned(),
                nicknames: vec!["Jack".to_owned(), "Snake".to_owned()],
                writer: &mut w,
            };
            // この行で`_a`の値がDropされる
        }
        expect(w.get_all_message()).equals(&vec![
            "Dropping John".to_owned(),
            "(AKA Jack, Snake)".to_owned(),
        ]);
    }

    #[test]
    fn deref() {
        let mut s = Selector {
            elements: vec!['x', 'y', 'z'],
            current: 0,
        };
        expect(&s.to_uppercase().to_string()).equals(&'X'.to_string());
        s.next();
        expect(&s.to_uppercase().to_string()).equals(&'Y'.to_string());
        s.next();
        expect(&s.to_uppercase().to_string()).equals(&'Z'.to_string());
        s.next();
        expect(&s.to_uppercase().to_string()).equals(&'Z'.to_string());

        s.elements.push('d');
        s.next();
        expect(&s.to_uppercase().to_string()).equals(&'D'.to_string());
    }
}
