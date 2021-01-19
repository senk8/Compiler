#!/bin/bash
assert() {
  expected="$1"
  input="$2"

  cargo run -- -c "$input" > tmp.s
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 "0;"
assert 42 "42;"
assert 1 "1<=3;"
assert 21 "5 + 20 - 4;"
assert 142 "7*20+2;"
assert 1 "1!=3;"
assert 0 "1==3;"
assert 0 "1>3;"
assert 4 "a=2;a*2;"
assert 36 "a=3;b=12;a*b;"
assert 36 "abc=3;bc=12;abc*bc;"
assert 29 "hoge=3;bar=26;hoge+bar;"
assert 29 "return 29;"
assert 29 "hoge=3;bar=26;return hoge+bar;"
assert 15 "if(1<2)3+12;"
assert 15 "if(2<1)3+11; else 15;"
assert 5 "k=1;while(k<5)k=k+1;k;"
assert 5 "for(k=1;k<5;k=k+1)1+2;k;"
assert 2 "
          {
            2*2;
            2+2;
            return 2;
          }
         "
assert 5 "
          if(1==1){
            x = 2;
            if(2*2==4){
              if(x==2)x = x + 3; 
              else return 2;
            }
            return x;
          }
       " 

echo OK
