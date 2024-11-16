use crate::tokenizer::Token;

use super::{parser_compass::Compass, parser_port::Port};

#[derive(Clone, Debug, PartialEq)]
pub enum ParseOutput {
    Compass(Compass),
    Port(Port),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParseBufferItem {
    Token(Token),
    ParseOutput(ParseOutput),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParseResult<T> {
    pub result: T,
    pub remaining: Vec<ParseBufferItem>,
}

pub trait Parser<T> {
    fn parse(&self, input: &[ParseBufferItem]) -> Option<ParseResult<T>>;
}

// pub struct ParseResult<'a, T> {
//     pub result: T,
//     pub remaining: &'a [Token],
// }

// pub trait Parser<'a, T> {
//     fn parse(&self, input: &'a [Token]) -> Option<ParseResult<'a, T>>;
// }
