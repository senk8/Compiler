.intel_syntax noprefix
.globl main
main:
  push rbp
  mov rbp, rsp
  sub rsp, 208
  push 2
  push 2
  pop rdi
  pop rax
  imul rax, rdi
  push rax
  pop rax
  push 2
  push 2
  pop rdi
  pop rax
  add rax, rdi
  push rax
  pop rax
  push 2
  pop rax
  mov rsp, rbp
  pop rbp
  ret
  pop rax
  mov rsp, rbp
  pop rbp
  ret
