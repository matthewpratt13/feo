use feo_ast::{
    item::{
        StructDef, StructDefField, TupleStructDef, TupleStructDefField, VisibilityKind, WhereClause,
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

use crate::{
    parse::{ParseItem, ParseTerm, ParseType},
    parser::Parser,
    utils,
};

impl ParseTerm for StructDefField {
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

        if let Some(field_name) = parser.peek_current::<Identifier>() {
            parser.next_token();

            if let Some(Punctuation {
                punc_kind: PuncKind::Colon,
                ..
            }) = parser.peek_current()
            {
                parser.next_token();

                if let Some(ty) = Type::parse(parser)? {
                    // parser.next_token();

                    let field_type = (field_name, Box::new(ty));

                    return Ok(Some(StructDefField {
                        attributes_opt,
                        visibility_opt,
                        field_type,
                    }));
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "type".to_string(),
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

impl ParseItem for StructDef {
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

        let kw_struct_opt = parser.peek_current();

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

                let open_brace_opt = parser.peek_current();

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) = open_brace_opt
                {
                    parser.next_token();

                    let fields_opt = utils::get_term_collection(parser)?;

                    let close_brace_opt = parser.peek_current();

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_opt
                    {
                        return Ok(Some(StructDef {
                            attributes_opt,
                            visibility_opt,
                            kw_struct: kw_struct_opt.unwrap(),
                            struct_name,
                            where_clause_opt,
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
        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = if let Some(v) = VisibilityKind::parse(parser)? {
            parser.next_token();
            Some(v)
        } else {
            None
        };

        if let Some(field_type) = Type::parse(parser)? {
            Ok(Some(TupleStructDefField {
                attributes_opt,
                visibility_opt,
                field_type: Box::new(field_type),
            }))
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
        let attributes_opt = utils::get_attributes(parser)?;

        let visibility_opt = if let Some(v) = VisibilityKind::parse(parser)? {
            parser.next_token();
            Some(v)
        } else {
            None
        };

        let kw_struct_opt = parser.peek_current();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwStruct,
            ..
        }) = kw_struct_opt
        {
            parser.next_token();

            if let Some(struct_name) = parser.peek_current::<Identifier>() {
                parser.next_token();

                let open_parenthesis_opt = parser.peek_current();

                if let Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Open),
                    ..
                }) = open_parenthesis_opt
                {
                    parser.next_token();

                    let fields_opt = utils::get_term_collection::<TupleStructDefField>(parser)?;

                    parser.next_token();

                    let close_parenthesis_opt = parser.peek_current();

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

                        let semicolon_opt = parser.peek_current();

                        if let Some(Punctuation {
                            punc_kind: PuncKind::Semicolon,
                            ..
                        }) = semicolon_opt
                        {
                            parser.next_token();

                            return Ok(Some(TupleStructDef {
                                attributes_opt,
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

                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`;`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                        });
                    } else {
                        parser.log_error(ParserErrorKind::UnexpectedToken {
                            expected: "`}`".to_string(),
                            found: parser.current_token().unwrap_or(Token::EOF).to_string(),
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

    #[test]
    fn parse_struct_def_field() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[foo]
        pub bar: u64
        "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let struct_def_field =
            StructDefField::parse(&mut parser).expect("unable to parse struct definition field");

        Ok(println!("{:#?}", struct_def_field))
    }

    #[test]
    fn parse_tuple_struct_def_field() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"#[foo] pub u64"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let tuple_struct_def_field = TupleStructDefField::parse(&mut parser)
            .expect("unable to parse tuple struct definition field");

        Ok(println!("{:#?}", tuple_struct_def_field))
    }

    #[test]
    fn parse_struct_def() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[abstract]
        struct Foo {
            pub bar: u64,
            baz: bool,
        }"#;
        let mut parser = test_utils::get_parser(source_code, false)?;

        let struct_def = StructDef::parse(&mut parser).expect("unable to parse struct definition");

        Ok(println!("{:#?}", struct_def))
    }

    #[test]
    fn parse_tuple_struct_def() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"
        #[abstract]
        struct Foo(pub u64, bool);
        "#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let tuple_struct_def =
            TupleStructDef::parse(&mut parser).expect("unable to parse tuple struct definition");

        Ok(println!("{:#?}", tuple_struct_def))
    }
}
