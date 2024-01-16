use crate::{
    expression::{InnerAttr, OuterAttr},
    identifier::Identifier,
    item::{AssociatedItem, Item},
    keyword::KeywordKind,
    type_utils::{Brace, Semicolon},
};

pub enum ProgramKind {
    Contract,
    Library,
    Script,
}

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

pub struct Storage<T> {
    attributes: Vec<OuterAttr>,
    kw_storage: KeywordKind,
    name: Identifier,
    open_brace: Brace,
    items_opt: Option<Vec<Box<dyn AssociatedItem<T>>>>,
    close_brace: Brace,
}

impl<T, C> ContractItem<C> for Storage<T> where C: Item {}

pub struct Abi {}

impl<C> ContractItem<C> for Abi where C: Item {}

pub struct Contract<T> {
    attributes: Vec<OuterAttr>,
    kw_contract: KeywordKind,
    semicolon: Semicolon,
    contract_items: Vec<Box<dyn ContractItem<T>>>,
}

pub struct Library<T> {
    attributes: Vec<OuterAttr>,
    kw_library: KeywordKind,
    semicolon: Semicolon,
    items: Vec<Box<dyn LibraryItem<T>>>,
}

pub struct Script {
    attributes: Vec<OuterAttr>,
    kw_script: KeywordKind,
    semicolon: Semicolon,
    items: Vec<Box<dyn Item>>,
}
