use std::str::FromStr;

use feo_types::{
    span::Span, DelimKind, DelimOrientation, Delimiter, DocComment, Identifier, Keyword,
    KeywordKind, Literal, PrimitiveType, PuncKind, Punctuation, TypeAnnotation, TypeName,
};

mod lexer;
use lexer::{Lexer, Token};

mod literals;
use literals::{BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral};

mod parse;
use parse::Parse;

impl Parse for CharLiteral {
    fn parse(src: &str, content: &str, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let parsed = content.parse::<char>().map_err(|_| ())?;

        let char_lit = Literal::new(parsed, span);

        let token = Token::CharLit(CharLiteral(char_lit));

        Ok(Some(token))
    }
}

impl Parse for StringLiteral {
    fn parse(src: &str, content: &str, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let string_lit = Literal::new(content.to_string(), span);

        let token = Token::StringLit(StringLiteral(string_lit));

        Ok(Some(token))
    }
}

impl Parse for BoolLiteral {
    fn parse(src: &str, content: &str, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let parsed = content.parse::<bool>().map_err(|_| ())?;

        let bool_lit = Literal::new(parsed, span);

        let token = Token::BoolLit(BoolLiteral(bool_lit));

        Ok(Some(token))
    }
}

impl Parse for IntLiteral {
    fn parse(src: &str, content: &str, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let parsed = i64::from_str_radix(content, 10 | 16).map_err(|_| ())?;

        let int_lit = Literal::new(parsed, span);

        let token = Token::IntLit(IntLiteral(int_lit));

        Ok(Some(token))
    }
}

impl Parse for UIntLiteral {
    fn parse(src: &str, content: &str, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let parsed = u64::from_str_radix(content, 10 | 16).map_err(|_| (()))?;

        let uint_lit = Literal::new(parsed, span);

        let token = Token::UIntLit(UIntLiteral(uint_lit));

        Ok(Some(token))
    }
}

impl Parse for FloatLiteral {
    fn parse(src: &str, content: &str, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let parsed = content.parse::<f64>().map_err(|_| (()))?;

        let float_lit = Literal::new(parsed, span);

        let token = Token::FloatLit(FloatLiteral(float_lit));

        Ok(Some(token))
    }
}

impl Parse for Identifier {
    fn parse(src: &str, content: &str, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let iden = Identifier::new(content.to_string(), span);

        let token = Token::Iden(iden);

        Ok(Some(token))
    }
}

impl Parse for Keyword {
    fn parse(src: &str, content: &str, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let keyword_kind = KeywordKind::from_str(content).map_err(|_| (()))?;

        let keyword = Keyword::new(keyword_kind, span);

        let token = Token::Keyword(keyword);

        Ok(Some(token))
    }
}

impl Parse for DocComment {
    fn parse(src: &str, content: &str, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let doc_comment = DocComment::new(content.to_string(), span);

        let token = Token::DocComment(doc_comment);

        Ok(Some(token))
    }
}

impl Parse for Delimiter {
    fn parse(src: &str, content: &str, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let delim_kind = DelimKind::from_str(content).map_err(|_| (()))?;

        let delim_orientation = DelimOrientation::from_str(content).map_err(|_| (()))?;

        let delim = Delimiter::new(delim_kind, delim_orientation, span);

        let token = Token::Delim(delim);

        Ok(Some(token))
    }
}

impl Parse for Punctuation {
    fn parse(src: &str, content: &str, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let punc_kind = PuncKind::from_str(content).map_err(|_| (()))?;

        let punc = Punctuation::new(punc_kind, span);

        let token = Token::Punc(punc);

        Ok(Some(token))
    }
}

impl Parse for TypeAnnotation {
    fn parse(src: &str, content: &str, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let type_name = TypeName::from_str(content)?;

        let type_ann = TypeAnnotation::new(type_name, span);

        let token = Token::Type(type_ann);

        Ok(Some(token))
    }
}

impl TryFrom<Token> for Delimiter {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Delim(d) => Ok(d),
            _ => return Err(()),
        }
    }
}

impl TryFrom<Token> for Punctuation {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Punc(p) => Ok(p),
            _ => return Err(()),
        }
    }
}

pub fn lex() {
    // let filename = "path/to/your/file.txt"; // Change this to your file path
    // let source_code = std::fs::read_to_string(filename).expect("Error reading file");

    let source_code = r#"
        // line comment
        /// outer doc comment
        /* 
        block comment
        */

        /!
        module doc comment
        */

        let foo = "bar";

        let baz = -10;

        let foo = false;

        let bar: u32 = 10;

        let baz = 'a';
        "#;

    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.tokenize();

    if let Ok(t) = tokens {
        for token in t.tokens() {
            println!("{:?}", token);
        }
    } else {
        println!("Error tokenizing file");
    }
}
