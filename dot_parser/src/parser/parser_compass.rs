use crate::tokenizer::Token;

use super::parser::{ParseBufferItem, ParseResult, Parser};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Compass {
    N,
    Ne,
    E,
    Se,
    S,
    Sw,
    W,
    Nw,
    #[default]
    C,
    Underscore,
}

impl Parser<Compass> for Compass {
    fn parse(&self, input: &[ParseBufferItem]) -> Option<ParseResult<Compass>> {
        let first = match input.first()? {
            ParseBufferItem::Token(val) => val,
            _ => {
                return None;
            }
        };

        match first {
            Token::Identifier(ref val) => {
                let result = match val.as_str() {
                    "n" => Some(Compass::N),
                    "ne" => Some(Compass::Ne),
                    "e" => Some(Compass::E),
                    "se" => Some(Compass::Se),
                    "s" => Some(Compass::S),
                    "sw" => Some(Compass::Sw),
                    "w" => Some(Compass::W),
                    "nw" => Some(Compass::Nw),
                    "c" => Some(Compass::C),
                    "_" => Some(Compass::Underscore),
                    _ => None,
                };
                result.map(|compass| ParseResult {
                    result: compass,
                    remaining: input[1..].to_vec(),
                })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_compass() {
        let input = vec![ParseBufferItem::Token(Token::Identifier("n".to_string()))];
        let expected = Compass::N;
        let result = Compass::N.parse(&input);
        assert_eq!(
            result,
            Some(ParseResult {
                result: expected,
                remaining: vec![]
            })
        );
    }

    #[test]
    fn test_parse_compass_with_remaining() {
        let input = vec![
            ParseBufferItem::Token(Token::Identifier("n".to_string())),
            ParseBufferItem::Token(Token::Identifier("ne".to_string())),
        ];
        let expected = Compass::N;
        let result = Compass::N.parse(&input);
        assert_eq!(
            result,
            Some(ParseResult {
                result: expected,
                remaining: vec![ParseBufferItem::Token(Token::Identifier("ne".to_string()))]
            })
        );
    }

    #[test]
    fn test_parse_compass_fail() {
        let input = vec![ParseBufferItem::Token(Token::Identifier(
            "hello".to_string(),
        ))];
        let result = Compass::N.parse(&input);
        assert_eq!(result, None);
    }
}
