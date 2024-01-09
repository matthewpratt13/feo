pub enum Program {
    Contract,
    Library,
    Script,
}

pub enum ContractItem {
    Item,
    Storage,
    Interface,
}

pub struct Storage {}

pub struct Interface {}

pub struct Contract {}

pub struct Library {}

pub struct Script {}
