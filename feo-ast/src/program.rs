#![allow(dead_code)]

use crate::{
    expression::InnerAttr,
    identifier::Identifier,
    item::{AssociatedItem, FunctionDef, Item, VisibilityKind},
    keyword::Keyword,
    type_utils::{Brace, Semicolon},
};

pub trait ContractItem {}

pub trait LibraryItem
where
    Self: Item,
{
}

pub struct Abi {
    visibility_opt: Option<VisibilityKind>,
    kw_abi: Keyword,
    contract_name: Identifier,
    open_brace: Brace,
    functions_opt: Option<Vec<Box<dyn FunctionDef>>>, // function signatures only
    close_brace: Brace,
}

impl ContractItem for Abi {}

pub struct Contract {
    attributes: Vec<InnerAttr>, // module-wide attributes
    kw_contract: Keyword,
    semicolon: Semicolon,
    contract_items_opt: Option<Vec<Box<dyn ContractItem>>>,
}

pub struct Library {
    attributes: Vec<InnerAttr>, // module-wide attributes
    kw_library: Keyword,
    semicolon: Semicolon,
    items_opt: Vec<Box<dyn LibraryItem>>,
}

pub struct Script {
    attributes: Vec<InnerAttr>, // module-wide attributes
    kw_script: Keyword,
    semicolon: Semicolon,
    items_opt: Option<Vec<Box<dyn Item>>>,
}

pub struct Storage {
    visibility_opt: Option<VisibilityKind>,
    kw_storage: Keyword,
    open_brace: Brace,
    items_opt: Option<Vec<Box<dyn AssociatedItem>>>,
    close_brace: Brace,
}

impl ContractItem for Storage {}
