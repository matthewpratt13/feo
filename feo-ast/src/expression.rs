mod array_expr;
mod attribute_expr;
mod block_expr;
mod call_expr;
mod conditional_expr;
mod iterator_expr;
mod literal_expr;
mod operator_expr;

mod path_expr;
pub use self::path_expr::PathExpr;

mod struct_expr;

pub enum Expression {
    ExprWithoutBlock,
    ExprWithBlock,
}

pub enum ExprWithoutBlock {
    Attribute,
    ArrayExpr,
    FunctionCallExpr,
    MethodCallExpr,
    BreakExpr,
    ContinueExpr,
    FieldAccessExpr,
    GroupedExpr,
    IndexExpr,
    LiteralExpr,
    OperatorExpr,
    PathExpr,
    StructExpr,
    TupleExpr,
    TupleIndexingExpr,
    ReturnExpr,
    UnderscoreExpr,
}

pub enum ExprWithBlock {
    Attribute,
    BlockExpr,
    IfExpr,
    LoopExpr,
    MatchExpr,
}

pub struct GroupedExpr {}

pub struct FieldAccessExpr {}

pub struct ReturnExpr {}

pub struct UnderscoreExpr {}
