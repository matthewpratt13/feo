use feo_ast::expression::{Expression, ParenthesizedExpr};
use feo_error::error::CompilerError;
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    Delimiter,
};

use crate::{
    parse::{ParseExpr, ParseTerm},
    parser::Parser,
};

impl ParseTerm for ParenthesizedExpr {
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

            if let Some(enclosed_operand) = Expression::parse(parser)? {
                parser.next_token();

                let close_parenthesis_opt = parser.peek_current::<Delimiter>();

                if let Some(Delimiter {
                    delim: (DelimKind::Parenthesis, DelimOrientation::Close),
                    ..
                }) = close_parenthesis_opt
                {
                    parser.next_token();

                    return Ok(Some(ParenthesizedExpr {
                        open_parenthesis: open_parenthesis_opt.unwrap(),
                        enclosed_operand: Box::new(enclosed_operand),
                        close_parenthesis: close_parenthesis_opt.unwrap(),
                    }));
                }
            } else {
                return Ok(None);
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
    fn parse_parenthesized_expr() -> Result<(), Vec<CompilerError>> {
        let source_code = r#"(x + 2)"#;

        let mut parser = test_utils::get_parser(source_code, false)?;

        let parenthesized_expr = ParenthesizedExpr::parse(&mut parser)
            .expect("unable to parse parenthesized expression");

        Ok(println!("{:#?}", parenthesized_expr))
    }
}
