use crate::{
    primitive::{Primitive, PrimitiveType},
    span::{Span, Spanned},
};

#[derive(Debug, Clone)]
pub struct Literal<L: 'static + Primitive + Clone> {
    raw_value: L,
    span: Span,
}

impl<L> PrimitiveType<L> for Literal<L>
where
    L: 'static + Primitive + Clone,
{
    fn new(raw_value: L, span: Span) -> Self {
        Self { raw_value, span }
    }

    fn raw_value(&self) -> &L {
        &self.raw_value
    }
}

impl<L> Spanned for Literal<L>
where
    L: 'static + Primitive + Clone,
{
    fn span(&self) -> Span {
        self.clone().span
    }
}
