#![allow(dead_code)]

use crate::{
    expression::OuterAttr,
    identifier::Identifier,
    item::{AssociatedItem, FunctionItem, Item, VisibilityKind},
    keyword::KeywordKind,
    type_utils::{Brace, Semicolon},
};

pub trait ContractItem<C>
where
    C: Item,
{
}

impl<C> Item for dyn ContractItem<C> {}

pub trait LibraryItem<L>
where
    L: Item,
{
}

impl<L> Item for dyn LibraryItem<L> {}

pub struct Abi<T> {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_abi: KeywordKind,
    contract_name: Identifier,
    open_brace: Brace,
    functions_opt: Option<Vec<Box<dyn FunctionItem<T>>>>,
    close_brace: Brace,
}

impl<T, C> ContractItem<C> for Abi<T> where C: Item {}

pub struct Contract<T> {
    attributes: Vec<OuterAttr>,
    kw_contract: KeywordKind,
    semicolon: Semicolon,
    contract_items_opt: Option<Vec<Box<dyn ContractItem<T>>>>,
}

pub struct Library<T> {
    attributes: Vec<OuterAttr>,
    kw_library: KeywordKind,
    semicolon: Semicolon,
    items_opt: Vec<Box<dyn LibraryItem<T>>>,
}

pub struct Script {
    attributes: Vec<OuterAttr>,
    kw_script: KeywordKind,
    semicolon: Semicolon,
    items_opt: Option<Vec<Box<dyn Item>>>,
}

pub struct Storage<T> {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_storage: KeywordKind,
    open_brace: Brace,
    items_opt: Option<Vec<Box<dyn AssociatedItem<T>>>>,
    close_brace: Brace,
}

impl<T, C> ContractItem<C> for Storage<T> where C: Item {}
