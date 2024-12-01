use crate::tokenizer::{Delimiter, Token};

use super::{
    parser::{ParseBufferItem, ParseResult, Parser},
    parser_a_list::AList,
    parser_attribute::Attribute,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AttrList {
    pub items: Vec<Attribute>,
}

impl Default for AttrList {
    fn default() -> Self {
        AttrList { items: vec![] }
    }
}

// attr_list : '[' [ a_list ] ']' [ attr_list ]
impl Parser<AttrList> for AttrList {
    fn parse(&self, input: &[ParseBufferItem]) -> Option<ParseResult<AttrList>> {
        if input.len() < 5 {
            return None;
        }

        let first = input.first()?;

        if first != &ParseBufferItem::Token(Token::Delimiter(Delimiter::OpenSquareBrace)) {
            return None;
        }

        // check if the next item is a_list
        let a_list = AList::default().parse(&input[1..].to_vec());
        let mut items: Vec<Attribute> = vec![];

        if a_list.is_none() {
            return None;
        }


        if let Some(a_list) = a_list.clone() {
            items = [items, a_list.result.items].concat();
        }

        let rest = a_list?.remaining;

        if rest.is_empty() {
            return None;
        }

        if rest.first()?
            != &ParseBufferItem::Token(Token::Delimiter(Delimiter::ClosedSquareBrace))
        {
            return None;
        }

        let rest = &rest[1..];

        let next = AttrList::default().parse(&rest);

        if next.is_none() {
            return Some(ParseResult {
                result: AttrList { items },
                remaining: rest.to_vec(),
            });
        }

        let next = next.unwrap();
        items = [items, next.result.items].concat();

        Some(ParseResult {
            result: AttrList { items },
            remaining: next.remaining,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attr_list() {
        let input = vec![
            ParseBufferItem::Token(Token::Delimiter(Delimiter::OpenSquareBrace)),
            ParseBufferItem::Token(Token::Identifier("label".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Equal)),
            ParseBufferItem::Token(Token::Identifier("hello".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::ClosedSquareBrace)),
        ];

        let expected = AttrList {
            items: vec![Attribute {
                lhs: "label".to_string(),
                rhs: "hello".to_string(),
            }],
        };

        let result = AttrList::default().parse(&input);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().result, expected);
    }

    #[test]
    fn test_attr_list_multiple() {
        let input = vec![
            ParseBufferItem::Token(Token::Delimiter(Delimiter::OpenSquareBrace)),
            ParseBufferItem::Token(Token::Identifier("label".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Equal)),
            ParseBufferItem::Token(Token::Identifier("hello".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::ClosedSquareBrace)),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::OpenSquareBrace)),
            ParseBufferItem::Token(Token::Identifier("color".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Equal)),
            ParseBufferItem::Token(Token::Identifier("red".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::ClosedSquareBrace)),
        ];

        let expected = AttrList {
            items: vec![
                Attribute {
                    lhs: "label".to_string(),
                    rhs: "hello".to_string(),
                },
                Attribute {
                    lhs: "color".to_string(),
                    rhs: "red".to_string(),
                },
            ],
        };

        let result = AttrList::default().parse(&input);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().result, expected);
    }

    #[test]
    fn mutliple_with_remaining() {
        let input = vec![
            ParseBufferItem::Token(Token::Delimiter(Delimiter::OpenSquareBrace)),
            ParseBufferItem::Token(Token::Identifier("label".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Equal)),
            ParseBufferItem::Token(Token::Identifier("hello".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::ClosedSquareBrace)),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::OpenSquareBrace)),
            ParseBufferItem::Token(Token::Identifier("color".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Equal)),
            ParseBufferItem::Token(Token::Identifier("red".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::ClosedSquareBrace)),
            ParseBufferItem::Token(Token::Identifier("graph".to_string())),
        ];

        let expected = AttrList {
            items: vec![
                Attribute {
                    lhs: "label".to_string(),
                    rhs: "hello".to_string(),
                },
                Attribute {
                    lhs: "color".to_string(),
                    rhs: "red".to_string(),
                },
            ],
        };

        let result = AttrList::default().parse(&input);

        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap().result, expected);
        assert_eq!(result.unwrap().remaining.len(), 1);
    }
}
