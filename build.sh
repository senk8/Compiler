#!/bin/bash
cargo run -- foo.c > foo.s
cc foo.s  -o main
./main
echo $?