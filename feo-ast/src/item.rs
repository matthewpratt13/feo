#![allow(dead_code)]

mod constant_item;
mod enum_item;
mod extern_crate_item;
mod function_item;
mod impl_item;
mod import_decl_item;
mod module_item;
mod struct_item;

use self::{
    extern_crate_item::ExternCrateItem, impl_item::ImplItemKind, import_decl_item::ImportDeclItem,
    module_item::ModuleItemKind, trait_item::TraitItem,
};

pub use self::constant_item::{ConstantItem, StaticItem};
pub use self::enum_item::EnumItem;
pub use self::function_item::FunctionItem;
pub use self::impl_item::AssociatedItem;
pub use self::struct_item::StructItemKind;
pub use self::type_alias_item::TypeAliasItem;
pub use self::visibility::VisibilityKind;

pub enum ItemKind {
    Visibility(VisibilityKind),
    Constant(ConstantItem),
    Static(StaticItem),
    Enum(EnumItem),
    ExternCrate(ExternCrateItem),
    Function(FunctionItem),
    Impl(ImplItemKind),
    ImportDecl(ImportDeclItem),
    Module(ModuleItemKind),
    Struct(StructItemKind),
    Trait(TraitItem),
    TypeAlias(TypeAliasItem),
}

mod trait_item {
    use crate::{identifier::Identifier, keyword::KeywordKind, type_utils::Brace};

    use super::AssociatedItem;

    pub struct TraitItem<T> {
        kw_unsafe_opt: Option<KeywordKind>,
        kw_impl: KeywordKind,
        name: Identifier,
        open_brace: Brace,
        associated_items: Vec<AssociatedItem<T>>,
        close_brace: Brace,
    }
}

mod type_alias_item {
    use crate::{
        identifier::Identifier,
        keyword::KeywordKind,
        ty::Type,
        type_utils::{Equals, Semicolon},
    };

    pub struct TypeAliasItem {
        kw_type: KeywordKind,
        name: Identifier,
        value_opt: Option<(Equals, Box<Type>)>,
        semicolon: Semicolon,
    }
}

mod visibility {
    use crate::{keyword::KeywordKind, type_utils::Parenthesis};

    pub enum VisibilityKind {
        Pub(KeywordKind),
        PubCrate(PubCrateVisibility),
    }

    pub struct PubCrateVisibility {
        kw_pub: KeywordKind,
        open_parenthesis: Parenthesis,
        kw_crate: KeywordKind,
        close_parenthesis: Parenthesis,
    }
}
