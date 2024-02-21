use feo_types::{
    span::{Span, Spanned},
    utils::Dot,
};

use crate::item::StructFieldName;

use super::Assignable;

#[derive(Debug, Clone)]
pub struct FieldAccessExpr {
    container_operand: Box<Assignable>,
    dot: Dot,
    field_name: StructFieldName,
}

impl Spanned for FieldAccessExpr {
    fn span(&self) -> Span {
        let s1 = self.container_operand.span();
        let s2 = self.field_name.span();

        Span::join(s1, s2)
    }
}
