use anyhow::{Ok, Result};

pub mod tokenizer;

#[derive(Debug, Clone)]
pub struct SubGraph {
    id: Option<String>,
    statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum EdgeOp {
    Directed,
    UnDirected,
}

#[derive(Debug, Clone)]
pub enum AttrStmtType {
    Graph,
    Node,
    Edge,
}

#[derive(Debug, Clone)]
pub struct AttrStmt {
    attr_stmt_type: AttrStmtType,
    items: Vec<Attribute>,
}

#[derive(Debug, Clone)]
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
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Port {
    id: String,
    compass: Compass,
}

#[derive(Debug, Clone)]
pub struct NodeId {
    id: String,
    port: Option<Port>,
}

#[derive(Debug, Clone)]
pub enum EdgeStmtSide {
    NodeId(NodeId),
    SubGraph(SubGraph),
}

#[derive(Debug, Clone)]
pub struct EdgeRhs {
    edge_op: EdgeOp,
    edge_to: EdgeStmtSide,
    edge_optional: Option<Box<EdgeRhs>>,
}

#[derive(Debug, Clone)]
pub struct EdgeStmt {
    edge_lhs: EdgeStmtSide,
    edge_rhs: EdgeRhs,
    attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    lhs: String,
    rhs: String,
}

#[derive(Debug, Clone)]
pub struct NodeStmt {
    id: String,
    attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    NodeStmt(NodeStmt),
    EdgeStmt(EdgeStmt),
    AttrStmt(AttrStmt),
    Attribute(Attribute),
    SubGraph(SubGraph),
}

#[derive(Debug, Clone)]
pub enum GraphType {
    Graph,
    Digraph,
}

#[derive(Debug, Clone)]
pub struct DotGraph {
    graph_type: GraphType,
    strict_mode: bool,
    id: Option<String>,
    statements: Vec<Statement>,
}

pub fn parse(code: String) -> Result<DotGraph> {
    let dg = DotGraph {
        graph_type: GraphType::Graph,
        strict_mode: false,
        id: None,
        statements: Vec::new(),
    };
    Ok(dg)
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
