use feo_ast::{path::PathType, ty::ImplTraitType};
use feo_error::error::CompilerError;
use feo_types::{keyword::KeywordKind, Keyword};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for ImplTraitType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_impl_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwImpl,
            ..
        }) = kw_impl_opt
        {
            parser.next_token();

            if let Some(trait_bound) = PathType::parse(parser)? {
                return Ok(Some(ImplTraitType {
                    kw_impl: kw_impl_opt.unwrap(),
                    trait_bound,
                }));
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
    fn parse_impl_trait_type() {
        let source_code = r#"impl Foo"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let impl_trait_type_type = ImplTraitType::parse(&mut parser).expect("unable to parse impl trait type");

        println!("{:#?}", impl_trait_type_type);
    }
}
