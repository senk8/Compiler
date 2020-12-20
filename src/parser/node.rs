#[derive(Debug)]
pub enum Node{
    NdAdd(Box<Node>,Box<Node>),
    NdSub(Box<Node>,Box<Node>),
    NdMul(Box<Node>,Box<Node>),
    NdDiv(Box<Node>,Box<Node>),
    NdNum(usize) 
}
