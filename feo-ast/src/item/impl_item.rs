pub enum ImplItem {
    Inherent(InherentImpl),
    Trait(TraitImpl),
}

pub struct InherentImpl {}

pub struct TraitImpl {}
