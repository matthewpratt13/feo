use feo_ast::{
    attribute::OuterAttr,
    item::{
        StructDef, StructDefField, StructDefFields, TupleStructDef, TupleStructDefField,
        TupleStructDefFields, VisibilityKind, WhereClause,
    },
    token::Token,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    punctuation::PuncKind,
    Delimiter, Identifier, Keyword, Punctuation,
};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for StructDefField {
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

        if let Some(field_name) = parser.peek_current::<Identifier>() {
            parser.next_token();

            if let Some(Punctuation {
                punc_kind: PuncKind::Colon,
                ..
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(ty) = Type::parse(parser)? {
                    parser.next_token();

                    let field_type = (field_name, Box::new(ty));

                    match &attributes.is_empty() {
                        true => {
                            return Ok(Some(StructDefField {
                                attributes_opt: None,
                                visibility_opt,
                                field_type,
                            }));
                        }

                        false => {
                            return Ok(Some(StructDefField {
                                attributes_opt: Some(attributes),
                                visibility_opt,
                                field_type,
                            }));
                        }
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`Type`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`:`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseTerm for StructDefFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_fields: Vec<StructDefField> = Vec::new();

        if let Some(first_field) = StructDefField::parse(parser)? {
            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(next_field) = StructDefField::parse(parser)? {
                    subsequent_fields.push(next_field);
                } else {
                    break;
                }
            }

            match &subsequent_fields.is_empty() {
                true => Ok(Some(StructDefFields {
                    first_field,
                    subsequent_fields: None,
                })),

                false => Ok(Some(StructDefFields {
                    first_field,
                    subsequent_fields: Some(subsequent_fields),
                })),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for StructDef {
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

        let kw_struct_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwStruct,
            ..
        }) = kw_struct_opt
        {
            parser.next_token();

            if let Some(struct_name) = parser.peek_current::<Identifier>() {
                parser.next_token();

                let where_clause_opt = if let Some(wc) = WhereClause::parse(parser)? {
                    parser.next_token();
                    Some(wc)
                } else {
                    None
                };

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

                        match &attributes.is_empty() {
                            true => {
                                return Ok(Some(StructDef {
                                    attributes_opt: None,
                                    visibility_opt,
                                    kw_struct: kw_struct_opt.unwrap(),
                                    struct_name,
                                    where_clause_opt,
                                    open_brace: open_brace_opt.unwrap(),
                                    fields_opt,
                                    close_brace: close_brace_opt.unwrap(),
                                }))
                            }

                            false => {
                                return Ok(Some(StructDef {
                                    attributes_opt: Some(attributes),
                                    visibility_opt,
                                    kw_struct: kw_struct_opt.unwrap(),
                                    struct_name,
                                    where_clause_opt,
                                    open_brace: open_brace_opt.unwrap(),
                                    fields_opt,
                                    close_brace: close_brace_opt.unwrap(),
                                }))
                            }
                        }
                    } else {
                        parser.log_error(ParserErrorKind::MissingDelimiter {
                            delim: "}".to_string(),
                        });
                    }
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

impl ParseTerm for TupleStructDefField {
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

        if let Some(field_type) = Type::parse(parser)? {
            parser.next_token();

            match &attributes.is_empty() {
                true => Ok(Some(TupleStructDefField {
                    attributes_opt: None,
                    visibility_opt,
                    field_type: Box::new(field_type),
                })),

                false => Ok(Some(TupleStructDefField {
                    attributes_opt: Some(attributes),
                    visibility_opt,
                    field_type: Box::new(field_type),
                })),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for TupleStructDefFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_fields: Vec<TupleStructDefField> = Vec::new();

        if let Some(first_field) = TupleStructDefField::parse(parser)? {
            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = parser.peek_current::<Punctuation>()
            {
                parser.next_token();

                if let Some(next_field) = TupleStructDefField::parse(parser)? {
                    subsequent_fields.push(next_field);
                } else {
                    break;
                }
            }

            match &subsequent_fields.is_empty() {
                true => Ok(Some(TupleStructDefFields {
                    first_field,
                    subsequent_fields: None,
                })),

                false => Ok(Some(TupleStructDefFields {
                    first_field,
                    subsequent_fields: Some(subsequent_fields),
                })),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for TupleStructDef {
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

        let kw_struct_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwStruct,
            ..
        }) = kw_struct_opt
        {
            parser.next_token();

            if let Some(struct_name) = parser.peek_current::<Identifier>() {
                parser.next_token();

                let open_parenthesis_opt = parser.peek_current::<Delimiter>();

                if let Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) = open_parenthesis_opt
                {
                    parser.next_token();

                    let fields_opt = if let Some(f) = TupleStructDefFields::parse(parser)? {
                        Some(f)
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

                        let where_clause_opt = if let Some(wc) = WhereClause::parse(parser)? {
                            parser.next_token();
                            Some(wc)
                        } else {
                            None
                        };

                        let semicolon_opt = parser.peek_current::<Punctuation>();

                        if let Some(Punctuation {
                            punc_kind: PuncKind::Semicolon,
                            ..
                        }) = semicolon_opt
                        {
                            parser.next_token();

                            match &attributes.is_empty() {
                                true => {
                                    return Ok(Some(TupleStructDef {
                                        attributes_opt: None,
                                        visibility_opt,
                                        kw_struct: kw_struct_opt.unwrap(),
                                        struct_name,
                                        open_parenthesis: open_parenthesis_opt.unwrap(),
                                        fields_opt,
                                        close_parenthesis: close_parenthesis_opt.unwrap(),
                                        where_clause_opt,
                                        semicolon: semicolon_opt.unwrap(),
                                    }));
                                }
                                false => {
                                    return Ok(Some(TupleStructDef {
                                        attributes_opt: Some(attributes),
                                        visibility_opt,
                                        kw_struct: kw_struct_opt.unwrap(),
                                        struct_name,
                                        open_parenthesis: open_parenthesis_opt.unwrap(),
                                        fields_opt,
                                        close_parenthesis: close_parenthesis_opt.unwrap(),
                                        where_clause_opt,
                                        semicolon: semicolon_opt.unwrap(),
                                    }));
                                }
                            }
                        } else {
                            parser.log_error(ParserErrorKind::MissingDelimiter {
                                delim: ";".to_string(),
                            });
                        }
                    } else {
                        parser.log_error(ParserErrorKind::MissingDelimiter {
                            delim: "}".to_string(),
                        });
                    }
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

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_struct_def_field() {
        let source_code = r#"
        #[foo]
        pub bar: u64
        "#;

        let mut parser = test_utils::get_parser(source_code, false);

        let struct_def_field =
            StructDefField::parse(&mut parser).expect("unable to parse struct def field");

        println!("{:#?}", struct_def_field);
    }

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_tuple_struct_def_field() {
        let source_code = r#"#[foo] pub u64"#;

        let mut parser = test_utils::get_parser(source_code, false);

        let tuple_struct_def_field = TupleStructDefField::parse(&mut parser)
            .expect("unable to parse tuple struct def field");

        println!("{:#?}", tuple_struct_def_field);
    }

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_struct_def() {
        let source_code = r#"
        #[abstract]
        struct Foo {
            pub bar: u64,
            baz: bool,
        }"#;
        let mut parser = test_utils::get_parser(source_code, false);

        let struct_def = StructDef::parse(&mut parser).expect("unable to parse struct def");

        println!("{:#?}", struct_def);
    }

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_tuple_struct_def() {
        let source_code = r#"
        #[abstract]
        struct Foo(pub u64, bool);
        "#;

        let mut parser = test_utils::get_parser(source_code, false);

        let tuple_struct_def =
            TupleStructDef::parse(&mut parser).expect("unable to parse tuple struct def");

        println!("{:#?}", tuple_struct_def);
    }
}
