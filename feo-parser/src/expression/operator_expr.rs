use feo_ast::expression::{
    ArithmeticOrLogicalExpr, ArithmeticOrLogicalOperatorKind, AssignmentExpr, ComparisonExpr,
    ComparisonOperatorKind, CompoundAssignOperatorKind, CompoundAssignmentExpr, DereferenceExpr,
    LazyBoolExpr, LazyBoolOperatorKind, NegationExpr, NegationOperatorKind, ReferenceExpr,
    TypeCastExpr, UnderscoreExpr, UnwrapExpr, UnwrapOperandKind,
};
use feo_error::handler::ErrorEmitted;
use feo_types::{punctuation::PuncKind, Punctuation};

use crate::{
    parse::{Parse, Peek},
    parser::{Parser, Peeker},
};

impl Peek for ArithmeticOrLogicalOperatorKind {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let operator_kind = if let Some(p) = Punctuation::peek(peeker) {
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
        let operator_kind = if let Some(p) = Punctuation::peek(peeker)  {
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
        let operator_kind = if let Some(p) = Punctuation::peek(peeker)  {
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
        let operator_kind = if let Some(p) = Punctuation::peek(peeker)  {
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

impl Peek for NegationOperatorKind {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let operator_kind = if let Some(p) = Punctuation::peek(peeker)  {
            match p.punc_kind {
                PuncKind::Minus => NegationOperatorKind::InvertNumeric(p),
                PuncKind::Bang => NegationOperatorKind::InvertBool(p),
                _ => return None,
            }
        } else {
            return None;
        };

        Some(operator_kind)
    }
}

// TODO: how ??
impl Peek for UnwrapOperandKind {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for ArithmeticOrLogicalExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for AssignmentExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for ComparisonExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for CompoundAssignmentExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for DereferenceExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for LazyBoolExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for NegationExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for ReferenceExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for TypeCastExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for UnderscoreExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl Parse for UnwrapExpr {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, ErrorEmitted>
    where
        Self: Sized,
    {
        todo!()
    }
}
