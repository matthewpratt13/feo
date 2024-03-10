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

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for EnumVariant {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        while let Some(a) = OuterAttr::parse(parser)? {
            attributes.push(a);
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
                    attributes: Some(attributes),
                    visibility_opt,
                    variant_name,
                    variant_type_opt,
                })),

                false => Ok(Some(EnumVariant {
                    attributes: None,
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
            let mut next_comma_opt = parser.peek_current::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = next_comma_opt
            {
                parser.next_token();

                if let Some(next_variant) = EnumVariant::parse(parser)? {
                    subsequent_variants.push(next_variant);

                    if let Some(p) = parser.peek_current::<Punctuation>() {
                        next_comma_opt = Some(p);
                    } else {
                        break;
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`EnumVariant`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                    break;
                }
            }

            let trailing_comma_opt = parser.peek_current::<Punctuation>();

            if let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = trailing_comma_opt
            {
                parser.next_token();
            }

            match &subsequent_variants.is_empty() {
                true => Ok(Some(EnumVariants {
                    first_variant,
                    subsequent_variants: None,
                    trailing_comma_opt,
                })),

                false => Ok(Some(EnumVariants {
                    first_variant,
                    subsequent_variants: Some(subsequent_variants),
                    trailing_comma_opt,
                })),
            }
        } else {
            return Ok(None);
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
                parser.next_token();
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
                parser.next_token();
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

impl ParseTerm for EnumDef {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        while let Some(attr) = OuterAttr::parse(parser)? {
            attributes.push(attr);
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
                        parser.next_token();
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
                                    attributes: None,
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
                                    attributes: Some(attributes),
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
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_enum_variant_struct() {
        let source_code = r#"Foo { bar: u64 }"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let enum_variant_struct =
            EnumVariantStruct::parse(&mut parser).expect("unable to parse enum variant struct");

        println!("{:#?}", enum_variant_struct);
    }

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_enum_variant_tuple() {
        let source_code = r#"Foo(u64)"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let enum_variant_tuple =
            EnumVariantTuple::parse(&mut parser).expect("unable to parse enum variant tuple");

        println!("{:#?}", enum_variant_tuple);
    }

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_enum_def() {
        let source_code = r#"
        #[abstract]
        enum Foo {
            Bar,
            Baz(u64)
        }
        "#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let enum_def = EnumDef::parse(&mut parser).expect("unable to parse enum def type");

        println!("{:#?}", enum_def);
    }
}
