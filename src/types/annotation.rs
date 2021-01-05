use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default)]
pub struct Pos(pub usize, pub usize);

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line:1 Column:{}~{}", self.0 + 1, self.1 + 1)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default)]
pub struct Annot<T> {
    pub val: T,
    pub pos: Pos,
}
