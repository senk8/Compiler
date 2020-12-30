#[derive(Debug)]
pub enum Node {
    NdAdd(Box<Node>, Box<Node>),
    NdSub(Box<Node>, Box<Node>),
    NdMul(Box<Node>, Box<Node>),
    NdDiv(Box<Node>, Box<Node>),
    NdEq(Box<Node>, Box<Node>),
    NdNeq(Box<Node>, Box<Node>),
    NdLt(Box<Node>, Box<Node>),
    NdLeq(Box<Node>, Box<Node>),
    NdAssign(Box<Node>, Box<Node>),
    NdReturn(Box<Node>),
    NdLVar(usize),
    NdNum(usize),
}
