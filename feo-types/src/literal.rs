use crate::{
    primitive::{Primitive, PrimitiveType},
    span::{Span, Spanned},
};

pub struct Literal<L: 'static + Primitive> {
    raw_value: L,
    span: Span,
}

impl<L> PrimitiveType<L> for Literal<L>
where
    L: 'static + Primitive,
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
    L: 'static + Primitive,
{
    fn span(&self) -> &Span {
        &self.span
    }
}
