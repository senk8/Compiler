assert() {
  expected="$1"
  input="$2"

  cargo run -- "$input"
  cc -o out out.s
  ./out
  result="$?"

  if [ "$result" = "$expected" ]; then
    echo "$input => $result"
  else
    echo "$input => $expected expected, but got $result"
    exit 1
  fi
}

assert 0 test_c/test_basic.c
assert 12 test_c/fn1.c
assert 5 test_c/fib.c
assert 8 test_c/sizeof.c
assert 0 test_c/array.c

cc test_c/for_alloc.c -c
cargo run -- test_c/alloc.c
cc out.s -c
cc out.o for_alloc.o -o main
./main
result="$?"
expected=2

if [ "$result" = "$expected" ]; then
  echo "test_c/test_alloc.c => $result"
else
  echo "test_c/test_alloc.c => $expected expected, but got $result"
  exit 1
fi