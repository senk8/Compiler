use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::token::TypeKind;
use crate::types::variable::TypeInfo;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::Result;

const ARG_REGS: [&str; 6] = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

pub fn gen_inst_x86_64(asts: Vec<Node>, path_name: &str) -> Result<()> {
    let path = Path::new(path_name);

    let file = File::create(path)?;
    let mut stream = BufWriter::new(file);

    /* start assemble prologue*/

    writeln!(&mut stream, ".intel_syntax noprefix")?;
    writeln!(&mut stream, ".globl main")?;

    let mut n = 0;
    for ast in asts.iter() {
        gen(&mut stream, &ast, &mut n)?;
    }

    stream.flush()?;

    Ok(())
}

fn gen(stream: &mut BufWriter<File>, node: &Node, n: &mut usize) -> Result<()> {
    match node {
        NdNum(num) => {
            writeln!(stream, "  push {}", num)?;
        }
        NdAdd(lhs, rhs) => {

            gen(stream, lhs, n)?;
            gen(stream, rhs, n)?;

            writeln!(stream, "  pop rdi")?;
            writeln!(stream, "  pop rax")?;

            if let NdLVar(_,ref type_info)= **lhs {
                if type_info.type_ == TypeKind::Pointer {
                    let bytes = bytes_per_type(type_info);
                    writeln!(stream,"  imul rdi, {}",bytes)?;
                }
            }

            writeln!(stream, "  add rax, rdi")?;
            writeln!(stream, "  push rax")?
        }
        NdSub(lhs, rhs) => {
            gen(stream, lhs, n)?;
            gen(stream, rhs, n)?;
            print_opration_epilogue(stream, "  sub rax, rdi")?;
        }
        NdMul(lhs, rhs) => {
            gen(stream, lhs, n)?;
            gen(stream, rhs, n)?;
            print_opration_epilogue(stream, "  imul rax, rdi")?;
        }
        NdDiv(lhs, rhs) => {
            gen(stream, lhs, n)?;
            gen(stream, rhs, n)?;
            print_opration_epilogue(stream, "  cqo\n  idiv rdi")?;
        }
        NdRef(operand) => {
            get_lvar_addr(stream, operand)?;
        }
        NdDeref(operand) => {
            gen(stream, operand, n)?;
            writeln!(stream, "  pop rax")?;
            writeln!(stream, "  mov rax, [rax]")?;
            writeln!(stream, "  push rax")?;
        }
        NdNeq(lhs, rhs) => {
            gen(stream, lhs, n)?;
            gen(stream, rhs, n)?;
            print_opration_epilogue(stream, "  cmp rax, rdi\n  setne al\n  movzb rax, al")?;
        }
        NdEq(lhs, rhs) => {
            gen(stream, lhs, n)?;
            gen(stream, rhs, n)?;
            print_opration_epilogue(stream, "  cmp rax, rdi\n  sete al\n  movzb rax, al")?;
        }
        NdLt(lhs, rhs) => {
            gen(stream, lhs, n)?;
            gen(stream, rhs, n)?;
            print_opration_epilogue(stream, "  cmp rax, rdi\n  setl al\n  movzb rax, al")?;
        }
        NdLeq(lhs, rhs) => {
            gen(stream, lhs, n)?;
            gen(stream, rhs, n)?;
            print_opration_epilogue(stream, "  cmp rax, rdi\n  setle al\n  movzb rax, al")?;
        }
        NdLVar(_,_) => {
            get_lvar_addr(stream, node)?;

            writeln!(stream, "  pop rax")?;
            writeln!(stream, "  mov rax, [rax]")?;
            writeln!(stream, "  push rax")?;
        }
        NdDecl(name, args, body) => {
            writeln!(stream, "{}:", name)?;

            writeln!(stream, "  push rbp")?;
            writeln!(stream, "  mov rbp, rsp")?;
            writeln!(stream, "  sub rsp, 208")?;

            /* 引数名のローカル変数を確保する */
            for i in 0..args.len() {
                get_lvar_addr(stream, &args[i])?; /* オフセットを渡す.*/
                writeln!(stream, "  pop rax")?;
                writeln!(stream, "  mov [rax], {}", ARG_REGS[i])?;
            }
            gen(stream, body, n)?; //NdBlock
        }
        NdVdecl(_) => {
            ();
        }
        NdCall(name, args) => {
            for i in 0..args.len() {
                gen(stream, &args[i], n)?;
                writeln!(stream, "  pop {}", ARG_REGS[i])?;
            }

            /*
            今の設計だと、raxの値が変更されてしまい関数呼び出しがおかしくなる。
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

            writeln!(stream, "  call {}", name)?;
            writeln!(stream, "  push rax")?;
        }
        NdAssign(lhs, rhs) => {

            if let NdDeref(ref node)= **lhs {
                gen(stream, &node, n)?;
            }else{
                get_lvar_addr(stream, lhs)?;
            }

            gen(stream, rhs, n)?;

            writeln!(stream, "  pop rdi")?;
            writeln!(stream, "  pop rax")?;
            writeln!(stream, "  mov [rax], rdi")?;
            writeln!(stream, "  push rdi")?;
        }
        /*
        NdSizeof(operand) => {

            gen(stream, operand, n)?;

            let bytes = match operand {
                NdNum(_) => 4,
                NdLVar(_,annot) => bytes_per_type(annot),
            }

            writeln!(stream, "  push {}", bytes)?;
        }
        */
        NdReturn(lhs) => {
            gen(stream, lhs, n)?;

            writeln!(stream, "  pop rax")?;
            writeln!(stream, "  mov rsp, rbp")?;
            writeln!(stream, "  pop rbp")?;
            writeln!(stream, "  ret")?;
        }
        NdIf(lhs, rhs) => {
            let label = *n;
            *n += 1;
            gen(stream, lhs, n)?;
            writeln!(stream, "  pop rax")?;
            writeln!(stream, "  cmp rax, 0")?;
            writeln!(stream, "  je  .Lend{}", label)?;
            gen(stream, rhs, n)?;
            writeln!(stream, ".Lend{}:", label)?;
        }
        NdIfElse(first, second, third) => {
            let label = *n;
            *n += 1;
            gen(stream, first, n)?;
            writeln!(stream, "  pop rax")?;
            writeln!(stream, "  cmp rax, 0")?;
            writeln!(stream, "  je  .Lelse{}", label)?;
            gen(stream, second, n)?;
            writeln!(stream, "  jmp  .Lend{}", label)?;
            writeln!(stream, ".Lelse{}:", label)?;
            gen(stream, third, n)?;
            writeln!(stream, ".Lend{}:", label)?;
        }
        NdWhile(lhs, rhs) => {
            let label = *n;
            *n += 1;
            writeln!(stream, ".Lbegin{}:", label)?;
            gen(stream, lhs, n)?;
            writeln!(stream, "  pop rax")?;
            writeln!(stream, "  cmp rax, 0")?;
            writeln!(stream, "  je  .Lend{}", label)?;
            gen(stream, rhs, n)?;
            writeln!(stream, "  jmp .Lbegin{}", label)?;
            writeln!(stream, ".Lend{}:", label)?;
        }
        NdFor(first, second, third, fourth) => {
            let label = *n;
            *n += 1;
            gen(stream, first, n)?;
            writeln!(stream, ".Lbegin{}:", label)?;
            gen(stream, second, n)?;
            writeln!(stream, "  pop rax")?;
            writeln!(stream, "  cmp rax, 0")?;
            writeln!(stream, "  je  .Lend{}", label)?;
            gen(stream, third, n)?;
            gen(stream, fourth, n)?;
            writeln!(stream, "  jmp .Lbegin{}", label)?;
            writeln!(stream, ".Lend{}:", label)?;
        }
        NdBlock(nodes) => {
            let len = nodes.len();
            for i in 0..len {
                gen(stream, &nodes[i], n)?;

                if let NdReturn(_) = &nodes[i] {
                    continue;
                } else if let NdBlock(_) = &nodes[i] {
                    continue;
                }

                /*ブロック自体は値を残さないので, pop raxしないこと */
                writeln!(stream, "  pop rax")?;
            }
        }
    };
    return Ok(());
}

fn get_lvar_addr(stream: &mut BufWriter<File>, node: &Node) -> Result<()> {
    if let NdLVar(offset,_) = *node {
        writeln!(stream, "  mov rax, rbp")?;
        writeln!(stream, "  sub rax, {}", offset)?;
        writeln!(stream, "  push rax")?;
    } else {
        panic!("left hand side in Assign is not variable.");
    }
    Ok(())
}

fn print_opration_epilogue(stream: &mut BufWriter<File>, message: &str) -> Result<()> {
    writeln!(stream, "  pop rdi")?;
    writeln!(stream, "  pop rax")?;
    writeln!(stream, "{}", message)?;
    writeln!(stream, "  push rax")?;
    Ok(())
}



fn bytes_per_type(type_:&TypeInfo)->usize{
    match type_.ptr.as_ref().unwrap().type_ {
        TypeKind::Int => 4,
        TypeKind::Pointer => 8,
    }
}
