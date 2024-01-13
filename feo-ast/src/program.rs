pub enum Program {
    Contract,
    Library,
    Script,
}

pub enum ContractItem {
    Item,
    Storage,
    Abi,
}

pub struct Storage {}

pub struct Abi {}

pub struct Contract {}

pub struct Library {}

pub struct Script {}
