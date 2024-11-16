use crate::tokenizer::{Delimiter, Token};

use super::parser::{ParseBufferItem, ParseResult, Parser};

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub lhs: String,
    pub rhs: String,
}

impl Attribute {
    pub fn new(lhs: String, rhs: String) -> Self {
        Self { lhs, rhs }
    }
}

impl Parser<Attribute> for Attribute {
    fn parse(&self, input: &[ParseBufferItem]) -> Option<ParseResult<Attribute>> {
        let first: Option<&ParseBufferItem> = input.first();
        let second: Option<&ParseBufferItem> = input.get(1);
        let third: Option<&ParseBufferItem> = input.get(2);
        match (first, second, third) {
            (
                Some(ParseBufferItem::Token(Token::Identifier(lhs))),
                Some(ParseBufferItem::Token(Token::Delimiter(Delimiter::Equal))),
                Some(ParseBufferItem::Token(Token::Identifier(rhs))),
            ) => Some(ParseResult {
                result: Attribute::new(lhs.to_string(), rhs.to_string()),
                remaining: input[3..].to_vec(),
            }),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_attribute() {
        let input = vec![
            ParseBufferItem::Token(Token::Identifier("label".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Equal)),
            ParseBufferItem::Token(Token::Identifier("hello".to_string())),
        ];
        let expected = Attribute::new("label".to_string(), "hello".to_string());
        let result = Attribute::new("".to_string(), "".to_string()).parse(&input);
        assert_eq!(result, Some(ParseResult { result: expected, remaining: vec![] }));
    }

    #[test]
    fn test_parse_attribute_with_remaining() {
        let input = vec![
            ParseBufferItem::Token(Token::Identifier("label".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Equal)),
            ParseBufferItem::Token(Token::Identifier("hello".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Semicolon)),
        ];
        let expected = Attribute::new("label".to_string(), "hello".to_string());
        let result = Attribute::new("".to_string(), "".to_string()).parse(&input);
        assert_eq!(result, Some(ParseResult { result: expected, remaining: vec![ParseBufferItem::Token(Token::Delimiter(Delimiter::Semicolon))] }));
    }



    #[test]
    fn test_parse_attribute_fail() {
        let input = vec![
            ParseBufferItem::Token(Token::Identifier("label".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Equal)),
        ];
        let result = Attribute::new("".to_string(), "".to_string()).parse(&input);
        assert_eq!(result, None);
    }
}
