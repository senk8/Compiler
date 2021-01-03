#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash,Default)]
pub struct Pos(pub usize,pub usize);

#[derive(Debug,Eq,PartialEq,Ord,PartialOrd,Clone,Hash,Default)]
pub struct Annot<T>{
    pub val:T,
    pub pos:Pos,
}