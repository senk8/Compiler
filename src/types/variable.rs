use super::token::TypeKind;

#[derive(Debug, PartialEq, Clone)]
pub struct LVar(pub usize, pub VarAnnot);

#[derive(Debug, PartialEq, Clone)]
pub struct VarAnnot {
    pub ty: TypeKind,
    pub ptr: Option<Box<VarAnnot>>,
}
