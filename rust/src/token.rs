#[derive(PartialEq, Debug)]
pub enum TokenType {
    Illegal,
    Eof,

    Ident(String),
    Int(i32),

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LT,
    GT,
    Eq,
    NotEq,

    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    Functiom,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TokenType::*;
        let string: String = match self {
            Illegal => "ILLEGAL".into(),
            Eof => "EOF".into(),

            Int(value) => format!("INT({})", value).into(),
            Ident(name) => format!("IDENT({})", name).into(),

            Assign => "ASSIGN".into(),
            Plus => "PLUS".into(),
            Minus => "MINUS".into(),
            Bang => "BANG".into(),
            Asterisk => "ASTERISK".into(),
            Slash => "SLASH".into(),

            LT => "LT".into(),
            GT => "GT".into(),
            Eq => "EQ".into(),
            NotEq => "NOTEQ".into(),

            Comma => "COMMA".into(),
            Semicolon => "SEMICOLON".into(),

            LParen => "L_PAREN".into(),
            RParen => "R_PAREN".into(),
            LBrace => "L_BRACE".into(),
            RBrace => "R_BRACE".into(),

            Functiom => "FUNCTION".into(),
            Let => "LET".into(),
            True => "TRUE".into(),
            False => "FALSE".into(),
            If => "IF".into(),
            Else => "ELSE".into(),
            Return => "RETURN".into(),
        };

        write!(f, "{}", string)
    }
}

pub fn lookup_ident(ident: String) -> Option<TokenType> {
    match ident.as_str() {
        "fn" => Some(TokenType::Functiom),
        "let" => Some(TokenType::Let),
        "true" => Some(TokenType::True),
        "false" => Some(TokenType::False),
        "if" => Some(TokenType::If),
        "else" => Some(TokenType::Else),
        "return" => Some(TokenType::Return),
        _ => None,
    }
}

pub struct Token {
    pub token_type: TokenType,
}
