use feo_error::handler::Handler;

use crate::{lexer::Lexer, parser::Parser};

pub fn get_parser(source_code: &str, print_stream: bool) -> Parser {
    let handler = Handler::default();

    let mut lexer = Lexer::new(&source_code, handler.clone());

    let token_stream = lexer.lex().expect("unable to lex source code");

    match print_stream {
        true => println!("tokens: {:#?}", token_stream),
        false => (),
    }

    Parser::new(token_stream, handler)
}
