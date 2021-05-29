//! This file might be needed in the future, however, it is dead code for now
//! since we're using the [Logos](https://github.com/maciejhirsz/logos) lexer
//! generator.

#![allow(dead_code)]
use std::rc::Rc;

/// Stream represents a functional-style char stream that doesn't modify itself,
/// but instead returns a copy of itself with all necessary modifications.
/// In this case, copying is efficient due to the fact that the chars are
/// read-only and can therefore by safely shared between all Stream clones.
#[derive(Clone)]
pub struct Stream {
    chars: Rc<Vec<char>>,
    cursor: usize,
}

pub type CharChecker = fn(char) -> bool;

impl Stream {
    pub fn new(input: String) -> Self {
        Self {
            chars: Rc::new(input.chars().collect()),
            cursor: 0,
        }
    }

    pub fn current(&self) -> char {
        self.chars[self.cursor]
    }

    pub fn eof(&self) -> bool {
        self.cursor >= self.chars.len()
    }

    /// Return a copy of this Stream with a cursor shifted one char to the
    /// right. On EOF, return None.
    pub fn shift(&self) -> Self {
        if self.eof() {
            return self.clone();
        }
        let mut clone = self.clone();
        clone.cursor += 1;
        clone
    }

    /// Take a peek at the next char wihout shifting the cursor.
    /// On EOF, return None.
    pub fn peek(&self) -> Option<char> {
        if self.eof() {
            return None;
        }
        Some(self.current())
    }

    pub fn shift_if(&self, condition: CharChecker) -> Self {
        if self.eof() || !condition(self.current()) {
            return self.clone();
        }
        self.shift()
    }

    pub fn shift_while(&self, condition: CharChecker) -> Self {
        if self.eof() || !condition(self.current()) {
            return self.clone();
        }
        self.shift().shift_while(condition)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_peek_shift_peek() {
        let s1 = Stream::new("a".to_string());
        assert_eq!('a', s1.peek().expect("peek was supposed to return Some"));
        let s2 = s1.shift();
        assert!(s2.peek().is_none());
    }

    #[test]
    fn test_shift_if_and_while() {
        let s1 = Stream::new("a \n".to_string());
        let s2 = s1.shift_if(|c| c == 'a');
        assert_eq!(' ', s2.peek().expect("peek was supposed to return Some"));
        let s3 = s2.shift_while(|c| c.is_ascii_whitespace());
        assert!(s3.eof());
    }
}
