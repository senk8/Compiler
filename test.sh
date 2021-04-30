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

assert 12 test_c/fn1.c
assert 5 test_c/fib.c