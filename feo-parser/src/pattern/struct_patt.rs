use feo_ast::{
    attribute::OuterAttr,
    pattern::{
        Pattern, StructPatt, StructPattField, StructPattFields, TupleStructPatt,
        TupleStructPattFields,
    },
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::Identifier;

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
        todo!()
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
