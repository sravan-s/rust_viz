use anyhow::{bail, Ok, Result};

use crate::tokenizer::{Delimiter, Keyword, Token};

#[derive(Debug, Clone, PartialEq)]
pub struct SubGraph {
    id: Option<String>,
    statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EdgeOp {
    Directed,
    UnDirected,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttrStmtType {
    Graph,
    Node,
    Edge,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttrStmt {
    attr_stmt_type: AttrStmtType,
    items: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Compass {
    N,
    Ne,
    E,
    Se,
    S,
    Sw,
    W,
    Nw,
    C,
    Underscore,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Port {
    id: Option<String>,
    compass: Option<Compass>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeId {
    id: String,
    port: Option<Port>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EdgeStmtSide {
    NodeId(NodeId),
    SubGraph(SubGraph),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EdgeRhs {
    edge_op: EdgeOp,
    edge_to: EdgeStmtSide,
    edge_optional: Option<Box<EdgeRhs>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EdgeStmt {
    edge_lhs: EdgeStmtSide,
    edge_rhs: EdgeRhs,
    attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    lhs: String,
    rhs: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeStmt {
    id: String,
    attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    NodeStmt(NodeStmt),
    EdgeStmt(EdgeStmt),
    AttrStmt(AttrStmt),
    Attribute(Attribute),
    SubGraph(SubGraph),
}

#[derive(Debug, Clone, PartialEq)]
pub enum GraphType {
    Graph,
    Digraph,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DotGraph {
    graph_type: Option<GraphType>,
    strict_mode: bool,
    id: Option<String>,
    statements: Vec<Statement>,
}

// compass_pt 	: 	n | ne | e | se | s | sw | w | nw | c | _
fn match_compass(token: &Token) -> Option<Compass> {
    match token {
        Token::Identifier(ref s) => match s.as_str() {
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
        },
        _ => None,
    }
}

/**
 * port : ':' ID [ ':' compass_pt ]
 * | ':' compass_pt
 */
fn match_port(tokens: &[Token]) -> Option<Port> {
    if tokens.len() < 2 {
        return None;
    }
    let first = tokens.first()?;
    if *first != Token::Delimiter(Delimiter::Colon) {
        return None;
    }

    if tokens.len() == 2 {
        let second = tokens.get(1)?;
        let second_as_compass = match_compass(second);
        if second_as_compass.is_some() {
            return Some(Port {
                id: None,
                compass: Some(second_as_compass?),
            });
        }

        if let Token::Identifier(ref id) = second {
            return Some(Port {
                id: Some(id.clone()),
                compass: None,
            });
        }
    }

    if tokens.len() == 4 {
        let second = tokens.get(1)?;
        let third = tokens.get(2)?;
        let fourth = tokens.get(3)?;
        let fourth_as_compass = match_compass(fourth);
        match (second, third, fourth_as_compass) {
            (Token::Identifier(ref id), Token::Delimiter(Delimiter::Colon), Some(compass)) => {
                return Some(Port {
                    id: Some(id.clone()),
                    compass: Some(compass),
                });
            }
            _ => {
                return None;
            }
        };
    }

    None
}

pub fn list_stmts(tokens: &[Token]) -> Result<Vec<Statement>> {
    let stmt: Vec<Statement> = vec![];
    if tokens.is_empty() {
        return Ok(stmt);
    }
    Ok(stmt)
}

#[derive(Debug)]
struct ParserError {
    token: Option<Token>,
    reason: Option<String>,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error happened at: token: {:?}, \n Reason: {:?} \n",
            self.token, self.reason
        )
    }
}

// Creates an AST from list of tokens
pub fn parse(tokens_vec: &Vec<Token>) -> Result<DotGraph> {
    let mut dg = DotGraph {
        graph_type: None,
        strict_mode: false,
        id: None,
        statements: Vec::new(),
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

    let start_idx = match (dg.strict_mode, dg.id.clone()) {
        (true, Some(_)) => 4,
        (false, Some(_)) => 3,
        (true, None) => 3,
        (false, None) => 2,
    };
    let stmt_tokens = &tokens_vec[start_idx..tokens_vec.len()];

    println!("{:?}", stmt_tokens);
    let stmts = list_stmts(tokens_vec)?;

    dg.statements = stmts;

    Ok(dg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_port_false() {
        let tokens = vec![Token::Delimiter(Delimiter::Colon)];
        let res = match_port(&tokens);
        assert!(res.is_none());

        let tokens = vec![
            Token::Delimiter(Delimiter::Colon),
        ];
        let res = match_port(&tokens);
        assert!(res.is_none());

        let tokens = vec![
            Token::Delimiter(Delimiter::Colon),
            Token::Identifier("n".to_string()),
            Token::Identifier("n".to_string()),
        ];
        let res = match_port(&tokens);
        assert!(res.is_none());

        let tokens = vec![
            Token::Delimiter(Delimiter::Colon),
            Token::Identifier("n".to_string()),
            Token::Delimiter(Delimiter::Colon),
            Token::Identifier("n".to_string()),
            Token::Identifier("n".to_string()),
        ];
        let res = match_port(&tokens);
        assert!(res.is_none());
    }

    #[test]
    fn parser_success_simple_strict_mode() {
        let tokens = vec![
            Token::Keyword(Keyword::Strict),
            Token::Keyword(Keyword::Graph),
            Token::Identifier("G".to_string()),
            Token::Delimiter(Delimiter::OpenCurlyBrace),
            Token::Delimiter(Delimiter::ClosedCurlyBrace),
        ];
        let res = parse(&tokens);
        assert!(res.is_ok());
    }

    #[test]
    fn parser_success_simple_no_strict_mode() {
        let tokens = vec![
            Token::Keyword(Keyword::Graph),
            Token::Identifier("G".to_string()),
            Token::Delimiter(Delimiter::OpenCurlyBrace),
            Token::Delimiter(Delimiter::ClosedCurlyBrace),
        ];
        let res = parse(&tokens);
        assert!(res.is_ok());
    }

    #[test]
    fn parser_success_strict_mode_with_name() {
        let tokens = vec![
            Token::Keyword(Keyword::Strict),
            Token::Keyword(Keyword::Graph),
            Token::Identifier("G".to_string()),
            Token::Delimiter(Delimiter::OpenCurlyBrace),
            Token::Delimiter(Delimiter::ClosedCurlyBrace),
        ];
        let res = parse(&tokens);
        assert!(res.is_ok());
    }

    #[test]
    fn parser_success_strict_mode_without_name() {
        let tokens = vec![
            Token::Keyword(Keyword::Strict),
            Token::Keyword(Keyword::Graph),
            Token::Delimiter(Delimiter::OpenCurlyBrace),
            Token::Delimiter(Delimiter::ClosedCurlyBrace),
        ];
        let res = parse(&tokens);
        assert!(res.is_ok());
    }

    #[test]
    fn parser_success_no_strict_mode_wihtout_name() {
        let tokens = vec![
            Token::Keyword(Keyword::Graph),
            Token::Delimiter(Delimiter::OpenCurlyBrace),
            Token::Delimiter(Delimiter::ClosedCurlyBrace),
        ];
        let res = parse(&tokens);
        assert!(res.is_ok());
    }

    #[test]
    fn parser_success_no_strict_mode_with_name() {
        let tokens = vec![
            Token::Keyword(Keyword::Graph),
            Token::Identifier("G".to_string()),
            Token::Delimiter(Delimiter::OpenCurlyBrace),
            Token::Delimiter(Delimiter::ClosedCurlyBrace),
        ];
        let res = parse(&tokens);
        assert!(res.is_ok());
    }

    #[test]
    fn parser_fail() {
        let tokens = vec![];
        let res = parse(&tokens);
        assert!(res.is_err());
    }

    #[test]
    fn parser_fail_with_less_tokens() {
        let tokens = vec![Token::Keyword(Keyword::Graph)];
        let res = parse(&tokens);
        assert!(res.is_err());
    }

    #[test]
    fn parser_fail_with_wrong_start() {
        let tokens = vec![
            Token::Delimiter(Delimiter::OpenCurlyBrace),
            Token::Keyword(Keyword::Graph),
            Token::Identifier("G".to_string()),
            Token::Keyword(Keyword::Strict),
        ];
        let res = parse(&tokens);
        assert!(res.is_err());
    }

    #[test]
    fn parser_fail_with_no_open_curly_brace() {
        let tokens = vec![
            Token::Keyword(Keyword::Graph),
            Token::Identifier("G".to_string()),
            Token::Keyword(Keyword::Strict),
            Token::Identifier("G".to_string()),
        ];
        let res = parse(&tokens);
        assert!(res.is_err());
    }

    #[test]
    fn parser_fail_with_no_closed_curly_brace() {
        let tokens = vec![
            Token::Keyword(Keyword::Graph),
            Token::Identifier("G".to_string()),
            Token::Keyword(Keyword::Strict),
            Token::Delimiter(Delimiter::OpenCurlyBrace),
        ];
        let res = parse(&tokens);
        assert!(res.is_err());
    }
}
