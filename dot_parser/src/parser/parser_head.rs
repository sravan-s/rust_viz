use anyhow::{bail, Result};

use crate::{
    parser::grammer::ParserError,
    tokenizer::{Delimiter, Keyword, Token},
};

use super::grammer::{DotGraph, GraphType};

// This one is not parser-combinator for now.. But, I could have ~~
pub fn parse_head(tokens_vec: &[Token]) -> Result<DotGraph> {
    let mut dg = DotGraph {
        graph_type: None,
        strict_mode: false,
        id: None,
        statements: Some(Vec::new()),
    };

    if tokens_vec.len() < 3 {
        bail!(ParserError {
            token: None,
            reason: Some("Need atleast 3 tokens".to_string()),
        });
    }

    let mut tokens = tokens_vec.iter();

    let mut tkn = tokens.next().unwrap().clone();
    if tkn == Token::Keyword(Keyword::Strict) {
        dg.strict_mode = true;
        tkn = tokens.next().unwrap().clone();
    }
    match tkn {
        Token::Keyword(Keyword::Graph) => {
            dg.graph_type = Some(GraphType::Graph);
        }
        Token::Keyword(Keyword::Digraph) => {
            dg.graph_type = Some(GraphType::Digraph);
        }
        _ => {
            bail!(ParserError {
                token: Some(tkn),
                reason: Some("Grpah should start with Keywords: strict/graph/digraph".to_string()),
            });
        }
    }

    tkn = tokens.next().unwrap().clone();
    match tkn {
        Token::Identifier(id) => {
            dg.id = Some(id);
            tkn = tokens.next().unwrap().clone();
            if tkn != Token::Delimiter(Delimiter::OpenCurlyBrace) {
                bail!(ParserError {
                    token: Some(tkn),
                    reason: Some("Expected { after graph's name".to_string()),
                });
            }
        }
        Token::Delimiter(Delimiter::OpenCurlyBrace) => {
            dg.id = None;
        }
        _ => {
            bail!(ParserError {
                token: Some(tkn),
                reason: Some(
                    "After graph/digraph, we expect graph's name or open brace".to_string()
                ),
            });
        }
    }

    let last = tokens.last().unwrap().clone();
    if last != Token::Delimiter(Delimiter::ClosedCurlyBrace) {
        bail!(ParserError {
            token: Some(last),
            reason: Some("Expected } at the end".to_string())
        });
    }

    Ok(dg)
}
