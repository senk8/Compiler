#!/bin/bash
cargo run -- hoge.c
cc out.s  -o main
./main
echo $?