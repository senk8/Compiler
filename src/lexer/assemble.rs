use crate::types::node::Node;
use crate::types::node::Node::*;

pub fn gen_lval(node:&Node)->(){
    if let NdLVar(offset) = *node {
        println!("  mov rax, rbp");
        println!("  sub rax, {}", offset);
        println!("  push rax");
    }else{
        panic!("left hand side in Assign is not variable.");
    }
}

pub fn gen(node:&Node)->() {

    let mut opr = "";

    match node{
        NdAdd(lhs,rhs) => {
            gen(lhs);
            gen(rhs);
            opr = "  add rax, rdi";
        },
        NdSub(lhs,rhs) => {
            gen(lhs);
            gen(rhs);
            opr = "  sub rax, rdi";
        },
        NdMul(lhs,rhs) => {
            gen(lhs);
            gen(rhs);
            opr="  imul rax, rdi";
        },
        NdDiv(lhs,rhs) => {
            gen(lhs);
            gen(rhs);
            opr="  cqo\n  idiv rdi";
        },
        NdNeq(lhs,rhs) => {
            gen(lhs);
            gen(rhs);
            opr="  cmp rax, rdi\n  setne al\n  movzb rax, al";
        },
        NdEq(lhs,rhs) => {
            gen(lhs);
            gen(rhs);
            opr="  cmp rax, rdi\n  sete al\n  movzb rax, al";
        },
        NdLt(lhs,rhs) => {
            gen(lhs);
            gen(rhs);
            opr="  cmp rax, rdi\n  setl al\n  movzb rax, al";
        },
        NdLeq(lhs,rhs) => {
            gen(lhs);
            gen(rhs);
            opr="  cmp rax, rdi\n  setle al\n  movzb rax, al";
        },
        NdNum(n) =>{
            println!("  push {}", n);
            return
        },
        NdLVar(_) =>{
            gen_lval(node);

            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            return;
        },
        NdAssign(lhs,rhs) =>{
            gen_lval(lhs);
            gen(rhs);
        
            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
            return;
        },
        NdReturn(lhs) => {
            gen(lhs);
            println!("  pop rax");
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
            return;
        }
    }
 
    println!("  pop rdi");
    println!("  pop rax");
    println!("{}",opr);
    println!("  push rax");
}

