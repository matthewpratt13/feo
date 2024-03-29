use feo_ast::{
    item::{ConstVarDef, FuncSig, FuncWithBlock, TraitDef, TraitDefItem, TypeDef},
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    Delimiter, Identifier, Keyword,
};

use crate::{
    parse::ParseItem,
    parser::Parser,
    test_utils::{self, LogMsgType},
    utils,
};

impl ParseItem for TraitDefItem {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(c) = ConstVarDef::parse(parser)? {
            return Ok(Some(TraitDefItem::Constant(c)));
        } else if let Some(fwb) = FuncWithBlock::parse(parser)? {
            return Ok(Some(TraitDefItem::FuncDef(fwb)));
        } else if let Some(fs) = FuncSig::parse(parser)? {
            return Ok(Some(TraitDefItem::FuncSig(fs)));
        } else if let Some(ta) = TypeDef::parse(parser)? {
            return Ok(Some(TraitDefItem::TypeAlias(ta)));
        } else {
            return Ok(None);
        }
    }
}

impl ParseItem for TraitDef {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let outer_attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = utils::get_visibility(parser)?;

        let kw_trait_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwTrait,
            ..
        }) = kw_trait_opt
        {
            test_utils::log_msg(LogMsgType::Detect, "`trait` keyword", parser);

            if let Some(trait_name) = parser.peek_next::<Identifier>() {
                parser.next_token();

                test_utils::log_msg(LogMsgType::Detect, "trait name", parser);

                parser.next_token();

                let type_param_bounds_opt = utils::get_term_collection(parser)?;

                let open_brace_opt = parser.peek_current();

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) = open_brace_opt
                {
                    test_utils::log_msg(LogMsgType::Enter, "trait definition body", parser);

                    parser.next_token();

                    let inner_attributes_opt = utils::get_attributes(parser)?;

                    let associated_items_opt = utils::get_items::<TraitDefItem>(parser)?;

                    let close_brace_opt = parser.peek_current();

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_opt
                    {
                        test_utils::log_msg(LogMsgType::Exit, "trait definition body", parser);

                        return Ok(Some(TraitDef {
                            outer_attributes_opt,
                            visibility_opt,
                            kw_trait: kw_trait_opt.unwrap(),
                            trait_name,
                            type_param_bounds_opt,
                            open_brace: open_brace_opt.unwrap(),
                            inner_attributes_opt,
                            associated_items_opt,
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
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_trait_def() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
            #[abstract]
            pub trait foo 
            {
                #![export]
                func baz();
            }
        "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let trait_def = TraitDef::parse(&mut parser).expect("unable to parse trait definition");

        Ok(println!("{:#?}", trait_def))
    }
}
