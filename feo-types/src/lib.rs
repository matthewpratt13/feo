use uint::construct_uint;

construct_uint! {
    pub struct U256(4);
}

pub mod comment;
pub use crate::comment::Comment;

pub mod delimiter;
pub use crate::delimiter::Delimiter;

pub mod doc_comment;
pub use crate::doc_comment::DocComment;

pub mod error;

pub mod identifier;
pub use crate::identifier::Identifier;

pub mod keyword;
pub use crate::keyword::Keyword;

pub mod literal;
pub use crate::literal::Literal;

pub mod primitive;

pub mod punctuation;
pub use crate::punctuation::Punctuation;

pub mod span;

pub mod utils;
