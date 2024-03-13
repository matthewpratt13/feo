use feo_ast::ty::SelfType;
use feo_error::error::CompilerError;
use feo_types::{keyword::KeywordKind, Keyword};

use crate::{parse::ParseType, parser::Parser};

impl ParseType for SelfType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let kw_self_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwSelfType,
            ..
        }) = kw_self_opt
        {
            parser.next_token();

            return Ok(Some(SelfType(kw_self_opt.unwrap())));
        } else {
            return Ok(None);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_self_type() {
        let source_code = r#"Self"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let self_type = SelfType::parse(&mut parser).expect("unable to parse Self type");

        println!("{:#?}", self_type);
    }
}
