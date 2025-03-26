use std::io::{stdin, stdout, Write};

use crate::{lexer::Lexer, token::TokenType};

const PROMPT: &'static str = ">>";

pub fn start() {
    loop {
        let mut line_input = String::new();
        print!("{}", PROMPT);
        Write::flush(&mut stdout()).expect("flush failure");
        stdin()
            .read_line(&mut line_input)
            .expect("Did not enter a correct string");
        if let Some('\n') = line_input.chars().next_back() {
            line_input.pop();
        }
        if let Some('\r') = line_input.chars().next_back() {
            line_input.pop();
        }
        let mut lex = Lexer::new(line_input);
        let mut tok = lex.next_token();

        while tok.token_type != TokenType::Eof {
            println!("{}", tok.token_type);
            tok = lex.next_token();
        }
    }
}
