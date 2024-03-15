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

#[cfg(test)]
mod tests {

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_built_in_type() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"u64"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let built_in_type = BuiltInType::parse(&mut parser).expect("unable to parse built-in type");

        Ok(println!("{:#?}", built_in_type))
    }
}
