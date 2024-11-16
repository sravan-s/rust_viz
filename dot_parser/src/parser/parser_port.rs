use crate::tokenizer::{Delimiter, Token};

use super::{
    parser::{ParseBufferItem, ParseResult, Parser},
    parser_compass::Compass,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Port {
    pub id: Option<String>,
    pub compass: Option<Compass>,
}

impl Parser<Port> for Port {
    fn parse(&self, input: &[ParseBufferItem]) -> Option<super::parser::ParseResult<Port>> {
        let first = input.first()?;
        let second = input.get(1)?;
        if *first != ParseBufferItem::Token(Token::Delimiter(Delimiter::Colon)) {
            return None;
        }

        let second_as_vec = vec![second.clone()];
        let second_as_compass = Compass::W.parse(&second_as_vec);
        let second_as_id = match second {
            ParseBufferItem::Token(Token::Identifier(ref val)) => Some(val),
            _ => None,
        };

        if second_as_compass.is_none() && second_as_id.is_none() {
            return None;
        }

        // If the second item is a compass, has higher priority
        if second_as_compass.is_some() {
            let second_compass = second_as_compass?;
            return Some(ParseResult {
                result: Port {
                    id: None,
                    compass: Some(second_compass.result),
                },
                remaining: input[2..].to_vec(),
            });
        }

        // If the second item is an identifier, check if the third item is a compass
        if second_as_id.is_some() {
            let second_as_id = second_as_id?;
            let third = input.get(2);
            let fourth = input.get(3);
            match (third, fourth) {
                (
                    Some(ParseBufferItem::Token(Token::Delimiter(Delimiter::Colon))),
                    Some(ParseBufferItem::Token(Token::Identifier(_))),
                ) => {
                    let fourth_as_vec = vec![fourth?.clone()];
                    let fourth_as_compass = Compass::W.parse(&fourth_as_vec);
                    if fourth_as_compass.is_some() {
                        let fourth_compass = fourth_as_compass?;
                        return Some(ParseResult {
                            result: Port {
                                id: Some(second_as_id.to_string()),
                                compass: Some(fourth_compass.result),
                            },
                            remaining: input[4..].to_vec(),
                        });
                    }
                }
                _ => {
                    return Some(ParseResult {
                        result: Port {
                            id: Some(second_as_id.to_string()),
                            compass: None,
                        },
                        remaining: input[2..].to_vec(),
                    });
                }
            };
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_port_has_priority_over_id() {
        let input = vec![
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Colon)),
            ParseBufferItem::Token(Token::Identifier("n".to_string())),
        ];
        let expected = Port {
            id: None,
            compass: Some(Compass::N),
        };
        let result = Port {
            id: None,
            compass: None,
        }
        .parse(&input);
        assert_eq!(
            result,
            Some(ParseResult {
                result: expected,
                remaining: vec![]
            })
        );
    }

    #[test]
    fn test_parse_port_returns_remainig() {
        let input = vec![
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Colon)),
            ParseBufferItem::Token(Token::Identifier("n".to_string())),
            ParseBufferItem::Token(Token::Identifier("port".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Colon)),
        ];
        let expected = Port {
            id: None,
            compass: Some(Compass::N),
        };
        let result = Port {
            id: None,
            compass: None,
        }
        .parse(&input);
        assert_eq!(
            result,
            Some(ParseResult {
                result: expected,
                remaining: vec![
                    ParseBufferItem::Token(Token::Identifier("port".to_string())),
                    ParseBufferItem::Token(Token::Delimiter(Delimiter::Colon))
                ]
            })
        );
    }

    #[test]
    fn test_parse_port_with_id() {
        let input = vec![
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Colon)),
            ParseBufferItem::Token(Token::Identifier("val".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::OpenSquareBrace)),
            ParseBufferItem::Token(Token::Identifier("port".to_string())),
        ];
        let expected = Port {
            id: Some("val".to_string()),
            compass: None,
        };
        let result = Port {
            id: None,
            compass: None,
        }
        .parse(&input);
        assert_eq!(
            result,
            Some(ParseResult {
                result: expected,
                remaining: vec![
                    ParseBufferItem::Token(Token::Delimiter(Delimiter::OpenSquareBrace)),
                    ParseBufferItem::Token(Token::Identifier("port".to_string())),
                ]
            })
        );
    }

    #[test]
    fn test_parse_port_with_id_and_compass() {
        let input = vec![
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Colon)),
            ParseBufferItem::Token(Token::Identifier("port".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Colon)),
            ParseBufferItem::Token(Token::Identifier("w".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::OpenSquareBrace)),
            ParseBufferItem::Token(Token::Identifier("port".to_string())),
        ];
        let expected = Port {
            id: Some("port".to_string()),
            compass: Some(Compass::W),
        };
        let result = Port {
            id: None,
            compass: None,
        }
        .parse(&input);
        assert_eq!(
            result,
            Some(ParseResult {
                result: expected,
                remaining: vec![
                    ParseBufferItem::Token(Token::Delimiter(Delimiter::OpenSquareBrace)),
                    ParseBufferItem::Token(Token::Identifier("port".to_string()))
                ]
            })
        );
    }

    #[test]
    fn test_parse_port_fail() {
        let input = vec![ParseBufferItem::Token(Token::Identifier(
            "hello".to_string(),
        ))];
        let result = Port {
            id: None,
            compass: None,
        }
        .parse(&input);
        assert_eq!(result, None);
    }
}
