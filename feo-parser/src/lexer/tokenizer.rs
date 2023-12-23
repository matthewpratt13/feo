use std::str::FromStr;
use std::sync::Arc;

use feo_types::span::Span;
use feo_types::{
    DelimKind, DelimOrientation, Delimiter, DocComment, Identifier, Keyword, KeywordKind, Literal,
    PrimitiveType, PuncKind, Punctuation, TypeAnnotation, TypeName,
};

use crate::lexer::Token;
use crate::literals::{
    BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral,
};

pub trait Tokenize {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ()>;
}

impl Tokenize for CharLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let parsed = content.parse::<char>().map_err(|_| ())?;

        let char_lit = Literal::new(parsed, span);

        let token = Token::CharLit(CharLiteral(char_lit));

        Ok(Some(token))
    }
}

impl Tokenize for StringLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let string_lit = Literal::new(content.to_string(), span);

        let token = Token::StringLit(StringLiteral(string_lit));

        Ok(Some(token))
    }
}

impl Tokenize for BoolLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let parsed = content.parse::<bool>().map_err(|_| ())?;

        let bool_lit = Literal::new(parsed, span);

        let token = Token::BoolLit(BoolLiteral(bool_lit));

        Ok(Some(token))
    }
}

impl Tokenize for IntLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let parsed = i64::from_str_radix(content, 10).map_err(|_| ())?;

        let int_lit = Literal::new(parsed, span);

        let token = Token::IntLit(IntLiteral(int_lit));

        Ok(Some(token))
    }
}

impl Tokenize for UIntLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let parsed = u64::from_str_radix(content, 10).map_err(|_| (()))?;

        let uint_lit = Literal::new(parsed, span);

        let token = Token::UIntLit(UIntLiteral(uint_lit));

        Ok(Some(token))
    }
}

impl Tokenize for FloatLiteral {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let parsed = content.parse::<f64>().map_err(|_| (()))?;

        let float_lit = Literal::new(parsed, span);

        let token = Token::FloatLit(FloatLiteral(float_lit));

        Ok(Some(token))
    }
}

impl Tokenize for Identifier {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let iden = Identifier::new(content.to_string(), span);

        let token = Token::Iden(iden);

        Ok(Some(token))
    }
}

impl Tokenize for Keyword {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let keyword_kind = KeywordKind::from_str(content).map_err(|_| (()))?;

        let keyword = Keyword::new(keyword_kind, span);

        let token = Token::Keyword(keyword);

        Ok(Some(token))
    }
}

impl Tokenize for DocComment {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let doc_comment = DocComment::new(content.to_string(), span);

        let token = Token::DocComment(doc_comment);

        Ok(Some(token))
    }
}

impl Tokenize for Delimiter {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let delim_kind = DelimKind::from_str(content).map_err(|_| (()))?;

        let delim_orientation = DelimOrientation::from_str(content).map_err(|_| (()))?;

        let delim = Delimiter::new(delim_kind, delim_orientation, span);

        let token = Token::Delim(delim);

        Ok(Some(token))
    }
}

impl Tokenize for Punctuation {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let punc_kind = PuncKind::from_str(content).map_err(|_| (()))?;

        let punc = Punctuation::new(punc_kind, span);

        let token = Token::Punc(punc);

        Ok(Some(token))
    }
}

impl Tokenize for TypeAnnotation {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let type_name = TypeName::from_str(content)?;

        let type_ann = TypeAnnotation::new(type_name, span);

        let token = Token::Type(type_ann);

        Ok(Some(token))
    }
}
