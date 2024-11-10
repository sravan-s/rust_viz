use regex::Regex;
use std::char;

use anyhow::{bail, Ok, Result};

// Case insensitve - node, edge, graph, digraph, subgraph, and strict
#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Node,
    Edge,
    Graph,
    Digraph,
    SubGraph,
    Strict,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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
    reason: Option<String>,
}

impl std::fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error happened at: line {}, col {}, symbol: {} \n Reason: {:?} \n",
            self.line, self.col, self.token, self.reason
        )
    }
}

fn is_proper_identifier(s: &str, line: usize, col: usize) -> Result<()> {
    if s.len() == 1 {
        let s: char = s.chars().next().unwrap();
        let result = s.is_ascii_alphabetic() || // Checks a-z, A-Zu
            ('\u{80}'..='\u{FF}').contains(&s) || // Checks extended ASCII \u{80} to \u{FF}
            s.is_ascii_digit();
        if !result {
            bail!(TokenizeError {
                line,
                col,
                token: s.to_string(),
                reason: Some("Invalid single character".to_string()),
            })
        }
        return Ok(());
    }
    // "" -> empty string
    if s.eq("\"\"") {
        bail!(TokenizeError {
            line,
            col,
            token: s.to_string(),
            reason: Some("Empty quotes".to_string()),
        });
    }
    let alphabetic_id = Regex::new(r"^[a-zA-Z\x80-\xFF_][a-zA-Z\x80-\xFF_0-9]*$").unwrap();
    let numeral_id = Regex::new(r"^-?(?:\.[0-9]+|[0-9]+(?:\.[0-9]*)?)$").unwrap();
    let quoted_string_id = Regex::new(r#"^"([^"\\]|\\.)*"$"#).unwrap();

    let result =
        alphabetic_id.is_match(s) || numeral_id.is_match(s) || quoted_string_id.is_match(s);
    if !result {
        bail!(TokenizeError {
            line,
            col,
            token: s.to_string(),
            reason: Some("Invalid identifier".to_string()),
        })
    }
    Ok(())
}

// note - this flowcan be made more idiomatic ~
// split into -> fn identify_keyword() & fn convert_to_idntifier()
fn chars_to_token(chars: Vec<char>, line: usize, col: usize) -> Result<Option<Token>> {
    if chars.is_empty() {
        return Ok(None);
    }
    let word = chars.iter().cloned().collect::<String>();
    // keywords
    let tkn = match word.to_lowercase().as_str() {
        "graph" => Token::Keyword(Keyword::Graph),
        "node" => Token::Keyword(Keyword::Node),
        "edge" => Token::Keyword(Keyword::Edge),
        "digraph" => Token::Keyword(Keyword::Digraph),
        "subgraph" => Token::Keyword(Keyword::SubGraph),
        "strict" => Token::Keyword(Keyword::Strict),
        _ => {
            let mut word: String = chars.iter().collect();
            is_proper_identifier(&word, line, col)?;
            // remove first and last quote
            if word.starts_with('"') && word.ends_with('"') {
                word.pop();
                word.remove(0);
            }
            Token::Identifier(word)
        }
    };
    Ok(Some(tkn))
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
        /*
        println!(
            "current_char: {}, line: {}, col: {}",
            current_char, parse_line, col
        );
        println!("tokens: {:?}", tokens);
        println!("prev_buffer: {:?} \n\n\n", token_buffer);
        */
        col += 1;

        if possible_edge {
            // remove last item, it is a optimistic Delimiter::UndirectedEdge
            tokens.pop();
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
                reason: Some("Invalid edge, expected - or >".to_string()),
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
            if let Some(identifier) = current_identifier {
                tokens.push(identifier);
            }
            token_buffer = vec![];
            continue;
        }
        if current_char == '\"' && !handling_double_quote {
            handling_double_quote = true;
            let prev_tkn = chars_to_token(token_buffer, parse_line, col)?;
            if let Some(identifier) = prev_tkn {
                tokens.push(identifier);
            }

            token_buffer = vec![current_char];
            continue;
        }
        // end double-quote handling

        // other delimiters
        let delim = match current_char {
            // start of quote
            // newline and space are same
            '\n' => {
                parse_line += 1;
                col = 0;
                Some(Token::Delimiter(Delimiter::Space))
            }
            ' ' => Some(Token::Delimiter(Delimiter::Space)),
            ':' => Some(Token::Delimiter(Delimiter::Colon)),
            ',' => Some(Token::Delimiter(Delimiter::Comma)),
            ';' => Some(Token::Delimiter(Delimiter::Semicolon)),
            '[' => Some(Token::Delimiter(Delimiter::OpenSquareBrace)),
            ']' => Some(Token::Delimiter(Delimiter::ClosedSquareBrace)),
            '{' => Some(Token::Delimiter(Delimiter::OpenCurlyBrace)),
            '}' => Some(Token::Delimiter(Delimiter::ClosedCurlyBrace)),
            '=' => Some(Token::Delimiter(Delimiter::Equal)),
            '-' => {
                possible_edge = true;
                // this will be over_written in the delimiter if/else
                Some(Token::Delimiter(Delimiter::UndirectedEdge))
            }
            _ => None,
        };
        match delim {
            Some(delimiter) => {
                let prev_tkn = chars_to_token(token_buffer, parse_line, col)?;
                if let Some(identifier) = prev_tkn {
                    tokens.push(identifier);
                }
                // reset token_buffer
                token_buffer = vec![];
                // combine multiple spaces(and newline) to one
                /*let mut skip_space = false;
                if let Some(last_token) = tokens.last() {
                    skip_space = delimiter == Token::Delimiter(Delimiter::Space)
                        && *last_token == Token::Delimiter(Delimiter::Space);
                }
                */
                // In dot language, spaces are not syntatically meaningful
                // They are only useful inside quoted strings
                // So, we skip spaces
                if delimiter != Token::Delimiter(Delimiter::Space) {
                    tokens.push(delimiter);
                }
            }
            _ => {
                token_buffer.push(current_char);
            }
        };
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_proper_identifier_alphabetic_ids() {
        // Valid alphabetic IDs
        assert!(is_proper_identifier("hello_world", 0, 0).is_ok());
        assert!(is_proper_identifier("_underscore", 0, 0).is_ok());
        assert!(is_proper_identifier("validID123", 0, 0).is_ok());
        assert!(is_proper_identifier("ü123", 0, 0).is_ok()); // Extended ASCII start, valid

        // Invalid alphabetic IDs (start with a digit)
        assert!(is_proper_identifier("123abc", 0, 0).is_err());
        assert!(is_proper_identifier("1underscore", 0, 0).is_err());
    }

    #[test]
    fn test_is_proper_identifier_numeral_ids() {
        // Valid numerals
        assert!(is_proper_identifier("123", 0, 0).is_ok());
        assert!(is_proper_identifier("-123", 0, 0).is_ok());
        assert!(is_proper_identifier(".5", 0, 0).is_ok());
        assert!(is_proper_identifier("42.42", 0, 0).is_ok());
        assert!(is_proper_identifier("-0.12345", 0, 0).is_ok());

        // Invalid numerals (malformed)
        assert!(is_proper_identifier("123abc", 0, 0).is_err()); // Starts like a number but includes non-numeral characters
        assert!(is_proper_identifier("--123", 0, 0).is_err()); // Multiple negative signs
        assert!(is_proper_identifier("-.5.", 0, 0).is_err()); // Extra decimal point
    }

    #[test]
    fn test_is_proper_identifier_quoted_string_ids() {
        // Valid quoted strings
        assert!(is_proper_identifier("\"quoted\"", 0, 0).is_ok());
        assert!(is_proper_identifier("no_quotes", 0, 0).is_ok());
        assert!(is_proper_identifier("\"\"", 0, 0).is_err()); // Empty quoted string
        assert!(is_proper_identifier("\"unmatched quote", 0, 0).is_err());
        assert!(is_proper_identifier("\"missing end escape\\", 0, 0).is_err());
    }

    #[test]
    fn test_is_proper_identifier_edge_cases() {
        // Empty string should be invalid
        assert!(is_proper_identifier("", 0, 0).is_err());

        // Single characters - valid
        assert!(is_proper_identifier("5", 0, 0).is_ok());
        assert!(is_proper_identifier("a", 0, 0).is_ok());
        assert!(is_proper_identifier("\"a\"", 0, 0).is_ok());

        // others
        assert!(is_proper_identifier("_", 0, 0).is_err());
        assert!(is_proper_identifier("\"", 0, 0).is_err()); // Unmatched quote
    }
    #[test]
    fn test_chars_to_case_insensitive_token_keywords() {
        // Keywords should be identified correctly, ignoring case
        let keywords = [
            ("graph", Keyword::Graph),
            ("NODE", Keyword::Node),
            ("EdGE", Keyword::Edge),
            ("digraph", Keyword::Digraph),
            ("SUBGRAPH", Keyword::SubGraph),
        ];
        for (keyword, expected_token) in keywords.iter() {
            let keyword_tkn = chars_to_token(keyword.chars().collect(), 0, 0);
            match keyword_tkn {
                Result::Ok(Some(tkn)) => {
                    assert_eq!(
                        tkn,
                        Token::Keyword(expected_token.clone()),
                        "Failed on keyword: {}",
                        keyword
                    );
                }
                _ => {
                    panic!(
                        "Expected Ok({:?}), Recived: {:?}",
                        expected_token,
                        keyword_tkn.unwrap()
                    );
                }
            }
        }
    }

    #[test]
    fn test_chars_to_token_identifiers() {
        let keywords = ["my_id", "_abc", "value_1", "value2", "Ü123df"];
        for identifier in keywords.iter() {
            let keyword_tkn = chars_to_token(identifier.chars().collect(), 0, 0);
            let expected = Token::Identifier(identifier.to_string());
            match keyword_tkn {
                Result::Ok(Some(tkn)) => {
                    assert_eq!(
                        tkn, expected,
                        "Expected: {:?} ... Recived: {:?}",
                        identifier, tkn
                    );
                }
                _ => {
                    panic!(
                        "Expected Ok({:?}), Recived: {:?}",
                        expected,
                        keyword_tkn.unwrap()
                    );
                }
            }
        }
    }

    #[test]
    fn test_chars_to_token_quoted_identifiers() {
        assert_eq!(
            chars_to_token("\"quo ted\"".chars().collect(), 0, 0)
                .unwrap()
                .unwrap(),
            Token::Identifier("quo ted".to_string())
        );
        // todo: check this case -> I suspect this is just display issue
        // should be okay when I render
        assert_eq!(
            chars_to_token(vec!['"', 'q', '\\', '"', 'u', '"'], 0, 0)
                .unwrap()
                .unwrap(),
            Token::Identifier("q\\\"u".to_string())
        );
    }

    #[test]
    fn test_chars_to_token_invalid_identifiers() {
        // Invalid identifiers should return an error
        assert_eq!(
            chars_to_token(vec!['1', '2', '3'], 0, 0).unwrap().unwrap(),
            Token::Identifier("123".to_string())
        ); // Starts with a digit
        assert!(chars_to_token(vec!['!', 'i', 'd'], 0, 0).is_err()); // Contains invalid character
        assert!(chars_to_token(vec!['a', '!', 'b'], 0, 0).is_err()); // Contains invalid character
    }

    #[test]
    fn test_tokenize_basic_1() {
        let code = "graph { a -- b; b -- c; }".to_string();
        let tokens = tokenize(code).unwrap();
        let expected = vec![
            Token::Keyword(Keyword::Graph),
            Token::Delimiter(Delimiter::OpenCurlyBrace),
            Token::Identifier("a".to_string()),
            Token::Delimiter(Delimiter::UndirectedEdge),
            Token::Identifier("b".to_string()),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Identifier("b".to_string()),
            Token::Delimiter(Delimiter::UndirectedEdge),
            Token::Identifier("c".to_string()),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Delimiter(Delimiter::ClosedCurlyBrace),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_basic_2() {
        let code = "digraph { a -> b; b -> c; }".to_string();
        let tokens = tokenize(code).unwrap();
        let expected = vec![
            Token::Keyword(Keyword::Digraph),
            Token::Delimiter(Delimiter::OpenCurlyBrace),
            Token::Identifier("a".to_string()),
            Token::Delimiter(Delimiter::DirectedEdge),
            Token::Identifier("b".to_string()),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Identifier("b".to_string()),
            Token::Delimiter(Delimiter::DirectedEdge),
            Token::Identifier("c".to_string()),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Delimiter(Delimiter::ClosedCurlyBrace),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_with_edge_attributes() {
        let code = "graph G {
            A -- B [label=\"edge label\"];
            B [color=blue];
            C [shape=circle];
        }"
        .to_string();
        let tokens = tokenize(code).unwrap();
        let expected = vec![
            Token::Keyword(Keyword::Graph),
            Token::Identifier("G".to_string()),
            Token::Delimiter(Delimiter::OpenCurlyBrace),
            Token::Identifier("A".to_string()),
            Token::Delimiter(Delimiter::UndirectedEdge),
            Token::Identifier("B".to_string()),
            Token::Delimiter(Delimiter::OpenSquareBrace),
            Token::Identifier("label".to_string()),
            Token::Delimiter(Delimiter::Equal),
            Token::Identifier("edge label".to_string()),
            Token::Delimiter(Delimiter::ClosedSquareBrace),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Identifier("B".to_string()),
            Token::Delimiter(Delimiter::OpenSquareBrace),
            Token::Identifier("color".to_string()),
            Token::Delimiter(Delimiter::Equal),
            Token::Identifier("blue".to_string()),
            Token::Delimiter(Delimiter::ClosedSquareBrace),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Identifier("C".to_string()),
            Token::Delimiter(Delimiter::OpenSquareBrace),
            Token::Identifier("shape".to_string()),
            Token::Delimiter(Delimiter::Equal),
            Token::Identifier("circle".to_string()),
            Token::Delimiter(Delimiter::ClosedSquareBrace),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Delimiter(Delimiter::ClosedCurlyBrace),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_with_node_attributes() {
        let code = "graph G {
            A [label=\"node label\"];
            B [color=blue];
            C [shape=circle];
        }"
        .to_string();
        let tokens = tokenize(code).unwrap();
        let expected = vec![
            Token::Keyword(Keyword::Graph),
            Token::Identifier("G".to_string()),
            Token::Delimiter(Delimiter::OpenCurlyBrace),
            Token::Identifier("A".to_string()),
            Token::Delimiter(Delimiter::OpenSquareBrace),
            Token::Identifier("label".to_string()),
            Token::Delimiter(Delimiter::Equal),
            Token::Identifier("node label".to_string()),
            Token::Delimiter(Delimiter::ClosedSquareBrace),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Identifier("B".to_string()),
            Token::Delimiter(Delimiter::OpenSquareBrace),
            Token::Identifier("color".to_string()),
            Token::Delimiter(Delimiter::Equal),
            Token::Identifier("blue".to_string()),
            Token::Delimiter(Delimiter::ClosedSquareBrace),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Identifier("C".to_string()),
            Token::Delimiter(Delimiter::OpenSquareBrace),
            Token::Identifier("shape".to_string()),
            Token::Delimiter(Delimiter::Equal),
            Token::Identifier("circle".to_string()),
            Token::Delimiter(Delimiter::ClosedSquareBrace),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Delimiter(Delimiter::ClosedCurlyBrace),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_tokenize_with_escaped_quotes() {
        let code = "graph G {
            A [label=\"ain\\\"t it\"];
            B [label=\"node label\"];
        }"
        .to_string();
        let tokens = tokenize(code).unwrap();
        let expected = vec![
            Token::Keyword(Keyword::Graph),
            Token::Identifier("G".to_string()),
            Token::Delimiter(Delimiter::OpenCurlyBrace),
            Token::Identifier("A".to_string()),
            Token::Delimiter(Delimiter::OpenSquareBrace),
            Token::Identifier("label".to_string()),
            Token::Delimiter(Delimiter::Equal),
            Token::Identifier("ain\\\"t it".to_string()),
            Token::Delimiter(Delimiter::ClosedSquareBrace),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Identifier("B".to_string()),
            Token::Delimiter(Delimiter::OpenSquareBrace),
            Token::Identifier("label".to_string()),
            Token::Delimiter(Delimiter::Equal),
            Token::Identifier("node label".to_string()),
            Token::Delimiter(Delimiter::ClosedSquareBrace),
            Token::Delimiter(Delimiter::Semicolon),
            Token::Delimiter(Delimiter::ClosedCurlyBrace),
        ];
        assert_eq!(tokens, expected);
    }
}
