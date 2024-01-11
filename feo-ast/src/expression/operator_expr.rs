use crate::{keyword::KeywordKind, punctuation::PuncKind, ty::Type};

use super::Expression;

pub enum OperatorExpr {
    ArithmeticOrLogical(ArithmeticOrLogicalExpr),
    Assign(AssignmentExpr),
    Bool(BoolExpr),
    Comparison(ComparisonExpr),
    Negation(NegationExpr),
    OptionUnwrap(OptionUnwrapExpr),
    TypeCast(TypeCastExpr),
}

pub struct ArithmeticOrLogicalExpr {
    first_operand: Box<Expression>,
    operator: PuncKind,
    second_operand: Box<Expression>,
}

pub struct AssignmentExpr {
    initial_value: Box<Expression>,
    operator: PuncKind,
    new_value: Box<Expression>,
}

pub struct ComparisonExpr {
    first_expression: Box<Expression>,
    operator: PuncKind,
    second_expression: Box<Expression>,
}

pub struct BoolExpr {
    first_expression: Box<Expression>,
    operator: PuncKind,
    second_expression: Box<Expression>,
}

pub struct NegationExpr {
    negator: PuncKind,
    expression: Box<Expression>,
}

pub struct OptionUnwrapExpr {
    expression: Box<Expression>,
    question_mark: PuncKind,
}

pub struct TypeCastExpr {
    original_expression: Box<Expression>,
    kw_as: KeywordKind,
    new_type: Type,
}
