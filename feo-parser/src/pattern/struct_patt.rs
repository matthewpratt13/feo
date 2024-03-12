use feo_ast::{
    attribute::OuterAttr,
    pattern::{
        Pattern, StructPatt, StructPattField, StructPattFields, TupleStructPatt,
        TupleStructPattFields,
    },
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{punctuation::PuncKind, Identifier, Punctuation};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for StructPattField {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        while let Some(oa) = OuterAttr::parse(parser)? {
            attributes.push(oa);
            parser.next_token();
        }

        if let Some(id) = parser.peek_current::<Identifier>() {
            parser.next_token();

            if let Some(p) = Pattern::parse(parser)? {
                parser.next_token();

                let field_contents = (id, Box::new(p));

                match &attributes.is_empty() {
                    true => return Ok(Some(StructPattField(None, field_contents))),
                    false => return Ok(Some(StructPattField(Some(attributes), field_contents))),
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`Pattern`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}

impl ParseTerm for StructPattFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut subsequent_fields: Vec<StructPattField> = Vec::new();

        if let Some(first_field) = StructPattField::parse(parser)? {
            let mut next_comma_opt = parser.peek_current::<Punctuation>();

            while let Some(Punctuation {
                punc_kind: PuncKind::Comma,
                ..
            }) = next_comma_opt
            {
                parser.next_token();

                if let Some(next_field) = StructPattField::parse(parser)? {
                    subsequent_fields.push(next_field);

                    if let Some(p) = parser.peek_current::<Punctuation>() {
                        next_comma_opt = Some(p)
                    } else {
                        break;
                    }
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`StructPattField`".to_string(),
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
                true => Ok(Some(StructPattFields {
                    first_field,
                    subsequent_fields: None,
                    trailing_comma_opt,
                })),

                false => Ok(Some(StructPattFields {
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

impl ParseTerm for StructPatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseTerm for TupleStructPattFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseTerm for TupleStructPatt {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[ignore] // TODO: remove when testing
    #[test]
    fn parse_struct_patt_field() {
        let source_code = r#"
            #[abstract]
            foo: "a",
        }"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let struct_patt_fields =
            StructPattField::parse(&mut parser).expect("unable to parse struct pattern field");

        println!("{:#?}", struct_patt_fields);
    }
}
