use crate::tokenizer::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct SubGraph {
    pub id: Option<String>,
    pub statements: Vec<Statement>,
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
    pub attr_stmt_type: AttrStmtType,
    pub items: Vec<Attribute>,
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
    pub id: Option<String>,
    pub compass: Option<Compass>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeId {
    pub id: String,
    pub port: Option<Port>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EdgeStmtSide {
    NodeId(NodeId),
    SubGraph(SubGraph),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EdgeRhs {
    pub edge_op: EdgeOp,
    pub edge_to: EdgeStmtSide,
    pub edge_optional: Option<Box<EdgeRhs>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EdgeStmt {
    pub edge_lhs: EdgeStmtSide,
    pub edge_rhs: EdgeRhs,
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub lhs: String,
    pub rhs: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeStmt {
    pub lhs: String,
    pub rhs: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeStmt {
    pub id: String,
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    NodeStmt(NodeStmt),
    EdgeStmt(EdgeStmt),
    AttrStmt(AttrStmt),
    AttributeStmt(AttributeStmt),
    SubGraph(SubGraph),
}

#[derive(Debug, Clone, PartialEq)]
pub enum GraphType {
    Graph,
    Digraph,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DotGraph {
    pub graph_type: Option<GraphType>,
    pub strict_mode: bool,
    pub id: Option<String>,
    pub statements: Option<Vec<Statement>>,
}

#[derive(Debug)]
pub struct ParserError {
    pub token: Option<Token>,
    pub reason: Option<String>,
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
