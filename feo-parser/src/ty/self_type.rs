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
            keyword_kind: KeywordKind::KwSelf,
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
