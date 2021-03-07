use super::token::TypeKind;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default)]
pub struct Pos(pub usize, pub usize);

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line:{} Column:{}", self.0 + 1, self.1 + 1)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default)]
pub struct Annot<T> {
    pub val: T,
    pub pos: Pos,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LVar(pub usize, pub VarAnnot);

#[derive(Debug, PartialEq, Clone)]
pub struct VarAnnot {
    pub ty: TypeKind,
    pub ptr: Option<Box<VarAnnot>>,
}
