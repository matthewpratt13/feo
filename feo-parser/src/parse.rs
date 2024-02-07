use feo_error::handler::ErrorEmitted;
use feo_types::Punctuation;

use crate::parser::{Parser, TokenType};

pub trait Parse {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized;
}

pub trait ParseExpr {
    fn parse_expr(&mut self, parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized;
}

impl Parse for Punctuation {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        match TokenType::from(parser.current_token()) {
            TokenType::Punctuation(p) => match Punctuation::try_from(p) {
                Ok(p) => Ok(Some(p)),
                Err(_) => todo!(),
            },
            _ => todo!(),
        }
    }
}
