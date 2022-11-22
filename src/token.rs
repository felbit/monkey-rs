//! src/token.rs

const ILLEGAL: &'static str = "ILLEGAL";
const EOF: &'static str = "EOF";

// identifiers & literals
const IDENT: &'static str = "IDENT";
const INT: &'static str = "INT";

// operators
const ASSIGN: &'static str = "=";
const PLUS: &'static str = "+";

// delimiters
const COMMA: &'static str = ",";
const SEMICOLON: &'static str = ";";
const LPAREN: &'static str = "(";
const RPAREN: &'static str = ")";
const LBRACE: &'static str = "{";
const RBRACE: &'static str = "}";

// keywords
const FUNCTION: &'static str = "FUNCTION";
const LET: &'static str = "LET";

pub struct Token {
    pub token_type: String,
    pub literal: String,
}
