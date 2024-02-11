use feo_ast::{
    expression::{AttributeKind, OuterAttr},
    path::SimplePath,
};
use feo_error::handler::ErrorEmitted;
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    Delimiter, Keyword, Punctuation,
};

use crate::{parse::Parse, parser::Parser};

impl Parse for AttributeKind {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let attr_kind = if let Some(k) = parser.peek::<Keyword>() {
            match k.keyword_kind {
                KeywordKind::KwAbstract => AttributeKind::KwAbstract(k),
                KeywordKind::KwExport => AttributeKind::KwExport(k),
                KeywordKind::KwExtern => AttributeKind::KwExtern(k),
                KeywordKind::KwUnsafe => AttributeKind::KwUnsafe(k),
                _ => todo!(),
            }
        } else if let Some(p) = SimplePath::parse(parser)? {
            AttributeKind::Path(p)
        } else {
            parser.advance();
            todo!() // return error
        };

        parser.advance();

        Ok(Some(attr_kind))
    }
}

impl Parse for OuterAttr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        let hash_sign_res = parser.peek::<Punctuation>();

        if let Some(Punctuation {
            punc_kind: PuncKind::Hash,
            ..
        }) = hash_sign_res
        {
            parser.advance();

            let open_bracket_res = parser.peek::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_res
            {
                parser.advance();

                if let Some(attr_kind) = AttributeKind::parse(parser)? {
                    let close_bracket_res = parser.peek::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_res
                    {
                        parser.advance();

                        let attr = OuterAttr {
                            hash: hash_sign_res.unwrap(),
                            open_bracket: open_bracket_res.unwrap(),
                            attribute: attr_kind,
                            close_bracket: close_bracket_res.unwrap(),
                        };

                        Ok(Some(attr))
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
}
