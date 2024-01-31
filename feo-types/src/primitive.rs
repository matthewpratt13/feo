use crate::{
    span::{Span, Spanned},
    U256,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Primitive<P>(pub P);

impl<P> Primitive<P>
where
    P: Clone,
{
    pub fn new(raw_value: P) -> Primitive<P> {
        Primitive(raw_value)
    }

    pub fn raw_value(&self) -> P {
        self.clone().0
    }
}

impl Spanned for Primitive<char> {
    fn span(&self) -> Span {
        Span::default()
    }
}

// impl Spanned for Primitive<&str> {
//     fn span(&self) -> Span {
//         Span::default()
//     }
// }

impl Spanned for Primitive<String> {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for Primitive<u8> {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for Primitive<u16> {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for Primitive<u32> {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for Primitive<u64> {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for Primitive<U256> {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for Primitive<i32> {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for Primitive<i64> {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for Primitive<f32> {
    fn span(&self) -> Span {
        Span::default()
    }
}

impl Spanned for Primitive<f64> {
    fn span(&self) -> Span {
        Span::default()
    }
}

// impl Spanned for Primitive<[u8; 32]> {
//     fn span(&self) -> Span {
//         Span::default()
//     }
// }

impl Spanned for Primitive<bool> {
    fn span(&self) -> Span {
        Span::default()
    }
}
