use feo_ast::{
    attribute::OuterAttr,
    item::{
        StructDefField, StructDefFields, TupleStructDefField, TupleStructDefFields, VisibilityKind,
    },
    token::Token,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{punctuation::PuncKind, utils::Comma, Identifier, Punctuation};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for StructDefField {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        while let Some(attr) = OuterAttr::parse(parser)? {
            attributes.push(attr);
            parser.next_token();
        }

        let visibility_opt = VisibilityKind::parse(parser)?;

        if let Some(field_name) = parser.peek_current::<Identifier>() {
            parser.next_token();

            let colon_opt = parser.peek_current::<Punctuation>();

            if let Some(Punctuation {
                punc_kind: PuncKind::Colon,
                ..
            }) = colon_opt
            {
                parser.next_token();

                if let Some(field_type) = Type::parse(parser)? {
                    parser.next_token();

                    let field_content = (field_name, colon_opt.unwrap(), Box::new(field_type));

                    match &attributes.is_empty() {
                        true => {
                            return Ok(Some(StructDefField(None, visibility_opt, field_content)));
                        }

                        false => {
                            return Ok(Some(StructDefField(
                                Some(attributes),
                                visibility_opt,
                                field_content,
                            )));
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
        let mut subsequent_fields: Vec<(Comma, StructDefField)> = Vec::new();

        if let Some(first_field) = StructDefField::parse(parser)? {
            let mut next_comma_opt = parser.peek_current::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = next_comma_opt
            {
                parser.next_token();

                if let Some(next_field) = StructDefField::parse(parser)? {
                    subsequent_fields.push((next_comma_opt.unwrap(), next_field));

                    if let Some(p) = parser.peek_current::<Punctuation>() {
                        next_comma_opt = Some(p);
                    } else {
                        break;
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`StructDefField`".to_string(),
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

            match &subsequent_fields.is_empty() {
                true => Ok(Some(StructDefFields {
                    first_field,
                    subsequent_fields: None,
                    trailing_comma_opt,
                })),

                false => Ok(Some(StructDefFields {
                    first_field,
                    subsequent_fields: Some(subsequent_fields),
                    trailing_comma_opt,
                })),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for TupleStructDefField {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        while let Some(attr) = OuterAttr::parse(parser)? {
            attributes.push(attr);
            parser.next_token();
        }

        let visibility_opt = VisibilityKind::parse(parser)?;

        if let Some(field_type) = Type::parse(parser)? {
            parser.next_token();

            match &attributes.is_empty() {
                true => Ok(Some(TupleStructDefField(
                    None,
                    visibility_opt,
                    Box::new(field_type),
                ))),

                false => Ok(Some(TupleStructDefField(
                    Some(attributes),
                    visibility_opt,
                    Box::new(field_type),
                ))),
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
        let mut subsequent_fields: Vec<(Comma, TupleStructDefField)> = Vec::new();

        if let Some(first_field) = TupleStructDefField::parse(parser)? {
            let mut next_comma_opt = parser.peek_current::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = next_comma_opt
            {
                parser.next_token();

                if let Some(next_field) = TupleStructDefField::parse(parser)? {
                    subsequent_fields.push((next_comma_opt.unwrap(), next_field));

                    if let Some(p) = parser.peek_current::<Punctuation>() {
                        next_comma_opt = Some(p)
                    } else {
                        break;
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`TupleStructDefField`".to_string(),
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

            match &subsequent_fields.is_empty() {
                true => Ok(Some(TupleStructDefFields {
                    first_field,
                    subsequent_fields: None,
                    trailing_comma_opt,
                })),

                false => Ok(Some(TupleStructDefFields {
                    first_field,
                    subsequent_fields: Some(subsequent_fields),
                    trailing_comma_opt,
                })),
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_struct_def_field() {
        let source_code = r#"
        #[foo]
        pub bar: u64
        "#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let struct_def_field =
            StructDefField::parse(&mut parser).expect("unable to parse struct def field");

        println!("{:#?}", struct_def_field);
    }

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_tuple_struct_def_field() {
        let source_code = r#"#[foo] pub u64"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let tuple_struct_def_field = TupleStructDefField::parse(&mut parser)
            .expect("unable to parse tuple struct def field");

        println!("{:#?}", tuple_struct_def_field);
    }
}
