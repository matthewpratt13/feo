use feo_ast::pattern::WildcardPatt;
use feo_error::error::CompilerError;
use feo_types::Identifier;

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for WildcardPatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(underscore) = parser.peek_current::<Identifier>() {
            Ok(Some(WildcardPatt(underscore)))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn parse_wildcard_patt() {
        let source_code = r#"_"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let wildcard_patt =
            WildcardPatt::parse(&mut parser).expect("unable to parse wildcard pattern");

        println!("{:#?}", wildcard_patt);
    }
}
