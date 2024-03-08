use feo_ast::pattern::IdentifierPatt;
use feo_error::error::CompilerError;
use feo_types::{keyword::KeywordKind, Identifier, Keyword};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for IdentifierPatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_ref_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwRef,
            ..
        }) = kw_ref_opt
        {
            parser.next_token();
        }

        let kw_mut_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwMut,
            ..
        }) = kw_mut_opt
        {
            parser.next_token();
        }

        if let Some(name) = parser.peek_current::<Identifier>() {
            parser.next_token();

            return Ok(Some(IdentifierPatt {
                kw_ref_opt,
                kw_mut_opt,
                name,
            }));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn parse_identifier_patt() {
        let source_code = r#"ref mut foo"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let identifier_patt =
            IdentifierPatt::parse(&mut parser).expect("unable to parse identifier pattern");

        println!("{:#?}", identifier_patt);
    }
}
