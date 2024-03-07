use feo_ast::{
    attribute::OuterAttr,
    item::{
        StructDefField, StructDefFields, TupleStructDefField, TupleStructDefFields, VisibilityKind,
    },
    token::Token,
    Type,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{punctuation::PuncKind, Identifier, Punctuation};

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

                    match &attributes.is_empty() {
                        true => {
                            return Ok(Some(StructDefField {
                                attributes: None,
                                visibility_opt,
                                field_name,
                                colon: colon_opt.unwrap(),
                                field_type: Box::new(field_type),
                            }))
                        }

                        false => {
                            return Ok(Some(StructDefField {
                                attributes: Some(attributes),
                                visibility_opt,
                                field_name,
                                colon: colon_opt.unwrap(),
                                field_type: Box::new(field_type),
                            }))
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
        todo!()
    }
}

impl ParseTerm for TupleStructDefField {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseTerm for TupleStructDefFields {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}
