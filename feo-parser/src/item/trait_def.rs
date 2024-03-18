use feo_ast::{
    item::{
        ConstantVarDef, FunctionSig, FunctionWithBlock, TraitDef, TraitDefItem, TypeAliasDef,
        TypeParamBounds, VisibilityKind, WhereClause,
    },
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    Delimiter, Identifier, Keyword, Punctuation,
};

use crate::{
    parse::{ParseItem, ParseTerm},
    parser::Parser,
    utils,
};

impl ParseItem for TraitDefItem {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(c) = ConstantVarDef::parse(parser)? {
            return Ok(Some(TraitDefItem::Constant(c)));
        } else if let Some(fwb) = FunctionWithBlock::parse(parser)? {
            return Ok(Some(TraitDefItem::FuncDef(fwb)));
        } else if let Some(fs) = FunctionSig::parse(parser)? {
            return Ok(Some(TraitDefItem::FuncSig(fs)));
        } else if let Some(ta) = TypeAliasDef::parse(parser)? {
            return Ok(Some(TraitDefItem::TypeAlias(ta)));
        } else {
            return Ok(None);
        }
    }
}

impl ParseItem for TraitDef {
    #[allow(unused_variables)]
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut associated_items: Vec<TraitDefItem> = Vec::new();

        let outer_attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = if let Some(v) = VisibilityKind::parse(parser)? {
            parser.next_token();
            Some(v)
        } else {
            None
        };

        let kw_trait_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwTrait,
            ..
        }) = kw_trait_opt
        {
            parser.next_token();

            if let Some(trait_name) = parser.peek_current::<Identifier>() {
                parser.next_token();

                let type_param_bounds_opt = if let Some(t) = TypeParamBounds::parse(parser)? {
                    parser.next_token();
                    Some(t)
                } else {
                    None
                };

                let where_clause_opt = WhereClause::parse(parser)?;

                let open_brace_opt = parser.peek_current::<Delimiter>();

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) = open_brace_opt
                {
                    parser.next_token();

                    let inner_attributes_opt = utils::get_attributes(parser)?;

                    if let Some(item) = TraitDefItem::parse(parser)? {
                        associated_items.push(item);

                        parser.next_token();
                    }

                    while let Some(Punctuation {
                        punc_kind: PuncKind::Comma,
                        ..
                    }) = parser.peek_current::<Punctuation>()
                    {
                        parser.next_token();

                        if let Some(next_item) = TraitDefItem::parse(parser)? {
                            associated_items.push(next_item);
                            parser.next_token();
                        } else {
                            break;
                        }
                    }

                    utils::skip_trailing_comma(parser)?;

                    let close_brace_opt = parser.peek_current::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_opt
                    {
                        return Ok(Some(TraitDef {
                            outer_attributes_opt,
                            visibility_opt,
                            kw_trait: kw_trait_opt.unwrap(),
                            trait_name,
                            type_param_bounds_opt,
                            where_clause_opt,
                            open_brace: open_brace_opt.unwrap(),
                            inner_attributes_opt,
                            associated_items,
                            close_brace: close_brace_opt.unwrap(),
                        }));
                    }

                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`}`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`{`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
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

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_trait_def() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
            #[abstract]
            pub trait foo 
                where Self: Bar
            {
                #![export]
                fn baz()
            }
        "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let trait_def = TraitDef::parse(&mut parser).expect("unable to parse trait definition");

        Ok(println!("{:#?}", trait_def))
    }
}
