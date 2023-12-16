use std::sync::Arc;

use feo_types::Primitive;

use crate::lexer::{Lexer, Token};

// use for delimiters, literals, punctuation
pub trait Parse {
    fn parse(l: &mut Lexer) -> Option<Token>;
}

// use for comments, doc comments, keywords, identifiers, path expressions, type annotations
pub trait ParseData<T>
where
    T: 'static + Primitive,
{
    fn parse(src: Arc<String>, content: T, start: usize, end: usize) -> Option<Token>;
}

///////////////////////////////////////////////////////////////////////////////

// these are parsed …

//  1/ `Punctuation` : `char` or `String` -> Punctuation { PuncKind, Span }
// 2/ `Delimiter` : `char` -> Delimiter { (DelimKind, Span) }
// 3/ `Keyword` : `String` -> Keyword { KeywordKind, Span }
// 4/ `TypeAnnotation` : `String` -> TypeAnnotation { TypeName, Span }
// 5/  `StringLiteral` : `String` -> `Literal<String>`
// 6/ `CharLiteral` : `String` -> `Literal<char>`
// 7/ `BoolLiteral` : `String` -> `Literal<bool>`
// 8/ `IntLiteral` : `String` -> `Literal<i64>`
// 9/ `UIntLiteral` : `String` -> `Literal<u64>`
// 10/ `FloatLiteral` : `String` -> `Literal<f64>`

// then tokenized:

// 1/ Punctuation { PuncKind, Span} -> Token::Punc(Punctuation)
// 2/ Delimiter { DelimKind, Span} -> Token::Delim(Delimiter)
// 3/ Keyword { KeywordKind, Span} -> Token::Keyword(Keyword)
// 4/ TypeAnnotation { TypeName, Span} -> Token::Type(TypeAnnotation)
// 5/ StringLiteral { Literal<String>, Span} -> Token::String(StringLiteral)
// 6/ CharLiteral { Literal<char>, Span} -> Token::Char(CharLiteral)
// 7/ BoolLiteral { Literal<bool>, Span} -> Token::Bool(BoolLiteral)
// 8/ IntLiteral { Literal<i64>, Span} -> Token::Int(IntLiteral)
// 9/ UintLiteral { Literal<u64>, Span} -> Token::Uint(UintLiteral)
// 10/ FloatLiteral { Literal<f64>, Span} -> Token::Float(FloatLiteral)

///////////////////////////////////////////////////////////////////////////////

// these are not parsed but directly tokenized (direct reference to the source code):

// 11/ `Comment` { String, Span } -> Token::Comment(Comment)
// 12/ `DocComment` { String, Span } -> Token::DocComment(DocComment)
// 13/ `Identifier` { String, Span } -> Token::Identifier(Identifier)
// 14/ `PathExpression` { Vec<String>, Span } -> Token::PathExpression(PathExpression)

fn parse(input: &str, start: usize, end: usize) -> Self {}

let lit = StringLiteral::parse(self.input, start_pos, self.pos);

// in tokenize():

(match '"') ...
self.advance();
let start_pos = self.pos;
let mut buf = String::new();

while Some(c) = self.current_char() {
    if c != '"' {
        buf.push(c);
    } else {
        break
    }
}

let string_lit = StringLiteral::parse(self.input, buf, start_pos, self.pos)



