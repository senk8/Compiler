use crate::types::node::Node;
use crate::types::node::Node::*;

const ARG_REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

pub fn gen_inst_x86_64(asts: Vec<Node>) -> () {
    /* start assemble prologue*/

    println!(".intel_syntax noprefix");
    println!(".globl main");
    
    /*
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");
    */

    for ast in asts.iter() {
        gen(&ast, &mut 0);
    }

    /* start assemble epilogue*/

    /*
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
    */
}

fn gen(node: &Node, n: &mut usize) -> () {
    match node {
        NdNum(n) => {
            println!("  push {}", n);
        }
        NdAdd(lhs, rhs) => {
            gen(lhs, n);
            gen(rhs, n);
            print_opration_epilogue("  add rax, rdi");
        }
        NdSub(lhs, rhs) => {
            gen(lhs, n);
            gen(rhs, n);
            print_opration_epilogue("  sub rax, rdi");
        }
        NdMul(lhs, rhs) => {
            gen(lhs, n);
            gen(rhs, n);
            print_opration_epilogue("  imul rax, rdi");
        }
        NdDiv(lhs, rhs) => {
            gen(lhs, n);
            gen(rhs, n);
            print_opration_epilogue("  cqo\n  idiv rdi");
        }
        NdNeq(lhs, rhs) => {
            gen(lhs, n);
            gen(rhs, n);
            print_opration_epilogue("  cmp rax, rdi\n  setne al\n  movzb rax, al");
        }
        NdEq(lhs, rhs) => {
            gen(lhs, n);
            gen(rhs, n);
            print_opration_epilogue("  cmp rax, rdi\n  sete al\n  movzb rax, al");
        }
        NdLt(lhs, rhs) => {
            gen(lhs, n);
            gen(rhs, n);
            print_opration_epilogue("  cmp rax, rdi\n  setl al\n  movzb rax, al");
        }
        NdLeq(lhs, rhs) => {
            gen(lhs, n);
            gen(rhs, n);
            print_opration_epilogue("  cmp rax, rdi\n  setle al\n  movzb rax, al");
        }
        NdLVar(_) => {
            gen_lval(node);

            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
        }
        NdDecl(name,args,body) => {
            println!("{}:",name);

            println!("  push rbp");
            println!("  mov rbp, rsp");
            println!("  sub rsp, 208");

            /* 引数名のローカル変数を確保する */
            for i in 0..args.len() {
                gen_lval(&args[i]); /* オフセットを渡す.*/
                println!("  pop rax");
                println!("  mov [rax], {}",ARG_REGS[i]);
            }
            gen(body,n);//NdBlock
        }
        NdCall(name, args) => {
            for i in 0..args.len() {
                gen(&args[i], n);
                println!("  pop {}", ARG_REGS[i]);
            }

            /*
            let label = *n;
            *n += 1;
            println!("  mov rdx, 0");
            println!("  mov rax, 16");
            println!("  mov rbx, rsp");
            println!("  div rbx");
            println!("  cmp rdx, 0");
            println!("  je  .Lend{}", label);
            println!("  sub rsp, 8");
            println!(".Lend{}:", label);
            */

            println!("  call {}", name);
            println!("  push rax");
        }
        NdAssign(lhs, rhs) => {
            gen_lval(lhs);
            gen(rhs, n);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
        }
        NdReturn(lhs) => {
            gen(lhs, n);
            println!("  pop rax");
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
        }
        NdIf(lhs, rhs) => {
            let label = *n;
            *n += 1;
            gen(lhs, n);
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je  .Lend{}", label);
            gen(rhs, n);
            println!(".Lend{}:", label);
        }
        NdIfElse(first, second, third) => {
            let label = *n;
            *n += 1;
            gen(first, n);
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je  .Lelse{}", label);
            gen(second, n);
            println!("  jmp  .Lend{}", label);
            println!(".Lelse{}:", label);
            gen(third, n);
            println!(".Lend{}:", label);
        }
        NdWhile(lhs, rhs) => {
            let label = *n;
            *n += 1;
            println!(".Lbegin{}:", label);
            gen(lhs, n);
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je  .Lend{}", label);
            gen(rhs, n);
            println!("  jmp .Lbegin{}", label);
            println!(".Lend{}:", label);
        }
        NdFor(first, second, third, fourth) => {
            let label = *n;
            *n += 1;
            gen(first, n);
            println!(".Lbegin{}:", label);
            gen(second, n);
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je  .Lend{}", label);
            gen(third, n);
            gen(fourth, n);
            println!("  jmp .Lbegin{}", label);
            println!(".Lend{}:", label);
        }
        NdBlock(nodes) => {
            let len = nodes.len();
            for i in 0..len {
                gen(&nodes[i], n);
                if i < len - 1 {
                    println!("  pop rax")
                };
            }
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
