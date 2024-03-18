use feo_error::error::CompilerError;
use feo_types::{punctuation::PuncKind, Punctuation};

use crate::parser::Parser;

pub fn skip_trailing_comma(parser: &mut Parser) -> Result<(), Vec<CompilerError>> {
    if let Some(Punctuation {
        punc_kind: PuncKind::Comma,
        ..
    }) = parser.peek_current::<Punctuation>()
    {
        parser.next_token();
        Ok(())
    } else {
        Ok(())
    }
}
