use feo_ast::expression::UnderscoreExpr;
use feo_error::error::CompilerError;
use feo_types::Identifier;

use crate::{parse::ParseExpr, parser::Parser};

impl ParseExpr for UnderscoreExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            return Ok(Some(UnderscoreExpr(id)));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {

    use crate::test_utils;

    use super::*;

    #[test]
    fn parse_underscore_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"_"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let underscore_expr =
            UnderscoreExpr::parse(&mut parser).expect("unable to parse underscore expression");

        Ok(println!("{:#?}", underscore_expr))
    }
}
