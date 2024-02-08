use feo_types::{
    span::{Span, Spanned},
    utils::DotDotEquals,
};

// pub trait RangePatt
// where
//     Self: Sized + Pattern + 'static,
// {
// }

#[derive(Clone)]
pub enum RangePatt {
    RangeFromPatt(RangeFromPatt),
    RangeInclusivePatt(RangeInclusivePatt),
    RangeToInclusivePatt(RangeToInclusivePatt),
}

pub trait RangePattBound
where
    Self: Spanned,
{
}

#[derive(Clone)]
pub struct RangeFromPatt {
    from: Box<dyn RangePattBound>,
    dot_dot_equals: DotDotEquals,
}

impl Spanned for RangeFromPatt {
    fn span(&self) -> Span {
        let start_pos = self.from.span().start();
        let end_pos = self.dot_dot_equals.span().end();
        let source = self.from.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Clone)]
pub struct RangeInclusivePatt {
    from: Box<dyn RangePattBound>,
    dot_dot_equals: DotDotEquals,
    to: Box<dyn RangePattBound>,
}

impl Spanned for RangeInclusivePatt {
    fn span(&self) -> Span {
        let start_pos = self.from.span().start();
        let end_pos = self.to.span().end();
        let source = self.from.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Clone)]
pub struct RangeToInclusivePatt {
    from: Box<dyn RangePattBound>,
    dot_dot_equals: DotDotEquals,
    to: Box<dyn RangePattBound>,
}

impl Spanned for RangeToInclusivePatt {
    fn span(&self) -> Span {
        let start_pos = self.from.span().start();
        let end_pos = self.to.span().end();
        let source = self.from.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
