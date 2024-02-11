use feo_ast::token::Token;
use feo_error::handler::ErrorEmitted;
use feo_types::literal::LiteralKind;

use crate::{parse::Parse, parser::Parser};

// TODO: unnecessary – already done in `Literal<T>::peek()` methods
impl Parse for LiteralKind {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        match parser.current_token() {
            Token::CharLit(c) => Ok(Some(LiteralKind::Char(c))),
            Token::StringLit(s) => Ok(Some(LiteralKind::String(s))),
            Token::BoolLit(b) => Ok(Some(LiteralKind::Bool(b))),
            Token::IntLit(i) => Ok(Some(LiteralKind::I64(i))),
            Token::UIntLit(ui) => Ok(Some(LiteralKind::U64(ui))),
            Token::U256Lit(u) => Ok(Some(LiteralKind::U256(u))),
            Token::FloatLit(f) => Ok(Some(LiteralKind::F64(f))),
            _ => todo!(),
        }
    }
}
