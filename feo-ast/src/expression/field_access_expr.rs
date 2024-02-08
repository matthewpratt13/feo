use feo_types::{
    span::{Span, Spanned},
    utils::Dot,
};

use crate::item::StructFieldName;

use super::Assignable;

#[derive(Clone)]
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

        // let start_pos = self.container_operand.span().start();
        // let end_pos = self.field_name.span().end();
        // let source = self.container_operand.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}
