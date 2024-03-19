use feo_ast::expression::{TermCollection, Value, ValueCollection};
use feo_error::error::CompilerError;
use feo_types::{punctuation::PuncKind, Punctuation};

use crate::{
    parse::{ParseItem, ParseTerm},
    parser::Parser,
};

pub fn get_attributes<T: ParseTerm>(
    parser: &mut Parser,
) -> Result<Option<Vec<T>>, Vec<CompilerError>> {
    let mut attributes: Vec<T> = Vec::new();

    while let Some(a) = T::parse(parser)? {
        attributes.push(a);
        parser.next_token();
    }

    if attributes.is_empty() {
        Ok(None)
    } else {
        Ok(Some(attributes))
    }
}

pub fn get_items<T: ParseItem>(parser: &mut Parser) -> Result<Option<Vec<T>>, Vec<CompilerError>> {
    let mut items: Vec<T> = Vec::new();

    while let Some(i) = T::parse(parser)? {
        items.push(i);
        parser.next_token();
    }

    if items.is_empty() {
        Ok(None)
    } else {
        Ok(Some(items))
    }
}

pub fn get_term_collection<T: ParseTerm>(
    parser: &mut Parser,
) -> Result<Option<TermCollection<T>>, Vec<CompilerError>> {
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

        skip_trailing_comma(parser)?;

        let subsequent_terms_opt = if terms.is_empty() { None } else { Some(terms) };

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

        skip_trailing_comma(parser)?;

        let subsequent_values_opt = if values.is_empty() {
            None
        } else {
            Some(values)
        };

        return Ok(Some(ValueCollection::new(
            first_value,
            subsequent_values_opt,
        )));
    } else {
        return Ok(None);
    }
}

pub fn skip_trailing_comma(parser: &mut Parser) -> Result<(), Vec<CompilerError>> {
    if let Some(Punctuation {
        punc_kind: PuncKind::Comma,
        ..
    }) = parser.peek_current::<Punctuation>()
    {
        parser.next_token();
        Ok(())
    } else {
        Ok(())
    }
}
