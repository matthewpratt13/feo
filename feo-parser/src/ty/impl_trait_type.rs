use feo_ast::{path::PathType, ty::ImplTraitType};
use feo_error::error::CompilerError;
use feo_types::{keyword::KeywordKind, Keyword};

use crate::{parse::ParseType, parser::Parser};

impl ParseType for ImplTraitType {
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

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_impl_trait_type() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"impl Foo"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let impl_trait_type =
            ImplTraitType::parse(&mut parser).expect("unable to parse `impl Trait` type");

        Ok(println!("{:#?}", impl_trait_type))
    }
}
