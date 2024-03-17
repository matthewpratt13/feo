use feo_types::{
    span::{Span, Spanned},
    Identifier,
};

use super::Value;

#[derive(Debug, Clone)]
pub struct FieldAccessExpr {
    pub container_operand: Box<Value>,
    pub field_name: Identifier,
}

impl Spanned for FieldAccessExpr {
    fn span(&self) -> Span {
        let s1 = self.container_operand.span();
        let s2 = self.field_name.span();

        Span::join(s1, s2)
    }
}
