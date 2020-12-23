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
        NdNeq(lhs,rhs) => {
            gen(&*lhs);
            gen(&*rhs);
            opr="  cmp rax, rdi\n  setne al\n  movzb rax, al";
        },
        NdEq(lhs,rhs) => {
            gen(&*lhs);
            gen(&*rhs);
            opr="  cmp rax, rdi\n  sete al\n  movzb rax, al";
        },
        NdLt(lhs,rhs) => {
            gen(&*lhs);
            gen(&*rhs);
            opr="  cmp rax, rdi\n  setl al\n  movzb rax, al";
        },
        NdLeq(lhs,rhs) => {
            gen(&*lhs);
            gen(&*rhs);
            opr="  cmp rax, rdi\n  setle al\n  movzb rax, al";
        },
        _ => panic!("The token has unexpected type")
    }
 
    println!("  pop rdi");
    println!("  pop rax");
    println!("{}",opr);
    println!("  push rax");
}

