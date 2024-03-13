use std::str::FromStr;

use feo_error::error::CompilerError;
use feo_types::{span::Spanned, type_annotation::TypeAnnotation, BuiltInType, Identifier};

use crate::{parse::ParseType, parser::Parser};

impl ParseType for BuiltInType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            let type_annotation = if let Ok(ta) = TypeAnnotation::from_str(&id.name) {
                ta
            } else {
                return Ok(None);
            };

            return Ok(Some(BuiltInType::new(type_annotation, id.span())));
        } else {
            return Ok(None);
        }
    }
}
