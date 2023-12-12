mod comment;
pub use crate::comment::{Comment, DocComment};

mod delimiter;
pub use crate::delimiter::{Delimiter, DelimiterError};

mod keyword;
pub use crate::keyword::Keyword;

mod literal;
pub use crate::literal::Literal;

mod primitive;
pub use crate::primitive::Primitive;

mod punctuation;
pub use crate::punctuation::Punctuation;

mod span;
pub use crate::span::{Span, SpanError, Spanned};

mod identifier {
    use crate::span::{Span, Spanned};

    pub struct Identifier {
        name: String,
        span: Span,
    }

    impl Spanned for Identifier {
        fn span(&self) -> &Span {
            &self.span
        }
    }
}

pub use crate::identifier::Identifier;
