use crate::{
    keyword::KeywordKind,
    ty::Type,
    type_utils::{Bang, Equals, OpArithmeticOrLogical, OpBool, OpComparison, QuestionMark},
};

use super::Expression;

// pub enum OperatorExprKind {
//     ArithmeticOrLogical(ArithmeticOrLogicalExpr),
//     Assign(AssignmentExpr),
//     Bool(BoolExpr),
//     Comparison(ComparisonExpr),
//     Negation(NegationExpr),
//     ResultUnwrap(ResultUnwrapExpr),
//     TypeCast(TypeCastExpr),
// }

pub struct ArithmeticOrLogicalExpr {
    first_operand: Box<dyn Expression>,
    operator: OpArithmeticOrLogical,
    second_operand: Box<dyn Expression>,
}

pub struct AssignmentExpr {
    initial_value: Box<dyn Expression>,
    equals: Equals,
    new_value: Box<dyn Expression>,
}

pub struct BoolExpr {
    first_expression: Box<dyn Expression>,
    operator: OpBool,
    second_expression: Box<dyn Expression>,
}

pub struct ComparisonExpr {
    first_expression: Box<dyn Expression>,
    operator: OpComparison,
    second_expression: Box<dyn Expression>,
}

pub struct NegationExpr {
    negator: Bang,
    expression: Box<dyn Expression>,
}

pub struct ResultUnwrapExpr {
    expression: Box<dyn Expression>,
    question_mark: QuestionMark,
}

pub struct TypeCastExpr {
    original_expression: Box<dyn Expression>,
    kw_as: KeywordKind,
    new_type: Box<Type>,
}
