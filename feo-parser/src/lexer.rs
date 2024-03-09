#![allow(dead_code)]

use std::{iter::Peekable, str::Chars, sync::Arc};

use feo_ast::{
    token::{Token, TokenStream},
    tokenize::Tokenize,
};

use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    lex_error::{LexError, LexErrorKind},
};

use feo_types::{
    delimiter, identifier,
    literal::{FloatType, IntType, UIntType},
    punctuation,
    type_annotation::{self, TypeAnnotation},
};
use feo_types::{
    literal::Literal, span::Position, Comment, Delimiter, DocComment, Identifier, Keyword,
    Punctuation, U256,
};

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    peekable_chars: Peekable<Chars<'a>>,
    handler: Handler,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, handler: Handler) -> Self {
        Self {
            input,
            pos: 0,
            peekable_chars: input.chars().peekable(),
            handler,
        }
    }

    pub fn errors(&self) -> Vec<CompilerError> {
        self.handler.clone().get_inner().0
    }

    // main lexer function
    // return a stream of tokens, parsed and tokenized from an input stream (i.e., source code)
    pub fn lex(&mut self) -> Result<TokenStream, ErrorEmitted> {
        let mut tokens: Vec<Option<Token>> = Vec::new();

        let mut num_open_delimiters: usize = 0; // to check for unclosed delimiters
        let mut num_closed_delimiters: usize = 0; // to check for opened delimiters

        while let Some(c) = self.current_char() {
            let start_pos = self.pos;

            match &c {
                _ if c.is_whitespace() => {
                    self.skip_whitespace();
                }

                _ if c == '/' && self.peek_next() == Some('/') || self.peek_next() == Some('*') => {
                    self.advance(); // skip first '/'
                    let mut block_comment_open = false;

                    match &self.current_char() {
                        Some('/') => {
                            self.advance(); // skip second '/'

                            // doc comment
                            if self.current_char() == Some('/') || self.current_char() == Some('!')
                            {
                                self.advance(); // skip third '/' or '!'

                                self.skip_whitespace();

                                while let Some(c) = self.current_char() {
                                    if c == '\n' {
                                        break;
                                    }

                                    self.advance();
                                }

                                let raw_content = self.input[start_pos..self.pos].to_string();

                                let doc_comment_content = Arc::new(&raw_content);

                                let doc_comment = DocComment::tokenize(
                                    &self.input,
                                    &doc_comment_content,
                                    start_pos,
                                    self.pos,
                                    &mut self.handler,
                                )?;

                                tokens.push(doc_comment);
                            } else {
                                while let Some(c) = self.current_char() {
                                    if c == '\n' {
                                        break;
                                    }

                                    self.advance();
                                }

                                let raw_data = self.input[start_pos..self.pos].to_string();

                                let comment_data = Arc::new(&raw_data);

                                let comment = Comment::tokenize(
                                    &self.input,
                                    &comment_data,
                                    start_pos,
                                    self.pos,
                                    &mut self.handler,
                                )?;

                                tokens.push(comment);
                            }
                        }

                        Some('*') => {
                            self.advance(); // skip '*'
                            block_comment_open = true;

                            while let Some(c) = self.current_char() {
                                if c == '*' {
                                    self.advance(); // skip closing '*'
                                    self.advance(); // skip closing '/'
                                    block_comment_open = false;
                                    break;
                                }

                                self.advance();
                            }

                            let raw_data = self.input[start_pos..self.pos].to_string();

                            let comment_data = Arc::new(&raw_data);

                            let comment = Comment::tokenize(
                                &self.input,
                                &comment_data,
                                start_pos,
                                self.pos,
                                &mut self.handler,
                            )?;

                            tokens.push(comment);
                        }

                        Some(_) | None => (),
                    }

                    if block_comment_open {
                        return Err(self.log_error(LexErrorKind::UnclosedBlockComment));
                    }
                }

                // identifiers and keywords
                'A'..='Z' | 'a'..='z' | '_' => {
                    let mut buf = String::new();

                    while let Some(c) = self.current_char() {
                        // cannot start with, but can contain numbers (`is_ascii_alphanumeric()`)
                        if c.is_ascii_alphanumeric() || c == '_' {
                            buf.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    if &buf == "true" || &buf == "false" {
                        let bool_literal = Literal::<bool>::tokenize(
                            &self.input,
                            &buf,
                            start_pos, // global `start_pos`
                            start_pos + buf.len(),
                            &mut self.handler,
                        )?;

                        tokens.push(bool_literal);
                        continue;
                    }

                    if identifier::is_keyword(&buf) {
                        let keyword = Keyword::tokenize(
                            &self.input,
                            &buf,
                            start_pos, // global `start_pos`
                            start_pos + buf.len(),
                            &mut self.handler,
                        )?;

                        tokens.push(keyword)
                    } else if type_annotation::is_built_in_type_annotation(&buf) {
                        let type_annotation = TypeAnnotation::tokenize(
                            &self.input,
                            &buf,
                            start_pos,
                            start_pos + buf.len(),
                            &mut self.handler,
                        )?;

                        tokens.push(type_annotation);
                    } else {
                        let identifier = Identifier::tokenize(
                            &self.input,
                            &buf,
                            start_pos, // global `start_pos`
                            start_pos + buf.len(),
                            &mut self.handler,
                        )?;

                        tokens.push(identifier);
                    }
                }

                '(' | '[' | '{' => {
                    self.advance(); // skip opening delimiter

                    match &c {
                        '(' => {
                            let delimiter = Delimiter::tokenize(
                                &self.input,
                                "(",
                                start_pos,
                                self.pos,
                                &mut self.handler,
                            )?;

                            tokens.push(delimiter);
                        }

                        '[' => {
                            let delimiter = Delimiter::tokenize(
                                &self.input,
                                "[",
                                start_pos,
                                self.pos,
                                &mut self.handler,
                            )?;

                            tokens.push(delimiter);
                        }

                        '{' => {
                            let delimiter = Delimiter::tokenize(
                                &self.input,
                                "{",
                                start_pos,
                                self.pos,
                                &mut self.handler,
                            )?;

                            tokens.push(delimiter);
                        }
                        _ => unreachable!(),
                    };

                    num_open_delimiters += 1;
                }

                ')' | ']' | '}' => {
                    self.advance(); // skip closing delimiter (advance counter for correct end pos)

                    match &c {
                        ')' => {
                            let delimiter = Delimiter::tokenize(
                                &self.input,
                                ")",
                                start_pos,
                                self.pos,
                                &mut self.handler,
                            )?;

                            tokens.push(delimiter);
                        }

                        ']' => {
                            let delimiter = Delimiter::tokenize(
                                &self.input,
                                "]",
                                start_pos,
                                self.pos,
                                &mut self.handler,
                            )?;

                            tokens.push(delimiter);
                        }

                        '}' => {
                            let delimiter = Delimiter::tokenize(
                                &self.input,
                                "}",
                                start_pos,
                                self.pos,
                                &mut self.handler,
                            )?;

                            tokens.push(delimiter);
                        }

                        _ => unreachable!(), // TODO: replace with something more meaningful
                    };

                    num_closed_delimiters += 1;

                    if num_closed_delimiters > num_open_delimiters {
                        return Err(self.log_error(LexErrorKind::UnexpectedCloseDelimiter));
                    }
                }

                '"' => {
                    // `start_pos` is global `start_pos` (above)
                    self.advance(); // skip opening '"' (double quote)

                    let mut string_literal_open = true; // to check for unclosed quotes

                    let mut buf = String::new();

                    while let Some(c) = self.current_char() {
                        match c {
                            '\\' => {
                                self.advance(); // skip '\'

                                if let Some(esc_c) = self.current_char() {
                                    self.advance(); // return escaped char

                                    match esc_c {
                                        'n' => buf.push('\n'),
                                        'r' => buf.push('\r'),
                                        't' => buf.push('\t'),
                                        '\\' => buf.push('\\'),
                                        '0' => buf.push('\0'),
                                        '\'' => buf.push('\''),
                                        '\"' => buf.push('"'),
                                        _ => {
                                            return Err(
                                                self.log_error(LexErrorKind::InvalidEscapeSequence)
                                            )
                                        }
                                    };
                                } else {
                                    // escape sequence is expected, but the input has ended
                                    return Err(
                                        self.log_error(LexErrorKind::ExpectedEscapeSequence)
                                    );
                                }
                            }

                            '"' => {
                                self.advance(); // skip closing '"'

                                string_literal_open = false;

                                let string_literal = Literal::<String>::tokenize(
                                    &self.input,
                                    &buf,
                                    start_pos,
                                    self.pos,
                                    &mut self.handler,
                                )?;

                                tokens.push(string_literal);
                                break;
                            }

                            _ => {
                                buf.push(c);
                                self.advance();
                            }
                        }
                    }

                    if string_literal_open {
                        panic!("Unclosed string literal");
                    }
                }

                '\'' => {
                    self.advance(); // skip opening '\'' (single quote)

                    let start_pos = self.pos; // start reading input after opening quote

                    if let Some(c) = self.current_char() {
                        match c {
                            '\\' => {
                                self.advance(); // skip '\'

                                let esc_char_literal = match self.current_char() {
                                    Some('n') => Literal::<char>::tokenize(
                                        &self.input,
                                        "\n",
                                        start_pos,
                                        self.pos + 1,
                                        &mut self.handler,
                                    )?,

                                    Some('r') => Literal::<char>::tokenize(
                                        &self.input,
                                        "\r",
                                        start_pos,
                                        self.pos + 1,
                                        &mut self.handler,
                                    )?,

                                    Some('t') => Literal::<char>::tokenize(
                                        &self.input,
                                        "\t",
                                        start_pos,
                                        self.pos + 1,
                                        &mut self.handler,
                                    )?,

                                    Some('\\') => Literal::<char>::tokenize(
                                        &self.input,
                                        "\\",
                                        start_pos,
                                        self.pos + 1,
                                        &mut self.handler,
                                    )?,

                                    Some('0') => Literal::<char>::tokenize(
                                        &self.input,
                                        "\0",
                                        start_pos,
                                        self.pos + 1,
                                        &mut self.handler,
                                    )?,

                                    Some('\'') => Literal::<char>::tokenize(
                                        &self.input,
                                        "\'",
                                        start_pos,
                                        self.pos + 1,
                                        &mut self.handler,
                                    )?,

                                    Some('"') => Literal::<char>::tokenize(
                                        &self.input,
                                        "\"",
                                        start_pos,
                                        self.pos + 1,
                                        &mut self.handler,
                                    )?,

                                    _ => {
                                        return Err(
                                            self.log_error(LexErrorKind::InvalidEscapeSequence)
                                        )?
                                    }
                                };

                                tokens.push(esc_char_literal);
                                self.advance(); // skip second char

                                if self.current_char() != Some('\'') {
                                    return Err(
                                        self.log_error(LexErrorKind::ExpectedClosingSingleQuote)
                                    );
                                }

                                self.advance(); // skip closing '\'' (single quote)
                            }

                            '\'' => {
                                return Err(self.log_error(LexErrorKind::EmptyCharLiteral));
                            }

                            _ => {
                                if c == ' ' {
                                    return Err(self.log_error(LexErrorKind::InvalidCharLiteral));
                                }

                                self.advance(); // return next (regular) char

                                if self.current_char() == Some('\'') {
                                    let char_literal = Literal::<char>::tokenize(
                                        &self.input,
                                        &c.to_string(),
                                        start_pos,
                                        self.pos,
                                        &mut self.handler,
                                    )?;

                                    tokens.push(char_literal);
                                    self.advance(); // skip closing '\'' (single quote)
                                } else {
                                    return Err(self.log_error(LexErrorKind::InvalidCharLiteral));
                                }
                            }
                        }
                    } else {
                        return Err(self.log_error(LexErrorKind::ExpectedCharLiteral));
                    }
                }

                // check for hexadecimal prefix
                _ if c == '0'
                    && self
                        .peek_next()
                        .is_some_and(|x| &x.to_lowercase().to_string() == "x") =>
                {
                    // `start_pos` is global `start_pos` (above)

                    self.advance(); // skip '0'
                    self.advance(); // skip 'x'

                    while let Some(c) = self.current_char() {
                        if c.is_digit(16) || c == '_' {
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    let data = self.input[start_pos..self.pos].to_string();

                    let num_content = Arc::new(&data);

                    let u256_literal = Literal::<U256>::tokenize(
                        &self.input,
                        &num_content,
                        start_pos,
                        self.pos,
                        &mut self.handler,
                    )?;

                    tokens.push(u256_literal);
                }

                _ if c.is_digit(10)
                    || (c == '-' && self.peek_next().is_some_and(|c| c.is_digit(10))) =>
                {
                    let mut is_negative = false;

                    if c == '-' && self.peek_next().is_some_and(|c| c.is_digit(10)) {
                        is_negative = true;
                        self.advance(); // skip '-'
                    }

                    // go back and read from previous char ('-') if neg, else read from current pos
                    let start_pos = if is_negative { self.pos - 1 } else { self.pos };

                    let mut is_float = false;

                    while let Some(c) = self.current_char() {
                        if c.is_digit(10) || c == '_' {
                            self.advance();
                        } else if c == '.' && !is_float && self.peek_next() != Some('.') {
                            self.advance();
                            is_float = true;
                        } else {
                            break;
                        }
                    }

                    let data = self.input[start_pos..self.pos].to_string();

                    let num_content = Arc::new(&data);

                    if is_float {
                        let float_literal = Literal::<FloatType>::tokenize(
                            &self.input,
                            &num_content,
                            start_pos,
                            self.pos,
                            &mut self.handler,
                        )?;

                        tokens.push(float_literal);
                        continue;
                    }

                    if is_negative {
                        let int_literal = Literal::<IntType>::tokenize(
                            &self.input,
                            &num_content,
                            start_pos,
                            self.pos,
                            &mut self.handler,
                        )?;

                        tokens.push(int_literal);
                    } else {
                        let uint_literal = Literal::<UIntType>::tokenize(
                            &self.input,
                            &num_content,
                            start_pos,
                            self.pos,
                            &mut self.handler,
                        )?;

                        tokens.push(uint_literal);
                    }
                }

                ',' | ';' => {
                    self.advance();

                    let data = self.input[start_pos..self.pos].to_string();

                    let punc_content = Arc::new(&data);

                    let punctuation = Punctuation::tokenize(
                        &self.input,
                        &punc_content,
                        start_pos,
                        self.pos,
                        &mut self.handler,
                    )?;

                    tokens.push(punctuation);
                }

                '.' => {
                    self.advance();

                    if let Some('.') = self.current_char() {
                        self.advance();
                    }

                    if let Some('=') = self.current_char() {
                        self.advance();
                    }

                    let data = self.input[start_pos..self.pos].to_string();

                    let punc_content = Arc::new(&data);

                    let punctuation = Punctuation::tokenize(
                        &self.input,
                        &punc_content,
                        start_pos,
                        self.pos,
                        &mut self.handler,
                    )?;

                    tokens.push(punctuation);
                }

                '!'
                | '#'..='&'
                | '*'
                | '+'
                | '/'
                | '-'
                | ':'
                | '<'..='@'
                | '\\'
                | '^'
                | '`'
                | '|' => {
                    while let Some(c) = self.current_char() {
                        if c.is_ascii_punctuation()
                            && !delimiter::is_delimiter(c)
                            && !punctuation::is_quote(c)
                            && !punctuation::is_separator(c)
                        {
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    let data = self.input[start_pos..self.pos].to_string();

                    let punc_content = Arc::new(&data);

                    let punctuation = Punctuation::tokenize(
                        &self.input,
                        &punc_content,
                        start_pos,
                        self.pos,
                        &mut self.handler,
                    )?;

                    tokens.push(punctuation);
                }

                _ if !self.peek_next().is_some() => tokens.push(Some(Token::EOF)),

                _ => return Err(self.log_error(LexErrorKind::InvalidChar(c))),
            }
        }

        if num_closed_delimiters != num_open_delimiters {
            return Err(self.log_error(LexErrorKind::UnclosedDelimiters));
        }

        let stream = TokenStream::new(&self.input, tokens, 0, self.pos);

        Ok(stream)
    }

    // progress through the source code so that the lexer can continue to process chars
    fn advance(&mut self) -> Option<char> {
        // update the lexer's position or other internal state if needed
        self.pos += 1;
        // move to the next char in the iterator
        self.peekable_chars.next()
    }

    // return the current char at the lexer's current position without advancing the pos
    fn current_char(&mut self) -> Option<char> {
        self.peekable_chars.peek().cloned()
    }

    // advance the lexer's pos past any whitespace chars in the input stream
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                // if the current char is whitespace, advance to the next char
                self.advance();
            } else {
                // if the current char is not whitespace, break out of the loop
                break;
            }
        }
    }

    // log and store information about an error encountered during the lexing process
    fn log_error(&self, error_kind: LexErrorKind) -> ErrorEmitted {
        let err = LexError {
            error_kind,
            position: Position::new(self.input.trim_start_matches('\n'), self.pos),
        };

        self.handler.emit_err(CompilerError::Lex(err))
    }

    // return the next char in the input stream (i.e., peek ahead by one char)
    fn peek_next(&self) -> Option<char> {
        // create a clone of the iterator and advance this cloned iterator
        let mut cloned_iter = self.peekable_chars.clone();
        cloned_iter.next();

        // peek at the next char from the original iterator
        cloned_iter.peek().cloned()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[ignore]
    #[test]
    fn lex() {
        let source_code = r#" 
        //! inner doc comment
        
        // line comment
        
        /*
        block comment
        */

        /// outer doc comment
        
        ////////////////////////////////////////////////////////////////////////////////
        // `src/lib.feo`
        ////////////////////////////////////////////////////////////////////////////////
        
        pub mod contract;
        pub mod some_library;

        ////////////////////////////////////////////////////////////////////////////////
        // `src/lib/some_library.feo`
        ////////////////////////////////////////////////////////////////////////////////
        
        library;
        
        pub trait SomeTrait {
            func bar() -> str; 
        }

        pub func hello_world() {
            print!("hello world");
        }

        ////////////////////////////////////////////////////////////////////////////////
        // `src/main.feo`
        ////////////////////////////////////////////////////////////////////////////////
        
        script;

        import crate::some_library;
        import crate::contract::Contract;

        func main() {
            greater_than(1, 2);

            let hello: str = Contract::bar();

            let chars: [char; 5] = ['w', 'o', 'r', 'l', 'd'];

            let world = str!(chars);

            print!("{} {}", hello, world);

            some_library::hello_world();
        }

        func greater_than(arg1: u256, arg2: u256) {
            if arg1 > arg2 {
                print!("{} is greater than {}", arg1, arg2);
            } else if arg1 == arg2 {
                print!("{} is equal to {}", arg1, arg2);
            } else {
                print!("{} is less than {}", arg1, arg2);
            }
        }


        ////////////////////////////////////////////////////////////////////////////////
        // `src/contract.feo`
        ////////////////////////////////////////////////////////////////////////////////
        
        contract;

        import crate::some_library::SomeTrait;

        mod some_abstract_contract;
        
        import self::some_abstract_contract::{SomeAbstractContract, Colour};

        struct Foo {
            field1: String,
            field2: char,
            field3: u256,
            field4: Vec<f64>,
            field5: i64,
            field6: bool
        }

        pub storage {
            pub const ADDRESS: Identity = Identity::Contract(ContractId::from(U256::ZERO));
            const STR: str = "foo";
            const STR_ARRAY: [char; 3] = chars!(STR);
        }

        abi Contract {
            func foo() -> Result<Foo>;
        }

        impl Contract {
            func foo() -> Result<Foo> {
                let array: [u64; 4] = [1, 2, 3, 4];
                let mut vec: Vec<f64> = Vec::new();

                for num in array {
                    vec.push(num as f64);
                }

                vec.push(-5.0);

                return Ok(Foo {
                    field1: String::from("foo"),
                    field2: '\'',
                    field3: 0x0123_4567_89AB_CDEF,
                    field4: vec,
                    field5: -1234,
                    field6: true
                })
            }
        }

        impl SomeAbstractContract for Contract {
            func colour(arg: char) -> Option<Colour> {
                return match arg {
                    'r' => Some(Colour::Red),
                    'g' => Some(Colour::Green),
                    'b' => Some(Colour::Blue),
                    _ => None
                }
            }
        }

        impl SomeTrait for Contract {
            func bar() -> str {
                return "hello"
            }
        }

        ////////////////////////////////////////////////////////////////////////////////
        // `src/contract/some_abstract_contract.feo`
        ////////////////////////////////////////////////////////////////////////////////

        #![abstract]
        contract;

        #[export]
        pub enum Colour {
            Red,
            Green, 
            Blue
        }
        
        pub abi SomeAbstractContract {
            func colour(arg: char) -> Option<Colour>;
        }
        "#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler);

        if let Ok(ts) = lexer.lex() {
            for t in ts.tokens().into_iter() {
                // for token in tokens {
                match t {
                    Token::CharLit(c) => println!("CharLit: {:?}", c.into_inner()),
                    Token::StringLit(s) => println!("StringLit: {:?}", s.into_inner()),
                    Token::BoolLit(b) => println!("BoolLit: {:?}", b.into_inner()),
                    Token::IntLit(i) => println!("IntLit: {:?}", i.into_inner()),
                    Token::UIntLit(ui) => println!("UIntLit: {:?}", ui.into_inner()),
                    Token::U256Lit(u) => println!("U256Lit: {:?}", u.into_inner()),
                    Token::FloatLit(f) => println!("FloatLit: {:?}", f.into_inner()),
                    Token::Iden(id) => println!("Iden: {:?}", id.name),
                    Token::Keyword(k) => println!("Keyword: {:?}", k.keyword_kind),
                    Token::Comment(c) => println!("Comment: {:?}", c.data),
                    Token::DocComment(dc) => println!("DocComment: {:?}", dc.content),
                    Token::Delim(d) => println!("Delim: {:?}", d.delim),
                    Token::Punc(p) => println!("Punc: {:?}", p.punc_kind),
                    Token::TypeAnn(ta) => println!("TypeAnnotation: {:?}", ta.type_ann_kind),
                    Token::EOF => println!("end of file"),
                };
                // }
            }
        } else {
            println!(
                "error: {}, \nposition: line {}, col {}",
                &lexer.errors().pop().expect("Error not found").error_kind(),
                &lexer.errors().pop().expect("Error not found").line_col().0 + 731,
                &lexer.errors().pop().expect("Error not found").line_col().1,
            );
        }
    }
}
