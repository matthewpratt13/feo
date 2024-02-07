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
        // TODO: replace with parser.peek()
        let attr_kind = if let Ok(k) = Keyword::try_from(parser.current_token()) {
            parser.advance();

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
            todo!()
        };

        Ok(Some(attr_kind))
    }
}

impl Parse for OuterAttr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        // TODO: replace with parser.peek()
        let hash_sign_res = Punctuation::try_from(parser.current_token());

        if let Ok(Punctuation {
            punc_kind: PuncKind::Hash,
            ..
        }) = hash_sign_res
        {
            parser.advance();

            // TODO: replace with parser.peek()
            let open_bracket_res = Delimiter::try_from(parser.current_token());

            if let Ok(Delimiter {
                delim: (DelimKind::Bracket, DelimOrientation::Open),
                ..
            }) = open_bracket_res
            {
                parser.advance();

                if let Some(attr_kind) = AttributeKind::parse(parser)? {
                    // TODO: replace with parser.peek()
                    let close_bracket_res = Delimiter::try_from(parser.current_token());

                    if let Ok(Delimiter {
                        delim: (DelimKind::Bracket, DelimOrientation::Close),
                        ..
                    }) = close_bracket_res
                    {
                        // consume last token and move to next token in prep for next parser
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
