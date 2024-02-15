use feo_ast::expression::{
    ArithmeticOrLogicalOperatorKind, ComparisonOperatorKind, CompoundAssignOperatorKind,
    LazyBoolOperatorKind,
};
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

impl Peek for ComparisonOperatorKind {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let operator_kind = if let Ok(p) = peeker.peek_punctuation() {
            match p.punc_kind {
                PuncKind::LessThan => ComparisonOperatorKind::LessThan(p),
                PuncKind::GreaterThan => ComparisonOperatorKind::GreaterThan(p),
                PuncKind::BangEquals => ComparisonOperatorKind::NotEqual(p),
                PuncKind::LessThanEquals => ComparisonOperatorKind::LessThanOrEqual(p),
                PuncKind::DblEquals => ComparisonOperatorKind::Equality(p),
                PuncKind::GreaterThanEquals => ComparisonOperatorKind::GreaterThanOrEqual(p),
                _ => return None,
            }
        } else {
            return None;
        };

        Some(operator_kind)
    }
}

impl Peek for CompoundAssignOperatorKind {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let operator_kind = if let Ok(p) = peeker.peek_punctuation() {
            match p.punc_kind {
                PuncKind::PercentEquals => CompoundAssignOperatorKind::ModulusAssign(p),
                PuncKind::AsteriskEquals => CompoundAssignOperatorKind::MultiplyAssign(p),
                PuncKind::PlusEquals => CompoundAssignOperatorKind::AddAssign(p),
                PuncKind::MinusEquals => CompoundAssignOperatorKind::SubtractAssign(p),
                PuncKind::ForwardSlashEquals => CompoundAssignOperatorKind::DivideAssign(p),
                _ => return None,
            }
        } else {
            return None;
        };

        Some(operator_kind)
    }
}

impl Peek for LazyBoolOperatorKind {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let operator_kind = if let Ok(p) = peeker.peek_punctuation() {
            match p.punc_kind {
                PuncKind::DblAmpersand => LazyBoolOperatorKind::LazyAnd(p),
                PuncKind::DblPipe => LazyBoolOperatorKind::LazyOr(p),
                _ => return None,
            }
        } else {
            return None;
        };

        Some(operator_kind)
    }
}
