#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub struct LVar(pub usize, pub TypeInfo);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum TypeInfo {
    Int,
    Pointer(Box<TypeInfo>),
    Array(Box<TypeInfo>, usize),
}
