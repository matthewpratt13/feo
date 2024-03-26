use feo_error::{error::CompilerError, handler::Handler};

use crate::{lexer::Lexer, parser::Parser};

/// Test helper for creating a generic instance of `Parser` with the given source code
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

pub enum LogMsgType {
    Enter,
    Exit,
    Detect,
    Expect,
}

/// Only for use in early stage debugging. All references should be removed as early as possible.
/// Consider replacing this mechanism with something like the `log` crate in production:
/// https://crates.io/crates/log
pub fn log_msg(msg_type: LogMsgType, object_name: &str, parser: &mut Parser) -> () {
    let msg_str = match msg_type {
        LogMsgType::Enter => "entering",
        LogMsgType::Exit => "exit",
        LogMsgType::Detect => "detected",
        LogMsgType::Expect => "expected",
    };

    println!(
        "{msg_str} {object_name}...\ncurrent_token: {:#?}",
        parser.current_token()
    );
}