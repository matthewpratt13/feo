use crate::{item::Item, keyword::KeywordKind, type_utils::Semicolon};

pub enum ProgramKind {
    Contract,
    Library,
    Script,
}

pub enum ContractItemKind {
    Item,
    Storage,
    Abi,
}

pub trait LibraryItem<L>
where
    L: Item,
{
}

impl<L> Item for dyn LibraryItem<L> {}

pub struct Storage {}

pub struct Abi {}

pub struct Contract {}

pub struct Library<T> {
    kw_library: KeywordKind,
    semicolon: Semicolon,
    items: Vec<Box<dyn LibraryItem<T>>>,
}

pub struct Script {}
