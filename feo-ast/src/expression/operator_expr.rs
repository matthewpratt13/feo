use crate::{
    type_utils::{Bang, Equals, OpArithmeticOrLogical, OpBool, OpComparison, QuestionMark},
    keyword::KeywordKind,
    ty::Type,
};

use super::ExpressionKind;

pub enum OperatorExprKind {
    ArithmeticOrLogical(ArithmeticOrLogicalExpr),
    Assign(AssignmentExpr),
    Bool(BoolExpr),
    Comparison(ComparisonExpr),
    Negation(NegationExpr),
    ResultUnwrap(ResultUnwrapExpr),
    TypeCast(TypeCastExpr),
}

pub struct ArithmeticOrLogicalExpr {
    first_operand: Box<ExpressionKind>,
    operator: OpArithmeticOrLogical,
    second_operand: Box<ExpressionKind>,
}

pub struct AssignmentExpr {
    initial_value: Box<ExpressionKind>,
    equals: Equals,
    new_value: Box<ExpressionKind>,
}

pub struct BoolExpr {
    first_expression: Box<ExpressionKind>,
    operator: OpBool,
    second_expression: Box<ExpressionKind>,
}

pub struct ComparisonExpr {
    first_expression: Box<ExpressionKind>,
    operator: OpComparison,
    second_expression: Box<ExpressionKind>,
}

pub struct NegationExpr {
    negator: Bang,
    expression: Box<ExpressionKind>,
}

pub struct ResultUnwrapExpr {
    expression: Box<ExpressionKind>,
    question_mark: QuestionMark,
}

pub struct TypeCastExpr {
    original_expression: Box<ExpressionKind>,
    kw_as: KeywordKind,
    new_type: Box<Type>,
}
