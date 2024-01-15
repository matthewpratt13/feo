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

pub struct Storage {}

pub struct Abi {}

pub struct Contract {}

pub struct Library {}

pub struct Script {}
