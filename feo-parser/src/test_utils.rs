use feo_error::{error::CompilerError, handler::Handler};

use crate::{lexer::Lexer, parser::Parser};

pub fn get_parser(source_code: &str, print_stream: bool) -> Result<Parser, Vec<CompilerError>> {
    let handler = Handler::default();

    let mut lexer = Lexer::new(&source_code, handler.clone());

    let token_stream = if let Ok(l) = lexer.lex() {
        l
    } else {
        return Err(lexer.errors());
    };

    match print_stream {
        true => println!("tokens: {:#?}", token_stream),
        false => (),
    }

    Ok(Parser::new(token_stream, handler))
}
