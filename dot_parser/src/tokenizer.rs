use std::{char, string};

use anyhow::{ bail, Error, Ok, Result};

// Case insensitve - node, edge, graph, digraph, subgraph, and strict
#[derive(Debug, Clone)]
pub enum Keyword {
    Node,
    Edge,
    Graph,
    Digraph,
    SubGraph,
    Strict,
}

#[derive(Debug, Clone)]
pub enum Delimiter {
    Colon,             // :
    Comma,             // ,
    Semicolon,         // ;
    OpenCurlyBrace,    // {
    ClosedCurlyBrace,  // }
    OpenSquareBrace,   // [
    ClosedSquareBrace, // ]
    Space,             // Space
    Equal,             // =
    UndirectedEdge,    // --
    DirectedEdge,      // ->
    DoubleQuote,       // "
}

#[derive(Debug, Clone)]
pub enum Token {
    // A string of alphabetic ([a-zA-Z\200-\377]) characters, underscores ('_') or digits([0-9]), not beginning with a digit;
    // A numeral [-]?(.[0-9]⁺ | [0-9]⁺(.[0-9]*)? );
    // any double-quoted string ("...") possibly containing escaped quotes (\")¹;
    Identifier(String),
    Keyword(Keyword),
    Delimiter(Delimiter),
}

#[derive(Debug)]
struct TokenizeError {
    line: usize,
    col: usize,
    token: String,
}

impl std::fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error happened at: line {}, col {}, symbol: {} ", self.line, self.col, self.token)
    }
}

fn starts_with_number_and_has_other_ascii(s: &str) -> bool {
    // Check if the first character is a digit and if there are other ASCII characters in the string
    let mut chars = s.chars();
    if let Some(first_char) = chars.next() {
        if first_char.is_ascii_digit() && chars.any(|c| c.is_ascii()) {
            return true;
        }
    }
    false
}

fn chars_to_token(chars: Vec<char>, line: usize, col: usize) -> Result<Token> {
    let word = chars.iter().cloned().collect::<String>();
    let tkn = match word.to_lowercase().as_str() {
        "graph" => Token::Keyword(Keyword::Graph),
        "node" => Token::Keyword(Keyword::Node),
        "edge" => Token::Keyword(Keyword::Edge),
        "digraph" => Token::Keyword(Keyword::Digraph),
        "subgraph" => Token::Keyword(Keyword::SubGraph),
        "strict" => Token::Keyword(Keyword::Strict),
        other => Token::Identifier(other.to_string()),
    };
    let mut valid = true;
    if word.starts_with('\"') && !word.ends_with('\"') {
        valid = false;
    }
    if starts_with_number_and_has_other_ascii(&word) {
        valid = false;
    }
    if !valid {
        bail!(TokenizeError {
            line,
            col,
            token: word, 
        })
    }
    Ok(tkn)
}

fn is_possibly_an_identifier(c: char) -> bool {
    // Check if the character is a lowercase or uppercase letter, extended ASCII, digit, underscore, or period
    c.is_ascii_alphabetic() 
        || ('\u{80}'..='\u{FF}').contains(&c) // Extended ASCII range
        || c.is_ascii_digit()
        || c == '_'
        || c == '.'
}

pub fn tokenize(code: String) -> Result<Vec<Token>> {
    let mut parse_line: usize = 0;
    let mut col: usize = 0;
    let mut token_buffer: Vec<char> = Vec::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut handling_double_quote = false;
    let mut espace_next_char = false;

    let mut possible_edge = false;
    for current_char in code.chars() {
        col += 1;

        if possible_edge {
            if current_char == '-' {
                tokens.push(Token::Delimiter(Delimiter::UndirectedEdge));
                possible_edge = false;
                continue;
            }
            if current_char == '>' {
                tokens.push(Token::Delimiter(Delimiter::DirectedEdge));
                possible_edge = false;
                continue;
            }
            bail!(TokenizeError {
                line: parse_line,
                col,
                token: current_char.to_string(),
            })
        }

        // escape must be processed first
        if current_char == '\\' {
            espace_next_char = true;
            token_buffer.push(current_char);
            continue;
        }
        if espace_next_char {
            espace_next_char = false;
            token_buffer.push(current_char);
            continue;
        }

        // double-quote handling
        if handling_double_quote && current_char != '\"' {
            token_buffer.push(current_char);
            continue;
        }
        if current_char == '\"' && handling_double_quote {
            handling_double_quote = false;
            token_buffer.push(current_char);
            let current_identifier = chars_to_token(token_buffer, parse_line, col)?;
            tokens.push(current_identifier);
            token_buffer = vec![];
            continue;
        }
        if current_char == '\"' && !handling_double_quote {
            handling_double_quote = true;
            let prev_tkn = chars_to_token(token_buffer, parse_line, col)?;
            tokens.push(prev_tkn);
            token_buffer = vec![current_char];
            continue;
        }
        // end double-quote handling

        // possible identifier
        if is_possibly_an_identifier(current_char) {
            token_buffer.push(current_char);
            continue;
        }
        
        // other delimiters
        match current_char {
            // start of quote
            // newline and space are same
            '\n' | ' ' => {
                parse_line += 1;
                col = 0;
                match tokens.last() {
                    // if last one is space, ignore~
                    Some(Token::Delimiter(Delimiter::Space)) => {}
                    _ => {
                        tokens.push(Token::Delimiter(Delimiter::Space));
                    }
                }
            }
            ':' => {
                tokens.push(Token::Delimiter(Delimiter::Colon));
            }
            ',' => {
                tokens.push(Token::Delimiter(Delimiter::Comma));
            }
            ';' => {
                tokens.push(Token::Delimiter(Delimiter::Semicolon));
            }
            '[' => {
                tokens.push(Token::Delimiter(Delimiter::OpenSquareBrace));
            }
            ']' => {
                tokens.push(Token::Delimiter(Delimiter::ClosedSquareBrace));
            }
            '{' => {
                tokens.push(Token::Delimiter(Delimiter::OpenCurlyBrace));
            }
            '}' => {
                tokens.push(Token::Delimiter(Delimiter::ClosedCurlyBrace));
            }
            '=' => {
                tokens.push(Token::Delimiter(Delimiter::Equal));
            }
            '-' => {
                possible_edge = true;
            }
            other => {
                bail!(TokenizeError{
                    col,
                    line: parse_line,
                    token: other.to_string(), 
                })
            }
        }
    }
    Ok(tokens)
}
