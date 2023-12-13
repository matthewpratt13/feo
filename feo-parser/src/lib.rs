use std::arch::x86_64::_SIDD_CMP_EQUAL_ANY;
use std::iter::Iterator;
use std::sync::Arc;

use feo_error::ParserErrorKind;
use feo_types::{Comment, CommentKind, Delimiter, Identifier, Keyword, Punctuation, Span};

mod lexer;
use crate::lexer::{Lexer, Token};

mod literals;
use crate::literals::{
    BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral,
};

mod parse;
use crate::parse::{Parse, ParseDigit};

impl Parse for Comment {
    fn parse(l: &mut Lexer) -> Result<Option<Token>, ParserErrorKind> {
        match l.peek_next() {
            // newline / trailing comment
            Some('/') => {
                l.advance(); // skip first '/'
                l.advance(); // skip second '/'

                //  (newline) doc comment
                if let Some('/') = l.current_char() {
                    l.advance(); // skip third '/'
                    let mut comment_content = String::new();

                    while let Some(c) = l.current_char() {
                        if c == '\n' {
                            break;
                        } else {
                            comment_content.push(c);
                            l.advance();
                        }
                    }

                    let span = Span::build(l.input, l.input.len(), l.pos)?;

                    let comment = Comment::new(CommentKind::OuterDocComment, span);
                    return Ok(Some(Token::Comment(comment)));
                } else {
                    // normal comment
                    while let Some(c) = l.current_char() {
                        if c == '\n' {
                            break;
                        } else {
                            comment_content.push(c);
                            l.advance();
                        }
                    }
                }
            }

            // module-level doc comments (multiline)
            Some('!') => {
                l.advance(); // skip '/'
                l.advance(); // skip '!'
                in_block_comment = true;
                let start_pos = l.pos;

                while let Some(c) = l.current_char() {
                    if c == '*' && l.peek_next() == Some('/') {
                        l.advance(); // skip '*'
                        l.advance(); // skip '/'
                        in_block_comment = false;
                        let end_pos = l.pos;
                        let src = Arc::new(l.input[start_pos..end_pos].trim().to_string());

                        let span = Span::build(src, start_pos, end_pos)?;

                        let comment = Comment::new(CommentKind::ModuleDocComment, span);
                        return Ok(Some(Token::Comment(comment)));
                    } else {
                        l.advance();
                    }
                }

                // if we reach here, the block comment is unterminated
                l.log_error("Unterminated doc comment");
            }

            // inline or multiline comments
            Some('*') => {
                // open ('*/')
                l.advance(); // skip '/'
                l.advance(); // skip '*'
                in_block_comment = true;

                while let Some(c) = l.current_char() {
                    if c == '*' && l.peek_next() == Some('/') {
                        // terminate ('*/')
                        l.advance(); // skip '*'
                        l.advance(); // skip '/'
                        in_block_comment = false;
                        break;
                    } else {
                        l.advance();
                    }
                }

                // if we reach here, the block comment is unterminated
                l.log_error("Unterminated block comment");
            }

            Some(_) | None => {
                l.log_error("Unexpected comment");
            }
        }
    }
}

// impl<I> Parse<I> for Delimiter
// where
//     I: Iterator,
// {
//     fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> Parse<I> for DocComment
// where
//     I: Iterator,
// {
//     fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> Parse<I> for Identifier
// where
//     I: Iterator,
// {
//     fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> Parse<I> for Keyword
// where
//     I: Iterator,
// {
//     fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> Parse<I> for Punctuation
// where
//     I: Iterator,
// {
//     fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> Parse<I> for BoolLiteral
// where
//     I: Iterator,
// {
//     fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> Parse<I> for CharLiteral
// where
//     I: Iterator,
// {
//     fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> Parse<I> for StringLiteral
// where
//     I: Iterator,
// {
//     fn parse(src: &mut I, input: char, i: usize) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> ParseDigit<I> for FloatLiteral
// where
//     I: Iterator,
// {
//     fn parse(
//         src: &mut I,
//         input: char,
//         i: usize,
//         is_negative_number: bool,
//         is_hexadecimal_int: bool,
//     ) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> ParseDigit<I> for IntLiteral
// where
//     I: Iterator,
// {
//     fn parse(
//         src: &mut I,
//         input: char,
//         i: usize,
//         is_negative_number: bool,
//         is_hexadecimal_int: bool,
//     ) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }

// impl<I> ParseDigit<I> for UIntLiteral
// where
//     I: Iterator,
// {
//     fn parse(
//         src: &mut I,
//         input: char,
//         i: usize,
//         is_negative_number: bool,
//         is_hexadecimal_int: bool,
//     ) -> Result<Option<Token>, ParserErrorKind> {
//         todo!()
//     }
// }
