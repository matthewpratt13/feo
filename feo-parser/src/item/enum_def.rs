use feo_ast::{
    attribute::OuterAttr,
    item::{
        EnumDef, EnumVariant, EnumVariantStruct, EnumVariantTuple, EnumVariantType, EnumVariants,
        StructDefFields, TupleStructDefFields, VisibilityKind,
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
};

impl ParseTerm for EnumVariant {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        while let Some(oa) = OuterAttr::parse(parser)? {
            attributes.push(oa);
            parser.next_token();
        }

        let visibility_opt = if let Some(v) = VisibilityKind::parse(parser)? {
            parser.next_token();
            Some(v)
        } else {
            None
        };

        if let Some(variant_name) = parser.peek_current::<Identifier>() {
            parser.next_token();

            let variant_type_opt = if let Some(v) = EnumVariantType::parse(parser)? {
                Some(v)
            } else {
                None
            };

            match &attributes.is_empty() {
                true => Ok(Some(EnumVariant {
                    attributes_opt: None,
                    visibility_opt,
                    variant_name,
                    variant_type_opt,
                })),

                false => Ok(Some(EnumVariant {
                    attributes_opt: Some(attributes),
                    visibility_opt,
                    variant_name,
                    variant_type_opt,
                })),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for EnumVariants {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_variants: Vec<EnumVariant> = Vec::new();

        if let Some(first_variant) = EnumVariant::parse(parser)? {
            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(next_variant) = EnumVariant::parse(parser)? {
                    subsequent_variants.push(next_variant);
                } else {
                    break;
                }
            }

            match &subsequent_variants.is_empty() {
                true => Ok(Some(EnumVariants {
                    first_variant,
                    subsequent_variants_opt: None,
                })),

                false => Ok(Some(EnumVariants {
                    first_variant,
                    subsequent_variants_opt: Some(subsequent_variants),
                })),
            }
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
        let open_brace_opt = parser.peek_current::<Delimiter>();

        if let Some(Delimiter {
            delim: (DelimKind::Brace, DelimOrientation::Open),
            ..
        }) = open_brace_opt
        {
            parser.next_token();

            let fields_opt = if let Some(f) = StructDefFields::parse(parser)? {
                Some(f)
            } else {
                None
            };

            let close_brace_opt = parser.peek_current::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Close),
                ..
            }) = close_brace_opt
            {
                parser.next_token();

                return Ok(Some(EnumVariantStruct {
                    open_brace: open_brace_opt.unwrap(),
                    fields_opt,
                    close_brace: close_brace_opt.unwrap(),
                }));
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
        let open_parenthesis_opt = parser.peek_current::<Delimiter>();

        if let Some(Delimiter {
            delim: (DelimKind::Parenthesis, DelimOrientation::Open),
            ..
        }) = open_parenthesis_opt
        {
            parser.next_token();

            let elements_opt = if let Some(e) = TupleStructDefFields::parse(parser)? {
                Some(e)
            } else {
                None
            };

            let close_parenthesis_opt = parser.peek_current::<Delimiter>();

            if let Some(Delimiter {
                delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                ..
            }) = close_parenthesis_opt
            {
                parser.next_token();

                return Ok(Some(EnumVariantTuple {
                    open_parenthesis: open_parenthesis_opt.unwrap(),
                    elements_opt,
                    close_parenthesis: close_parenthesis_opt.unwrap(),
                }));
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
        let mut attributes: Vec<OuterAttr> = Vec::new();

        while let Some(oa) = OuterAttr::parse(parser)? {
            attributes.push(oa);
            parser.next_token();
        }

        let visibility_opt = if let Some(v) = VisibilityKind::parse(parser)? {
            parser.next_token();
            Some(v)
        } else {
            None
        };

        let kw_enum_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwEnum,
            ..
        }) = kw_enum_opt
        {
            parser.next_token();

            if let Some(enum_name) = parser.peek_current::<Identifier>() {
                parser.next_token();

                let open_brace_opt = parser.peek_current::<Delimiter>();

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) = open_brace_opt
                {
                    parser.next_token();

                    let enum_variants_opt = if let Some(e) = EnumVariants::parse(parser)? {
                        Some(e)
                    } else {
                        None
                    };

                    let close_brace_opt = parser.peek_current::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_opt
                    {
                        match &attributes.is_empty() {
                            true => {
                                return Ok(Some(EnumDef {
                                    attributes_opt: None,
                                    visibility_opt,
                                    kw_enum: kw_enum_opt.unwrap(),
                                    enum_name,
                                    open_brace: open_brace_opt.unwrap(),
                                    enum_variants_opt,
                                    close_brace: close_brace_opt.unwrap(),
                                }))
                            }

                            false => {
                                return Ok(Some(EnumDef {
                                    attributes_opt: Some(attributes),
                                    visibility_opt,
                                    kw_enum: kw_enum_opt.unwrap(),
                                    enum_name,
                                    open_brace: open_brace_opt.unwrap(),
                                    enum_variants_opt,
                                    close_brace: close_brace_opt.unwrap(),
                                }))
                            }
                        }
                    }

                    parser.log_error(ParserErrorKind::MissingDelimiter {
                        delim: "}".to_string(),
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
    fn parse_enum_variant_struct() {
        let source_code = r#"{ bar: u64 }"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let enum_variant_struct =
            EnumVariantStruct::parse(&mut parser).expect("unable to parse enum variant struct");

        println!("{:#?}", enum_variant_struct);
    }

    #[test]
    fn parse_enum_variant_tuple() {
        let source_code = r#"(u64, bool)"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let enum_variant_tuple =
            EnumVariantTuple::parse(&mut parser).expect("unable to parse enum variant tuple");

        println!("{:#?}", enum_variant_tuple);
    }

    #[test]
    fn parse_enum_def() {
        let source_code = r#"
        #[abstract]
        enum Foo {
            Bar,
            Baz(u64)
        }"#;

        let mut parser = test_utils::get_parser(source_code, true);

        let enum_def = EnumDef::parse(&mut parser).expect("unable to parse enum def");

        println!("{:#?}", enum_def);
    }
}
