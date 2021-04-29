use super::token::TypeKind;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub struct LVar(pub usize, pub VarAnnot);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone,Hash)]
pub struct VarAnnot {
    pub ty: TypeKind,
    pub ptr: Option<Box<VarAnnot>>,
}
