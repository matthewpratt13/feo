use feo_ast::expression::{OuterAttr, Struct, StructExprField, StructExprFields};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
    Delimiter, Punctuation,
};

use crate::{parse::Parse, parser::Parser};

impl Parse for Struct {
    fn parse(parser: &mut Parser) -> Option<Self>
    where
        Self: Sized,
    {
        if let Some(d) = Delimiter::try_from(parser.next_token()?).ok() {
            match d.delim {
                (DelimKind::Brace, DelimOrientation::Open) => {}
                _ => todo!(),
            }
        } else {
            todo!()
        }
    }
}
