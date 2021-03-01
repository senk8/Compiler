#!/bin/bash
cargo run -- foo.c
cc out.s  -o main
./main
echo $?