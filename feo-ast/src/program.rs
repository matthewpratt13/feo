use crate::{item::Item, keyword::KeywordKind, type_utils::Semicolon};

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

pub struct Storage {}

impl<C> ContractItem<C> for Storage where C: Item {}

pub struct Abi {}

impl<C> ContractItem<C> for Abi where C: Item {}

pub struct Contract {}

pub struct Library<T> {
    kw_library: KeywordKind,
    semicolon: Semicolon,
    items: Vec<Box<dyn LibraryItem<T>>>,
}

pub struct Script {}
