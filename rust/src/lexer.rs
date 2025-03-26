use crate::token::*;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };

        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None
        } else {
            self.ch = self.input.chars().nth(self.read_position);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            return None;
        } else {
            return self.input.chars().nth(self.read_position);
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let validated_tok = match self.ch {
            None => TokenType::Eof,
            Some(ch) => match ch {
                '=' => {
                    if self.peek_char().is_some_and(|ch| ch == '=') {
                        self.read_char();
                        TokenType::Eq
                    } else {
                        TokenType::Assign
                    }
                }
                ';' => TokenType::Semicolon,
                '(' => TokenType::LParen,
                ')' => TokenType::RParen,
                '{' => TokenType::LBrace,
                '}' => TokenType::RBrace,
                ',' => TokenType::Comma,
                '+' => TokenType::Plus,
                '-' => TokenType::Minus,
                '!' => {
                    if self.peek_char().is_some_and(|ch| ch == '=') {
                        self.read_char();
                        TokenType::NotEq
                    } else {
                        TokenType::Bang
                    }
                }
                '/' => TokenType::Slash,
                '*' => TokenType::Asterisk,
                '<' => TokenType::LT,
                '>' => TokenType::GT,
                _ => {
                    if ch.is_alphabetic() {
                        let literal = self.read_identifier();
                        let matching_keyword = lookup_ident(literal.clone());
                        return Token {
                            token_type: matching_keyword.unwrap_or(TokenType::Ident(literal)),
                        };
                    } else if ch.is_numeric() {
                        let number = self.read_number();
                        return Token {
                            token_type: TokenType::Int(number),
                        };
                    } else {
                        TokenType::Illegal
                    }
                }
            },
        };

        self.read_char();
        return Token {
            token_type: validated_tok,
        };
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_some_and(|ch| ch.is_whitespace()) {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;

        while self.ch.is_some_and(|ch| ch.is_alphabetic() || ch == '_') {
            self.read_char();
        }

        return self
            .input
            .chars()
            .skip(start_pos)
            .take(self.position - start_pos)
            .collect();
    }

    fn read_number(&mut self) -> i32 {
        let start_pos = self.position;

        while self.ch.is_some_and(|ch| ch.is_numeric()) {
            self.read_char();
        }

        let str = self
            .input
            .chars()
            .skip(start_pos)
            .take(self.position - start_pos)
            .collect::<String>();

        // TODO: can come back to this and give better error handling
        return str.parse::<i32>().unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y
}

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
  return true;
} else {
  return false;
}

10 == 10;
10 != 9;
        "#;

        let mut lex = Lexer::new(input.into());

        use TokenType::*;
        let tests = vec![
            Let,
            Ident("five".into()),
            Assign,
            Int(5),
            Semicolon,
            Let,
            Ident("ten".into()),
            Assign,
            Int(10),
            Semicolon,
            Let,
            Ident("add".into()),
            Assign,
            Functiom,
            LParen,
            Ident("x".into()),
            Comma,
            Ident("y".into()),
            RParen,
            LBrace,
            Ident("x".into()),
            Plus,
            Ident("y".into()),
            RBrace,
            Let,
            Ident("result".into()),
            Assign,
            Ident("add".into()),
            LParen,
            Ident("five".into()),
            Comma,
            Ident("ten".into()),
            RParen,
            Semicolon,
            Bang,
            Minus,
            Slash,
            Asterisk,
            Int(5),
            Semicolon,
            Int(5),
            LT,
            Int(10),
            GT,
            Int(5),
            Semicolon,
            If,
            LParen,
            Int(5),
            LT,
            Int(10),
            RParen,
            LBrace,
            Return,
            True,
            Semicolon,
            RBrace,
            Else,
            LBrace,
            Return,
            False,
            Semicolon,
            RBrace,
            Int(10),
            Eq,
            Int(10),
            Semicolon,
            Int(10),
            NotEq,
            Int(9),
            Semicolon,
            Eof,
        ];

        for expected in tests {
            let tok = lex.next_token();

            assert_eq!(
                tok.token_type, expected,
                "mismatch in token type. expected: {}, got: {}",
                expected, tok.token_type
            );
        }
    }
}
