use crate::tokenizer::{Delimiter, Token};

use super::{
    parser::{ParseBufferItem, ParseResult, Parser},
    parser_attribute::Attribute,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AList {
    pub items: Vec<Attribute>,
}

impl Default for AList {
    fn default() -> Self {
        AList { items: vec![] }
    }
}

// I am taking a risk here, ID = ID is same as Attribute
// a_list : ID '=' ID [ (';' | ',') ] [ a_list ]
impl Parser<AList> for AList {
    fn parse(&self, input: &[ParseBufferItem]) -> Option<ParseResult<AList>> {
        if input.len() < 3 {
            return None;
        }
        let attribute: Option<ParseResult<Attribute>> = Attribute::default().parse(&input[0..3].to_vec());

        if attribute.is_none() {
            return None;
        }

        let results = attribute.unwrap();
        let attributes = vec![results.result];

        let mut has_more = false;
        match input.get(3) {
            Some(ParseBufferItem::Token(Token::Delimiter(Delimiter::Semicolon))) => {
                has_more = true;
            }
            Some(ParseBufferItem::Token(Token::Delimiter(Delimiter::Comma))) => {
                has_more = true;
            }
            _ => {}
        };

        if !has_more {
            return Some(ParseResult {
                result: AList {
                    items: attributes,
                },
                remaining: input[3..].to_vec(),
            });
        }

        let rest = &input[4..];
        let next = AList::default().parse(rest);
        match next {
            None => Some(ParseResult {
                result: AList {
                    items: attributes,
                },
                remaining: rest.to_vec(),
            }),
            Some(next) => {
                let next_items = next.result.items;
                let items = [attributes, next_items].concat();
                return Some(ParseResult {
                    result: AList {
                        items,
                    },
                    remaining: next.remaining,
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_a_list() {
        let input = vec![
            ParseBufferItem::Token(Token::Identifier("node1".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Equal)),
            ParseBufferItem::Token(Token::Identifier("node2".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Semicolon)),
            ParseBufferItem::Token(Token::Identifier("node3".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Equal)),
            ParseBufferItem::Token(Token::Identifier("node4".to_string())),
        ];
        let expected = AList {
            items: vec![
                Attribute {
                    lhs: "node1".to_string(),
                    rhs: "node2".to_string(),
                },
                Attribute {
                    lhs: "node3".to_string(),
                    rhs: "node4".to_string(),
                },
            ],
        };
        let result = AList::default().parse(&input);
        assert_eq!(
            result,
            Some(ParseResult {
                result: expected,
                remaining: vec![]
            })
        );
    }

    #[test]
    fn test_parse_a_list_with_remaining() {
        let input = vec![
            ParseBufferItem::Token(Token::Identifier("node1".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Equal)),
            ParseBufferItem::Token(Token::Identifier("node2".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Semicolon)),
            ParseBufferItem::Token(Token::Identifier("node3".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Equal)),
            ParseBufferItem::Token(Token::Identifier("node4".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Semicolon)),
            ParseBufferItem::Token(Token::Identifier("node5".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Equal)),
            ParseBufferItem::Token(Token::Identifier("node6".to_string())),
            ParseBufferItem::Token(Token::Identifier("node7".to_string())),
        ];
        let expected = AList {
            items: vec![
                Attribute {
                    lhs: "node1".to_string(),
                    rhs: "node2".to_string(),
                },
                Attribute {
                    lhs: "node3".to_string(),
                    rhs: "node4".to_string(),
                },
                Attribute {
                    lhs: "node5".to_string(),
                    rhs: "node6".to_string(),
                },
            ],
        };
        let result = AList::default().parse(&input);
        assert_eq!(
            result,
            Some(ParseResult {
                result: expected,
                remaining: vec![ParseBufferItem::Token(Token::Identifier(
                    "node7".to_string()
                ))]
            })
        );
    }
}
