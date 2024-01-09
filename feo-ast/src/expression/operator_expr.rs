pub enum OperatorExpr {
    ArithmeticOrLogical(ArithmeticOrLogicalExpr),
    Assignment(AssignExpr),
    CompoundAssignment(CompoundAssignExpr),
    Comparison(ComparisonExpr),
    LazyBool(LazyBoolExpr),
    Negation(NegationExpr),
    OptionUnwrap(OptionUnwrapExpr),
    TypeCast(TypeCastExpr),
}

pub struct ArithmeticOrLogicalExpr {}

pub struct AssignExpr {}

pub struct CompoundAssignExpr {}

pub struct ComparisonExpr {}

pub struct LazyBoolExpr {}

pub struct NegationExpr {}

pub struct OptionUnwrapExpr {}

pub struct TypeCastExpr {}
