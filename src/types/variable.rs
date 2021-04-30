use super::token::TypeKind;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub struct LVar(pub usize, pub TypeInfo);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone,Hash)]
pub struct TypeInfo {
    pub type_: TypeKind,
    pub ptr: Option<Box<TypeInfo>>,
}
