#![allow(dead_code)]
#![allow(unused_variables)]

use feo_ast::{expression::Returnable, token::Token};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};

use feo_types::{literal::LiteralKind, Identifier};

use crate::{parse::ParseExpr, parser::Parser};

mod array_expr;
mod call_expr;
mod closure_expr;
mod field_access_expr;
mod literal_expr;
mod operator_expr;
mod parenthesized_expr;
mod struct_expr;
mod tuple_expr;

impl ParseExpr for Returnable {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        if let Some(id) = parser.peek_current::<Identifier>() {
            // if let Some(fc) = FunctionCallExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::FunctionCallExpr(fc)));
            // } else if let Some(mc) = MethodCallExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::MethodCallExpr(mc)));
            // } else if let Some(fa) = FieldAccessExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::FieldAccessExpr(fa)));
            // } else if let Some(se) = StructExpr::parse(parser).unwrap_or(None) {
            // if let Some(se) = StructExpr::parse(parser).unwrap_or(None) {
            // return Ok(Some(Returnable::StructExpr(StructExprKind::Struct(se))));
            // } else if let Some(ts) = TupleStructExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::StructExpr(StructExprKind::TupleStruct(
            //         ts,
            //     ))));
            // } else if let Some(us) = UnitStructExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::StructExpr(StructExprKind::UnitStruct(us))));
            // } else if let Some(pat) = PathInExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::PathExpr(pat)));
            // } else if let Some(al) = ArithmeticOrLogicalExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::ArithmeticOrLogicalExpr(al)));
            // } else {
            return Ok(Some(Returnable::Identifier(id)));
            // }
            // } else if let Some(_) = parser.peek_current::<Delimiter>() {
            // if let Some(ae) = ArrayExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::ArrayExpr(ae)))
            // } else if let Some(ie) = IndexExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::IndexExpr(ie)))
            // } else if let Some(te) = TupleExpr::parse(parser).unwrap_or(None) {
            //    return Ok( Some(Returnable::TupleExpr(te)))
            // } else if let Some(ti) = TupleIndexExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::TupleIndexExpr(ti)))
            // } else if let Some(par) = ParenthesizedExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::ParenthesizedExpr(par)))
            // } else {
            // parser.log_error(ParserErrorKind::UnexpectedToken {
            //     expected: "`Returnable`".to_string(),
            //     found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            // });
            // }
        } else if let Some(l) = parser.peek_current::<LiteralKind>() {
            //     if let Some(al) = ArithmeticOrLogicalExpr::parse(parser).unwrap_or(None) {
            //        return Ok( Some(Returnable::ArithmeticOrLogicalExpr(al)))
            //     } else if let Some(tc) = TypeCastExpr::parse(parser).unwrap_or(None) {
            //    return Ok( Some(Returnable::TypeCastExpr(tc)))
            //     } else {
            return Ok(Some(Returnable::Literal(l)));
            //     }
            // } else if let Some(_) = parser.peek_current::<Keyword>() {
            //     if let Some(pe) = PathInExpr::parse(parser).unwrap_or(None) {
            //         return Ok(Some(Returnable::PathExpr(pe)));
            //     } else {
            //         parser.log_error(ParserErrorKind::UnexpectedToken {
            //             expected: "`Returnable`".to_string(),
            //             found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            //         });
            //     }
            // } else if let Some(_) = parser.peek_current::<Punctuation>() {
            // if let Some(cwb) = ClosureWithBlock::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::ClosureWithBlock(cwb)));
            // } else if let Some(c) = ClosureWithoutBlock::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::ClosureWithoutBlock(c)));
            // } else if let Some(de) = DereferenceExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::DereferenceExpr(de)));
            // } else if let Some(ne) = NegationExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::NegationExpr(ne)));
            // } else if let Some(re) = ReferenceExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::ReferenceExpr(re)));
            // } else if let Some(ue) = UnderscoreExpr::parse(parser).unwrap_or(None) {
            //     return Ok(Some(Returnable::UnderscoreExpr(ue)));
            // } else {
            //     parser.log_error(ParserErrorKind::UnexpectedToken {
            //         expected: "`Returnable`".to_string(),
            //         found: parser.current_token().unwrap_or(Token::EOF).to_string(),
            //     });
            // }
        } else {
            parser.log_error(ParserErrorKind::InvalidToken {
                token: parser.current_token().unwrap_or(Token::EOF).to_string(),
            });
        }

        parser.next_token();

        Err(parser.errors())
    }
}
