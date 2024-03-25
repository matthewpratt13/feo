use feo_ast::{
    expression::{TermCollection, Value, ValueCollection},
    item::{PathCollection, VisibilityKind},
    statement::Statement,
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
    Delimiter, Punctuation,
};

use crate::{
    parse::{ParseItem, ParseStatement, ParseTerm},
    parser::Parser,
};

pub fn get_attributes<T: ParseTerm>(
    parser: &mut Parser,
) -> Result<Option<Vec<T>>, Vec<CompilerError>> {
    // log_msg(LogMsgType::Enter, "`get_attributes()`", parser);

    let mut attributes: Vec<T> = Vec::new();

    while let Some(a) = T::parse(parser)? {
        attributes.push(a);
        parser.next_token();
    }

    println!("number of attributes: {}", attributes.len());

    // log_msg(LogMsgType::Exit, "`get_attributes()`", parser);

    if attributes.is_empty() {
        Ok(None)
    } else {
        Ok(Some(attributes))
    }
}

pub fn get_items<T: ParseItem>(parser: &mut Parser) -> Result<Option<Vec<T>>, Vec<CompilerError>> {
    log_msg(LogMsgType::Enter, "`get_items()`", parser);

    let mut items: Vec<T> = Vec::new();

    while let Some(i) = T::parse(parser)? {
        items.push(i);
        parser.next_token();
    }

    println!("number of items: {}", items.len());

    log_msg(LogMsgType::Exit, "`get_items()`", parser);

    if items.is_empty() {
        Ok(None)
    } else {
        Ok(Some(items))
    }
}

pub fn get_path_collection<T: ParseTerm>(
    parser: &mut Parser,
) -> Result<Option<PathCollection<T>>, Vec<CompilerError>> {
    log_msg(LogMsgType::Enter, "`get_path_collection()`", parser);

    if let Some(root_path) = T::parse(parser)? {
        if let Some(Punctuation {
            punc_kind: PuncKind::DblColon,
            ..
        }) = parser.peek_current()
        {
            parser.next_token();

            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = parser.peek_current()
            {
                parser.next_token();

                let path_suffixes = if let Some(inner_paths) = get_term_collection::<T>(parser)? {
                    Some(Box::new(inner_paths))
                } else {
                    None
                };

                if let Some(Punctuation {
                    punc_kind: PuncKind::ColonColonAsterisk,
                    ..
                }) = parser.peek_current()
                {
                    parser.next_token();
                    skip_trailing_comma(parser)?;
                }

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Close),
                    ..
                }) = parser.peek_current()
                {
                    log_msg(LogMsgType::Exit, "`get_path_collection()`", parser);

                    return Ok(Some(PathCollection {
                        root_path: Box::new(root_path),
                        path_suffixes,
                    }));
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`{`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            log_msg(LogMsgType::Exit, "`get_path_collection()`", parser);

            return Ok(Some(PathCollection {
                root_path: Box::new(root_path),
                path_suffixes: None,
            }));
        }
    } else {
        log_msg(LogMsgType::Exit, "`get_path_collection()`", parser);

        return Ok(None);
    }

    Err(parser.errors())
}

pub fn get_statements(parser: &mut Parser) -> Result<Option<Vec<Statement>>, Vec<CompilerError>> {
    log_msg(LogMsgType::Enter, "`get_statements()`", parser);

    let mut statements: Vec<Statement> = Vec::new();

    while let Some(s) = Statement::parse(parser)? {
        statements.push(s);

        if let Some(Delimiter {
            delim: (DelimKind::Brace, DelimOrientation::Close),
            ..
        }) = parser.peek_current()
        {
            break;
        }
    }

    println!("number of statements: {}", statements.len());

    log_msg(LogMsgType::Exit, "`get_statements()`", parser);

    if statements.is_empty() {
        return Ok(None);
    } else {
        return Ok(Some(statements));
    }
}

pub fn get_term_collection<T: ParseTerm>(
    parser: &mut Parser,
) -> Result<Option<TermCollection<T>>, Vec<CompilerError>> {
    // log_msg(LogMsgType::Enter, "`get_term_collection()`", parser);

    let mut terms: Vec<T> = Vec::new();

    if let Some(first_term) = T::parse(parser)? {
        parser.next_token();

        while let Some(Punctuation {
            punc_kind: PuncKind::Comma,
            ..
        }) = parser.peek_current()
        {
            parser.next_token();

            if let Some(next_term) = T::parse(parser)? {
                terms.push(next_term);
                parser.next_token();
            } else {
                break;
            }
        }

        println!("number of terms: {}", terms.len() + 1);

        let subsequent_terms_opt = if terms.is_empty() { None } else { Some(terms) };

        skip_trailing_comma(parser)?;

        // log_msg(LogMsgType::Exit, "`get_term_collection()`", parser);

        return Ok(Some(TermCollection::new(first_term, subsequent_terms_opt)));
    } else {
        return Ok(None);
    }
}

pub fn get_value_collection(
    parser: &mut Parser,
) -> Result<Option<ValueCollection>, Vec<CompilerError>> {
    let mut values: Vec<Value> = Vec::new();

    if let Some(first_value) = Value::parse(parser)? {
        parser.next_token();

        while let Some(Punctuation {
            punc_kind: PuncKind::Comma,
            ..
        }) = parser.peek_current::<Punctuation>()
        {
            parser.next_token();

            if let Some(next_value) = Value::parse(parser)? {
                values.push(next_value);
                parser.next_token();
            } else {
                break;
            }
        }

        println!("number of values: {}", values.len() + 1);

        log_msg(LogMsgType::Exit, "`get_value_collection()`", parser);

        let subsequent_values_opt = if values.is_empty() {
            None
        } else {
            Some(values)
        };

        return Ok(Some(ValueCollection {
            first_value: Box::new(first_value),
            subsequent_values_opt,
        }));
    } else {
        return Ok(None);
    }
}

pub fn get_visibility(parser: &mut Parser) -> Result<Option<VisibilityKind>, Vec<CompilerError>> {
    // log_msg(LogMsgType::Enter, "`get_visibility()`", parser);

    if let Some(v) = VisibilityKind::parse(parser)? {
        parser.next_token();

        println!("visibility kind: {:#?}", v);

        // log_msg(LogMsgType::Exit, "`get_visibility()`", parser);
        Ok(Some(v))
    } else {
        // log_msg(LogMsgType::Exit, "`get_visibility()`", parser);
        Ok(None)
    }
}

pub enum LogMsgType {
    Enter,
    Exit,
    Detect,
    Expect,
}

pub fn log_msg(msg_type: LogMsgType, object_name: &str, parser: &mut Parser) -> () {
    let msg_str = match msg_type {
        LogMsgType::Enter => "entering",
        LogMsgType::Exit => "exit",
        LogMsgType::Detect => "detected",
        LogMsgType::Expect => "expected",
    };

    println!(
        "{msg_str} {object_name}...\ncurrent_token: {:#?}",
        parser.current_token()
    );
}

pub fn skip_trailing_comma(parser: &mut Parser) -> Result<(), Vec<CompilerError>> {
    if let Some(Punctuation {
        punc_kind: PuncKind::Comma,
        ..
    }) = parser.peek_current::<Punctuation>()
    {
        log_msg(LogMsgType::Detect, "trailing comma", parser);

        parser.next_token();
        Ok(())
    } else {
        Ok(())
    }
}
