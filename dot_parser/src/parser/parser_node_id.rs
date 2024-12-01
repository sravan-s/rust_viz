use crate::tokenizer::{Delimiter, Token};

use super::{
    parser::{ParseBufferItem, ParseResult, Parser},
    parser_port::{self, Port},
};

#[derive(Debug, Clone, PartialEq)]
pub struct NodeId {
    pub id: String,
    pub port: Option<Port>,
}

impl Default for NodeId {
    fn default() -> Self {
        NodeId {
            id: "".to_string(),
            port: None,
        }
    }
}

impl Parser<NodeId> for NodeId {
    fn parse(&self, input: &[ParseBufferItem]) -> Option<ParseResult<NodeId>> {
        let first: &ParseBufferItem = input.first()?;
        // first item should be an identifier

        // get value of id from first item
        let id = match first {
            ParseBufferItem::Token(Token::Identifier(val)) => val.to_string(),
            _ => return None,
        };

        let rest = &input[1..];
        let is_port = parser_port::Port::default().parse(rest);
        match is_port {
            None => Some(ParseResult {
                result: NodeId { id, port: None },
                remaining: rest.to_vec(),
            }),
            Some(port) => Some(ParseResult {
                result: NodeId {
                    id,
                    port: Some(port.result),
                },
                remaining: port.remaining,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_node_id() {
        let input = vec![
            ParseBufferItem::Token(Token::Identifier("node1".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Colon)),
            ParseBufferItem::Token(Token::Identifier("port1".to_string())),
        ];
        let expected = NodeId {
            id: "node1".to_string(),
            port: Some(Port {
                id: Some("port1".to_string()),
                compass: None,
            }),
        };
        let result = NodeId::default().parse(&input);
        assert_eq!(
            result,
            Some(ParseResult {
                result: expected,
                remaining: vec![]
            })
        );
    }

    #[test]
    fn test_parse_node_id_without_port() {
        let input = vec![ParseBufferItem::Token(Token::Identifier("node1".to_string()))];
        let expected = NodeId {
            id: "node1".to_string(),
            port: None,
        };
        let result = NodeId::default().parse(&input);
        assert_eq!(
            result,
            Some(ParseResult {
                result: expected,
                remaining: vec![]
            })
        );
    }

    #[test]
    fn test_parse_node_id_with_remaining() {
        let input = vec![
            ParseBufferItem::Token(Token::Identifier("node1".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Colon)),
            ParseBufferItem::Token(Token::Identifier("port1".to_string())),
            ParseBufferItem::Token(Token::Delimiter(Delimiter::Semicolon)),
        ];
        let expected = NodeId {
            id: "node1".to_string(),
            port: Some(Port {
                id: Some("port1".to_string()),
                compass: None,
            }),
        };
        let result = NodeId::default().parse(&input);
        assert_eq!(
            result,
            Some(ParseResult {
                result: expected,
                remaining: vec![ParseBufferItem::Token(Token::Delimiter(Delimiter::Semicolon))]
            })
        );
    }
}
