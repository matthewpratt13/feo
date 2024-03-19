use feo_ast::{
    item::{
        EnumDef, EnumVariant, EnumVariantStruct, EnumVariantTuple, EnumVariantType,
        TupleStructDefField, VisibilityKind,
    },
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    Delimiter, Identifier, Keyword,
};

use crate::{
    parse::{ParseItem, ParseTerm},
    parser::Parser,
    utils,
};

impl ParseTerm for EnumVariant {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = if let Some(v) = VisibilityKind::parse(parser)? {
            parser.next_token();
            Some(v)
        } else {
            None
        };

        if let Some(variant_name) = parser.peek_current::<Identifier>() {
            let variant_type_opt = match parser.peek_next::<Delimiter>() {
                Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(t) = EnumVariantTuple::parse(parser)? {
                        Some(EnumVariantType::Tuple(t))
                    } else {
                        None
                    }
                }

                Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) => {
                    if let Some(s) = EnumVariantStruct::parse(parser)? {
                        Some(EnumVariantType::Struct(s))
                    } else {
                        None
                    }
                }

                _ => None,
            };

            Ok(Some(EnumVariant {
                attributes_opt,
                visibility_opt,
                variant_name,
                variant_type_opt,
            }))
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for EnumVariantType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(s) = EnumVariantStruct::parse(parser)? {
            Ok(Some(EnumVariantType::Struct(s)))
        } else if let Some(t) = EnumVariantTuple::parse(parser)? {
            Ok(Some(EnumVariantType::Tuple(t)))
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for EnumVariantStruct {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(_) = parser.peek_current::<Identifier>() {
            parser.next_token();

            let open_brace_opt = parser.peek_current();

            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = open_brace_opt
            {
                parser.next_token();

                let fields_opt = utils::get_term_collection(parser)?;

                parser.next_token();

                let close_brace_opt = parser.peek_current();

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Close),
                    ..
                }) = close_brace_opt
                {
                    return Ok(Some(EnumVariantStruct {
                        open_brace: open_brace_opt.unwrap(),
                        fields_opt,
                        close_brace: close_brace_opt.unwrap(),
                    }));
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`}`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseTerm for EnumVariantTuple {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(_) = parser.peek_current::<Identifier>() {
            parser.next_token();

            let open_parenthesis_opt = parser.peek_current();

            if let Some(Delimiter {
                delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                ..
            }) = open_parenthesis_opt
            {
                parser.next_token();

                let elements_opt = utils::get_term_collection::<TupleStructDefField>(parser)?;

                parser.next_token();

                let close_parenthesis_opt = parser.peek_current();

                if let Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                    ..
                }) = close_parenthesis_opt
                {
                    return Ok(Some(EnumVariantTuple {
                        open_parenthesis: open_parenthesis_opt.unwrap(),
                        elements_opt,
                        close_parenthesis: close_parenthesis_opt.unwrap(),
                    }));
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`)`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseItem for EnumDef {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = if let Some(v) = VisibilityKind::parse(parser)? {
            parser.next_token();
            Some(v)
        } else {
            None
        };

        let kw_enum_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwEnum,
            ..
        }) = kw_enum_opt
        {
            parser.next_token();

            if let Some(enum_name) = parser.peek_current::<Identifier>() {
                parser.next_token();

                let open_brace_opt = parser.peek_current();

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) = open_brace_opt
                {
                    parser.next_token();

                    let enum_variants_opt = utils::get_term_collection::<EnumVariant>(parser)?;

                    parser.next_token();

                    let close_brace_opt = parser.peek_current();

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_opt
                    {
                        parser.next_token();

                        return Ok(Some(EnumDef {
                            attributes_opt,
                            visibility_opt,
                            kw_enum: kw_enum_opt.unwrap(),
                            enum_name,
                            open_brace: open_brace_opt.unwrap(),
                            enum_variants_opt,
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

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_enum_variant() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"Foo(u64)"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let enum_variant = EnumVariant::parse(&mut parser).expect("unable to parse enum variant");

        Ok(println!("{:#?}", enum_variant))
    }

    #[test]
    fn parse_enum_variant_struct() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"{ bar: u64 }"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let enum_variant_struct =
            EnumVariantStruct::parse(&mut parser).expect("unable to parse struct enum variant");

        Ok(println!("{:#?}", enum_variant_struct))
    }

    #[test]
    fn parse_enum_variant_tuple() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"(u64, bool)"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let enum_variant_tuple =
            EnumVariantTuple::parse(&mut parser).expect("unable to parse tuple enum variant");

        Ok(println!("{:#?}", enum_variant_tuple))
    }

    #[test]
    fn parse_enum_def() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[abstract]
        enum Foo {
            Bar,
            Baz(u64)
        }"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let enum_def = EnumDef::parse(&mut parser).expect("unable to parse enum definition");

        Ok(println!("{:#?}", enum_def))
    }
}
