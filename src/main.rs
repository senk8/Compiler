use::std::env;
use crate::Token::*;

pub enum Token{
    TkNum(usize),
    TkPlus,
    TkMinus,
    TkEof
}

impl Token{
    fn expect_num(&self) -> usize{
        match self {
           TkNum(n) => *n,
           _ => panic!("Error! expect number,found other")
        }
    }
}

pub struct Tokenizer<'a> {
    line: &'a str,
}

fn consume_num_greedy(line:&str)->(&str,&str){
    let first_non_num_idx = line.find(|c| !char::is_numeric(c)).unwrap_or(line.len());
    line.split_at(first_non_num_idx)
}

impl<'a> Iterator for Tokenizer<'a>{
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        // triming a head of string
        self.line = self.line.trim_start();

        match self.line.as_bytes()[0] {
            b'+' => {
                self.line = &self.line[1..];
                Some(TkPlus)
            },
            b'-' => {
                self.line = &self.line[1..];
                Some(TkMinus)
            }
            x if char::is_numeric(x) =>{
                let (head,tail) = consume_num_greedy(self.line);
                self.line = tail;
                Some(TkNum(usize::from_str_radix(head,10).unwrap()))
            },
            _ => panic!("unexpected token")
        }
    }
}

pub fn tokenize<'a>(line: &'a str) -> Tokenizer<'a> {
    Tokenizer { line }
}

fn main() {
    let arg = env::args().nth(1).unwrap();
    let tokens = tokenize(arg.as_str());

    print!(".intel_syntax noprefix\n");
    print!(".globl main\n");
    print!("main:\n");

    print!("  mov rax, {}\n" ,tokens.);

    for token in tokens{
        token
    }

    while let Some(op) = cs.next() {
        let n = {
            let s = cs.next().unwrap();
            usize::from_str_radix(s, 10).unwrap()
        };
        match op {
            "+" => println!("  add rax, {}", &n),
            "-" => println!("  sub rax, {}", &n),
             _  => panic!("予期しない文字です: '{}'\n", &op),
        }
    }

    println!("  ret");
    return;
}
