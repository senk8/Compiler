use crate::types::node::Node;
use crate::types::node::Node::*;

pub fn gen(node: &Node) -> () {
    match node {
        NdNum(n) => {
            println!("  push {}", n);
        }
        NdAdd(lhs, rhs) => {
            gen(lhs);
            gen(rhs);
            print_opration_epilogue("  add rax, rdi");
        }
        NdSub(lhs, rhs) => {
            gen(lhs);
            gen(rhs);
            print_opration_epilogue("  sub rax, rdi");
        }
        NdMul(lhs, rhs) => {
            gen(lhs);
            gen(rhs);
            print_opration_epilogue("  imul rax, rdi");
        }
        NdDiv(lhs, rhs) => {
            gen(lhs);
            gen(rhs);
            print_opration_epilogue("  cqo\n  idiv rdi");
        }
        NdNeq(lhs, rhs) => {
            gen(lhs);
            gen(rhs);
            print_opration_epilogue("  cmp rax, rdi\n  setne al\n  movzb rax, al");
        }
        NdEq(lhs, rhs) => {
            gen(lhs);
            gen(rhs);
            print_opration_epilogue("  cmp rax, rdi\n  sete al\n  movzb rax, al");
        }
        NdLt(lhs, rhs) => {
            gen(lhs);
            gen(rhs);
            print_opration_epilogue("  cmp rax, rdi\n  setl al\n  movzb rax, al");
        }
        NdLeq(lhs, rhs) => {
            gen(lhs);
            gen(rhs);
            print_opration_epilogue("  cmp rax, rdi\n  setle al\n  movzb rax, al");
        }
        NdLVar(_) => {
            gen_lval(node);

            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
        }
        NdAssign(lhs, rhs) => {
            gen_lval(lhs);
            gen(rhs);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
        }
        NdReturn(lhs) => {
            gen(lhs);
            println!("  pop rax");
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
        }
    };
    return;
}

fn gen_lval(node: &Node) -> () {
    if let NdLVar(offset) = *node {
        println!("  mov rax, rbp");
        println!("  sub rax, {}", offset);
        println!("  push rax");
    } else {
        panic!("left hand side in Assign is not variable.");
    }
}

fn print_opration_epilogue(message: &str) -> () {
    println!("  pop rdi");
    println!("  pop rax");
    println!("{}", message);
    println!("  push rax");
}
