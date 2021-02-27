#!/bin/bash
cargo run -- foo.c
cc foo.s  -o main
./main
echo $?