pub enum OperatorExpr {
    ArithmeticOrLogical,
    Assignment,
    CompoundAssignment,
    Comparison,
    LazyBool,
    Negation,
    OptionUnwrap,
    TypeCast,
}

pub struct ArithmeticOrLogicalExpr {}

pub struct AssignmentExpr {}

pub struct CompoundAssigExpr {}

pub struct ComparisonExpr {}

pub struct LazyBoolExpr {}

pub struct NegationExpr {}

pub struct OptionUnwrapExpr {}

pub struct TypeCastExpr {}
