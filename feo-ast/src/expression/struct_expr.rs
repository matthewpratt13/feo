use super::path_expr::SimplePath;

pub enum StructExpr {
    Struct(Struct),
    Tuple(Tuple),
    Unit(Unit),
}

pub struct Struct {}

pub struct Tuple {}

pub struct Unit(SimplePath);


