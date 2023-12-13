use crate::{
    primitive::{Primitive, PrimitiveType},
    span::{Span, Spanned},
};

#[derive(Debug)]
pub struct Identifier<I> {
    name: I,
    span: Span,
}

impl<I> PrimitiveType<I> for Identifier<I>
where
    Self: Sized,
    I: 'static + Primitive,
{
    fn new(raw_value: I, span: Span) -> Self {
        Self {
            name: raw_value,
            span,
        }
    }

    fn raw_value(&self) -> &I {
        &self.name
    }
}

impl<I> Spanned for Identifier<I> {
    fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Debug)]
pub struct PathExpression<P> {
    path: P,
    span: Span,
}

impl<P> PrimitiveType<P> for PathExpression<P>
where
    Self: Sized,
    P: 'static + Primitive,
{
    fn new(raw_value: P, span: Span) -> Self {
        Self {
            path: raw_value,
            span,
        }
    }

    fn raw_value(&self) -> &P {
        &self.path
    }
}

impl<P> Spanned for PathExpression<P> {
    fn span(&self) -> &Span {
        &self.span
    }
}
