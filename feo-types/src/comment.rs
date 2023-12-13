use crate::{
    primitive::{Primitive, PrimitiveType},
    span::{Span, Spanned},
};

#[derive(Debug)]
pub struct Comment<C> {
    content: C,
    span: Span,
}

impl<C> PrimitiveType<C> for Comment<C>
where
    C: 'static + Primitive,
{
    fn new(raw_value: C, span: Span) -> Self {
        Self {
            content: raw_value,
            span,
        }
    }

    fn raw_value(&self) -> &C {
        &self.content
    }
}

impl<C> Spanned for Comment<C> {
    fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug)]
pub struct DocComment<D> {
    content: D,
    span: Span,
}

impl<D> PrimitiveType<D> for DocComment<D>
where
    D: 'static + Primitive,
{
    fn new(raw_value: D, span: Span) -> Self {
        Self {
            content: raw_value,
            span,
        }
    }

    fn raw_value(&self) -> &D {
        &self.content
    }
}

impl<D> Spanned for DocComment<D> {
    fn span(&self) -> &Span {
        &self.span
    }
}
