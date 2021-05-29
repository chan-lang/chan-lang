extern crate logos;

use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq, Eq)]
pub enum Token {
    #[regex(r"(0|-?[1-9][0-9]*)", |lex| lex.slice().parse())]
    Int(i32),

    #[regex(r#""([^"\\]|\\[0nt"])*""#, string_without_ticks)]
    Str(String),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)] // skip whitespace
    Error,
}

fn string_without_ticks(lex: &mut Lexer<Token>) -> String {
    let slice = lex.slice();
    slice[1..slice.len() - 1].to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lex() {
        let mut lex = Token::lexer(r#"  "this is magic"  42  "#);

        assert_eq!(lex.next(), Some(Token::Str(r"this is magic".to_string())));
        assert_eq!(lex.span(), 2..17);
        assert_eq!(lex.slice(), r#""this is magic""#);

        assert_eq!(lex.next(), Some(Token::Int(42)));
        assert_eq!(lex.span(), 19..21);
        assert_eq!(lex.slice(), "42");
    }
}
