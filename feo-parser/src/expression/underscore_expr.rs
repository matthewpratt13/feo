use feo_ast::{expression::UnderscoreExpr, token::Token};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::Identifier;

use crate::{parse::ParseExpr, parser::Parser};

impl ParseExpr for UnderscoreExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            parser.next_token();

            return Ok(Some(UnderscoreExpr(id)));
        }

        parser.log_error(ParserErrorKind::UnexpectedToken {
            expected: "`_`".to_string(),
            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
        });

        Err(parser.errors())
    }
}

#[cfg(test)]
mod tests {
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn parse_underscore_expr() {
        let source_code = r#"_"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let underscore_expr =
            UnderscoreExpr::parse(&mut parser).expect("unable to parse underscore expression");

        println!("{:#?}", &underscore_expr);
    }
}
