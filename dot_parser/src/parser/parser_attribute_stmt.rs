use crate::tokenizer::{Keyword, Token};

use super::{
    parser::{ParseBufferItem, ParseResult, Parser},
    parser_attr_list::AttrList,
};

#[derive(Debug, Clone, PartialEq)]
pub enum AttrStmtKind {
    Graph,
    Node,
    Edge,
}

// attr_stmt: (graph | node | edge) attr_list
#[derive(Debug, Clone, PartialEq)]
pub struct AttrStmt {
    pub kind: AttrStmtKind,
    pub attr_list: AttrList,
}

impl AttrStmt {
    pub fn new(kind: AttrStmtKind, attr_list: AttrList) -> Self {
        Self { kind, attr_list }
    }
}

impl Default for AttrStmt {
    fn default() -> Self {
        AttrStmt {
            kind: AttrStmtKind::Graph,
            attr_list: AttrList::default(),
        }
    }
}

impl Parser<AttrStmt> for AttrStmt {
    fn parse(&self, input: &[ParseBufferItem]) -> Option<ParseResult<AttrStmt>> {
        if input.is_empty() {
            return None;
        }
        let first: Option<&ParseBufferItem> = input.first();
        match first {
            Some(ParseBufferItem::Token(Token::Keyword(Keyword::Graph))) => {
                let attr_list = AttrList::default().parse(&input[1..]);
                let attr_list = attr_list.as_ref()?.clone();
                Some(ParseResult {
                    result: AttrStmt::new(AttrStmtKind::Graph, attr_list.result),
                    remaining: attr_list.remaining,
                })
            }
            Some(ParseBufferItem::Token(Token::Keyword(Keyword::Node))) => {
                let attr_list = AttrList::default().parse(&input[1..]);
                let attr_list = attr_list.as_ref()?.clone();
                Some(ParseResult {
                    result: AttrStmt::new(AttrStmtKind::Node, attr_list.result),
                    remaining: attr_list.remaining,
                })
            }
            Some(ParseBufferItem::Token(Token::Keyword(Keyword::Edge))) => {
                let attr_list = AttrList::default().parse(&input[1..]);
                let attr_list = attr_list.as_ref()?.clone();
                Some(ParseResult {
                    result: AttrStmt::new(AttrStmtKind::Edge, attr_list.result),
                    remaining: attr_list.remaining,
                })
            }
            _ => None,
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    use crate::{parser::parser_attribute, tokenizer::Delimiter};

    use super::*;

    #[test]
    fn test_attribute_stmt() {
        let input = vec![
            ParseBufferItem::Token(Token::Keyword(Keyword::Graph)),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::OpenSquareBrace)),
            ParseBufferItem::Token(Token::Identifier("label".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Equal)),
            ParseBufferItem::Token(Token::Identifier("hello".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::ClosedSquareBrace)),
        ];
        let expected = AttrStmt::new(
            AttrStmtKind::Graph,
            AttrList {
                items: vec![parser_attribute::Attribute {
                    lhs: "label".to_string(),
                    rhs: "hello".to_string(),
                }],
            },
        );
        let result = AttrStmt::new(AttrStmtKind::Graph, AttrList::default()).parse(&input);
        assert_eq!(
            result,
            Some(ParseResult {
                result: expected,
                remaining: vec![]
            })
        );
    }
}
