mod comment;
pub use crate::comment::{Comment, DocComment};

mod delimiter;
pub use crate::delimiter::{DelimKind, Delimiter, DelimiterError};

mod keyword;
pub use crate::keyword::Keyword;

mod literal;
pub use crate::literal::Literal;

mod primitive;
pub use crate::primitive::Primitive;

mod punctuation;
pub use crate::punctuation::Punctuation;

mod span;
pub use crate::span::{Span, Spanned};

mod identifier {
    use std::sync::Arc;

    use crate::span::{Span, Spanned};

    #[derive(Debug)]

    pub struct Identifier {
        pub name: String,
        span: Span,
    }

    impl Identifier {
        pub fn new(name: String, input: &str, start: usize, end: usize) -> Self {
            Self {
                name,
                span: Span::new(Arc::new(input.to_string()), start, end),
            }
        }
    }

    impl Spanned for Identifier {
        fn span(&self) -> &Span {
            &self.span
        }
    }
}

pub use crate::identifier::Identifier;
