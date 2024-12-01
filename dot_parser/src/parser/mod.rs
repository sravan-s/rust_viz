use anyhow::{Ok, Result};
use grammer::DotGraph;

mod grammer;
mod parser;
mod parser_a_list;
mod parser_attr_list;
mod parser_attribute_stmt;
mod parser_attribute;
mod parser_compass;
mod parser_head;
mod parser_node_id;
mod parser_port;

use crate::tokenizer::Token;

// Creates an AST from list of tokens
pub fn parse(tokens_vec: &[Token]) -> Result<DotGraph> {
    let dg = parser_head::parse_head(tokens_vec).unwrap();
    let start_idx = match (dg.strict_mode, dg.id.clone()) {
        (true, Some(_)) => 4,
        (false, Some(_)) => 3,
        (true, None) => 3,
        (false, None) => 2,
    };
    let _stmt_tokens = &tokens_vec[start_idx..tokens_vec.len()];
    // dg.statements = parse_stmts::parse_stmts(stmt_tokens);

    Ok(dg)
}
