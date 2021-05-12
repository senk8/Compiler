use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::parse::TypeInfo;

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
        NdAdd(lhs, rhs)
        | NdSub(lhs, rhs)
        | NdMul(lhs, rhs)
        | NdDiv(lhs, rhs)
        | NdEq(lhs, rhs)
        | NdNeq(lhs, rhs)
        | NdLt(lhs, rhs)
        | NdLeq(lhs, rhs) => {
            gen(stream, lhs, n)?;
            gen(stream, rhs, n)?;

            writeln!(stream, "  pop rdi")?;
            writeln!(stream, "  pop rax")?;

            /* a type */
            match node {
                NdAdd(_, _) | NdSub(_, _) => {
                    if let type_info @ TypeInfo::Pointer(_) = analyze_subtree(lhs) {
                        let bytes = size_pointer_to(&type_info);
                        writeln!(stream, "  imul rdi, {}", bytes)?;
                    };

                    match node {
                        NdAdd(_, _) => writeln!(stream, "  add rax, rdi")?,
                        NdSub(_, _) => writeln!(stream, "  sub rax, rdi")?,
                        _ => unreachable!(),
                    }
                }
                NdMul(_, _) => writeln!(stream, "  imul rax, rdi")?,
                NdDiv(_, _) => writeln!(stream, "  cqo\n  idiv rdi")?,
                NdEq(_, _) => writeln!(stream, "  cmp rax, rdi\n  sete al\n  movzb rax, al")?,
                NdNeq(_, _) => writeln!(stream, "  cmp rax, rdi\n  setne al\n  movzb rax, al")?,
                NdLt(_, _) => writeln!(stream, "  cmp rax, rdi\n  setl al\n  movzb rax, al")?,
                NdLeq(_, _) => writeln!(stream, "  cmp rax, rdi\n  setle al\n  movzb rax, al")?,
                _ => unreachable!(),
            }

            writeln!(stream, "  push rax")?
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
        NdSizeof(operand) => {
            gen(stream, operand, n)?;

            let type_info = analyze_subtree(operand);
            let bytes = size_of_type(&type_info);

            writeln!(stream, "  push {}", bytes)?;
        }
        NdLVar(_, _) => {
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

            // rspの値をrbxに残しておき、rspを8の倍数に調整する。調整後にrbxをスタックトップに置くことで、16の倍数になる。
            writeln!(stream, "  mov rbx, rsp")?;
            writeln!(stream, "  and rsp, 0xfffffffffffffff8")?;
            writeln!(stream, "  push rbx")?;

            writeln!(stream, "  call {}", name)?;

            // 調整前のrspに復帰する。
            writeln!(stream, "  pop rsp")?;

            writeln!(stream, "  push rax")?;
        }
        NdAssign(lhs, rhs) => {
            if let NdDeref(ref node) = **lhs {
                gen(stream, &node, n)?;
            } else {
                get_lvar_addr(stream, lhs)?;
            }

            gen(stream, rhs, n)?;

            writeln!(stream, "  pop rdi")?;
            writeln!(stream, "  pop rax")?;
            writeln!(stream, "  mov [rax], rdi")?;
            writeln!(stream, "  push rdi")?;
        }
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
    if let NdLVar(offset, _) = *node {
        //let offset = offset * size_of_type(type_info);
        writeln!(stream, "  mov rax, rbp")?;
        writeln!(stream, "  sub rax, {}", offset)?;
        writeln!(stream, "  push rax")?;
    } else {
        panic!("left hand side in Assign is not variable.");
    }
    Ok(())
}

fn analyze_subtree(node: &Node) -> TypeInfo {
    match node {
        NdNum(_num) => TypeInfo::Int,
        NdAdd(lhs, _)
        | NdSub(lhs, _)
        | NdMul(lhs, _)
        | NdDiv(lhs, _)
        | NdEq(lhs, _)
        | NdNeq(lhs, _)
        | NdLt(lhs, _)
        | NdLeq(lhs, _) => analyze_subtree(&lhs),
        NdSizeof(operand) | NdRef(operand) | NdDeref(operand) => analyze_subtree(&operand),
        NdLVar(_, type_info) => type_info.clone(),
        NdCall(_, _) => TypeInfo::Int,
        _ => {
            dbg!("BUG:{}", node);
            unreachable!()
        }
    }
}

fn size_of_type(type_info: &TypeInfo) -> usize {
    match type_info {
        TypeInfo::Int => 4,
        TypeInfo::Pointer(_) => 8,
        TypeInfo::Array(type_) => 8,
    }
}

/// for p++;
fn size_pointer_to(type_info: &TypeInfo) -> usize {
    if let TypeInfo::Pointer(dst_type) = type_info {
        match **dst_type {
            TypeInfo::Int => 4,
            TypeInfo::Pointer(_) => 8,
            TypeInfo::Array(_) => 8,
        }
    } else {
        panic!("Argument is not Pointer");
    }
}

/*
fn print_opration_epilogue(stream: &mut BufWriter<File>, message: &str) -> Result<()> {
    writeln!(stream, "  pop rdi")?;
    writeln!(stream, "  pop rax")?;
    writeln!(stream, "{}", message)?;
    writeln!(stream, "  push rax")?;
    Ok(())
}
*/
