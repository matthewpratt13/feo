#![allow(dead_code)]

use crate::{
    expression::OuterAttr,
    identifier::Identifier,
    item::{AssociatedItem, FunctionItem, Item, VisibilityKind},
    keyword::KeywordKind,
    type_utils::{Brace, Semicolon},
};

pub trait ContractItem {}

pub trait LibraryItem
where
    Self: Item,
{
}

pub struct Abi {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_abi: KeywordKind,
    contract_name: Identifier,
    open_brace: Brace,
    functions_opt: Option<Vec<Box<dyn FunctionItem>>>,
    close_brace: Brace,
}

impl ContractItem for Abi {}

pub struct Contract {
    attributes: Vec<OuterAttr>,
    kw_contract: KeywordKind,
    semicolon: Semicolon,
    contract_items_opt: Option<Vec<Box<dyn ContractItem>>>,
}

pub struct Library {
    attributes: Vec<OuterAttr>,
    kw_library: KeywordKind,
    semicolon: Semicolon,
    items_opt: Vec<Box<dyn LibraryItem>>,
}

pub struct Script {
    attributes: Vec<OuterAttr>,
    kw_script: KeywordKind,
    semicolon: Semicolon,
    items_opt: Option<Vec<Box<dyn Item>>>,
}

pub struct Storage {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_storage: KeywordKind,
    open_brace: Brace,
    items_opt: Option<Vec<Box<dyn AssociatedItem>>>,
    close_brace: Brace,
}

impl ContractItem for Storage {}
