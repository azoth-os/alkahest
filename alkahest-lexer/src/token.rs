/// Tokenized WebAssembly 3.0 source code representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    Comma,
    Semicolon,

    // Literals.
    Identifier,
    Number,
    String,

    // Keywords.
    Func,
    Param,
    Result,
    Local,
    Global,
    Memory,
    Import,
    Export,
    Module,
    Start,
    Type,
    Table,
    Elem,
    Data,

    // End of file.
    Eof,
}

/// A token produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: &'static str,
    pub line: usize,
    pub column: usize,
}
