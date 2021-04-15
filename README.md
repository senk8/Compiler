# Compiler

The C Compiler implemantation by Rustlang . 

# Features

- This is a compiler which contains Rust.
- The input format is as a file or as a row program text.

# Design

![parser_struct](https://user-images.githubusercontent.com/44151180/110480341-ba0d2d00-8129-11eb-9551-0a033cdd84b8.png)

# Requirements

- rustc 1.49.0

# Usage

1. Go ahead and clone this repo. 

2. You can compile `test.c` to `out.s`.

```
$ cargo run -- test.c
```

and You can also use the following command.

```
$ cargo run -- -c "2+5;"
```

# Contribution

# License

# Progress

Compiler can complie a functions declaration .

# Reference

- 9cc
https://www.sigbus.info/compilerbook

