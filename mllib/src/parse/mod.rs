pub mod lexer;

#[derive(PartialEq, Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    Function(String),
    Symbol(String),
    Number(i64),
}
