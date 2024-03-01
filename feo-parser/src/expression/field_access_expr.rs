use feo_ast::{
    expression::{Assignable, FieldAccessExpr},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{punctuation::PuncKind, Identifier, Punctuation};

use crate::{parse::ParseExpr, parser::Parser};

impl ParseExpr for FieldAccessExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(a) = Assignable::parse(parser)? {
            parser.next_token();

            let full_stop_opt = parser.peek_current::<Punctuation>();

            if let Some(Punctuation {
                punc_kind: PuncKind::FullStop,
                ..
            }) = full_stop_opt
            {
                parser.next_token();

                if let Some(field_name) = parser.peek_current::<Identifier>() {
                    parser.next_token();

                    return Ok(Some(FieldAccessExpr {
                        container_operand: Box::new(a),
                        full_stop: full_stop_opt.unwrap(),
                        field_name,
                    }));
                }

                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`.`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

#[cfg(test)]
mod tests {
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn parse_field_access_expr() {
        let source_code = r#"hello.world"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let field_access_expr =
            FieldAccessExpr::parse(&mut parser).expect("unable to parse field access expr");
    }
}
