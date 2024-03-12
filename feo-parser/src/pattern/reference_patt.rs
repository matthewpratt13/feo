use feo_ast::pattern::{PatternWithoutRange, ReferencePatt};
use feo_error::error::CompilerError;
use feo_types::{keyword::KeywordKind, punctuation::PuncKind, Keyword, Punctuation};

use crate::{parse::ParsePatt, parser::Parser};

impl ParsePatt for ReferencePatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let ampersand_opt = parser.peek_current::<Punctuation>();

        if let Some(Punctuation {
            punc_kind: PuncKind::Ampersand,
            ..
        }) = ampersand_opt
        {
            let kw_mut_opt = parser.peek_current::<Keyword>();

            if let Some(Keyword {
                keyword_kind: KeywordKind::KwMut,
                ..
            }) = kw_mut_opt
            {
                parser.next_token();
            } else {
                if let Some(pattern) = PatternWithoutRange::parse(parser)? {
                    parser.next_token();

                    return Ok(Some(ReferencePatt {
                        ampersand: ampersand_opt.unwrap(),
                        kw_mut_opt,
                        pattern: Box::new(pattern),
                    }));
                }
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}
