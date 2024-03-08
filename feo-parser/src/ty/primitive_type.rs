/* use feo_ast::ty::PrimitiveType;
use feo_error::error::CompilerError;
use feo_types::literal::{FloatType, IntType, LiteralKind, UIntType};

use crate::{parse::ParseTerm, parser::Parser};

// parsing unnecessary – this would conflict with `Literal` in `Expression` variants etc.

impl ParseTerm for PrimitiveType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let primitive = if let Some(l) = parser.peek_current::<LiteralKind>() {
            match l {
                LiteralKind::Char(c) => Some(PrimitiveType::Char(c.into_inner().unwrap())),
                LiteralKind::String(s) => Some(PrimitiveType::String(s.into_inner().unwrap())),
                LiteralKind::Bool(b) => Some(PrimitiveType::Bool(b.into_inner().unwrap())),
                LiteralKind::Int(it) => match it.into_inner().unwrap() {
                    IntType::I32(i) => Some(PrimitiveType::I32(i)),
                    IntType::I64(i) => Some(PrimitiveType::I64(i)),
                },
                LiteralKind::UInt(uit) => match uit.into_inner().unwrap() {
                    UIntType::U8(ui) => Some(PrimitiveType::U8(ui)),
                    UIntType::U16(ui) => Some(PrimitiveType::U16(ui)),
                    UIntType::U32(ui) => Some(PrimitiveType::U32(ui)),
                    UIntType::U64(ui) => Some(PrimitiveType::U64(ui)),
                },
                LiteralKind::U256(u) => Some(PrimitiveType::U256(u.into_inner().unwrap())),
                LiteralKind::Float(ft) => match ft.into_inner().unwrap() {
                    FloatType::F32(f) => Some(PrimitiveType::F32(f)),
                    FloatType::F64(f) => Some(PrimitiveType::F64(f)),
                },
            }
        } else {
            None
        };

        Ok(primitive)
    }
}

#[cfg(test)]
mod tests {
    use feo_error::handler::Handler;

    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn parse_primitive_type() {
        let source_code = r#""a" 1 bool"#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler.clone());

        let token_stream = lexer.lex().expect("unable to lex source code");

        // println!("{:#?}", token_stream);

        let mut parser = Parser::new(token_stream, handler);

        let primitive_type =
            PrimitiveType::parse(&mut parser).expect("unable to parse primitive type");

        println!("{:#?}", primitive_type);
    }
}
 */