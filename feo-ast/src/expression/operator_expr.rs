use crate::{
    item::{BangOrMinus, Equals, OpArithmeticOrLogical, OpBool, OpComparison, QuestionMark},
    keyword::KeywordKind,
    ty::Type,
};

use super::Expression;

pub enum OperatorExpr {
    ArithmeticOrLogical(ArithmeticOrLogicalExpr),
    Assign(AssignmentExpr),
    Bool(BoolExpr),
    Comparison(ComparisonExpr),
    Negation(NegationExpr),
    ResultUnwrap(ResultUnwrapExpr),
    TypeCast(TypeCastExpr),
}

pub struct ArithmeticOrLogicalExpr {
    first_operand: Box<Expression>,
    operator: OpArithmeticOrLogical,
    second_operand: Box<Expression>,
}

pub struct AssignmentExpr {
    initial_value: Box<Expression>,
    equals: Equals,
    new_value: Box<Expression>,
}

pub struct ComparisonExpr {
    first_expression: Box<Expression>,
    operator: OpComparison,
    second_expression: Box<Expression>,
}

pub struct BoolExpr {
    first_expression: Box<Expression>,
    operator: OpBool,
    second_expression: Box<Expression>,
}

pub struct NegationExpr {
    negator: BangOrMinus,
    expression: Box<Expression>,
}

pub struct ResultUnwrapExpr {
    expression: Box<Expression>,
    question_mark: QuestionMark,
}

pub struct TypeCastExpr {
    original_expression: Box<Expression>,
    kw_as: KeywordKind,
    new_type: Type,
}
