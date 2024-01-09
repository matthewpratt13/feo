use super::path_expr::SimplePath;

pub enum StructExpr {
    Struct(Struct),
    TupleStruct(TupleStruct),
    UnitStruct(UnitStruct),
}

pub struct Struct {}

pub struct TupleStruct {}

pub struct UnitStruct(SimplePath);
