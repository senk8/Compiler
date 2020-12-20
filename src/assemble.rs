use super::parser::node::Node;
use super::parser::node::Node::*;

pub fn gen(node:&Node)->() {
    if let NdNum(n) = node {
      println!("  push {}", n);
      return;
    }

    let mut opr = "";
    match node{
        NdAdd(lhs,rhs) => {
            gen(&*lhs);
            gen(&*rhs);
            opr = "  add rax, rdi";
        },
        NdSub(lhs,rhs) => {
            gen(&*lhs);
            gen(&*rhs);
            opr = "  sub rax, rdi";
        },
        NdMul(lhs,rhs) => {
            gen(&*lhs);
            gen(&*rhs);
            opr="  imul rax, rdi";
        },
        NdDiv(lhs,rhs) => {
            gen(&*lhs);
            gen(&*rhs);
            opr="  cqo\n  idiv rdi";
        },
        _ => panic!("unexpected token")
    }
 
    println!("  pop rdi");
    println!("  pop rax");
    println!("{}",opr);
    println!("  push rax");
}