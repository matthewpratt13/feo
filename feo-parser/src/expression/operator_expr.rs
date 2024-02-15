use feo_ast::expression::ArithmeticOrLogicalOperatorKind;
use feo_types::punctuation::PuncKind;

use crate::{parse::Peek, parser::Peeker};

impl Peek for ArithmeticOrLogicalOperatorKind {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let operator_kind = if let Ok(p) = peeker.peek_punctuation() {
            match p.punc_kind {
                PuncKind::Percent => ArithmeticOrLogicalOperatorKind::Modulus(p),
                PuncKind::Ampersand => ArithmeticOrLogicalOperatorKind::LogicalAnd(p),
                PuncKind::Asterisk => ArithmeticOrLogicalOperatorKind::Multiply(p),
                PuncKind::Plus => ArithmeticOrLogicalOperatorKind::Add(p),
                PuncKind::Minus => ArithmeticOrLogicalOperatorKind::Subtract(p),
                PuncKind::ForwardSlash => ArithmeticOrLogicalOperatorKind::Divide(p),
                PuncKind::Caret => ArithmeticOrLogicalOperatorKind::LogicalXOr(p),
                PuncKind::Pipe => ArithmeticOrLogicalOperatorKind::LogicalOr(p),
                PuncKind::DblLessThan => ArithmeticOrLogicalOperatorKind::ShiftLeft(p),
                PuncKind::DblGreaterThan => ArithmeticOrLogicalOperatorKind::ShiftRight(p),
                _ => return None,
            }
        } else {
            return None;
        };

        Some(operator_kind)
    }
}
