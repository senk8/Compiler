
use crate::tokenizer::*;
use crate::tokenizer::Tokenizer;
use crate::tokenizer::Token::*;
use core::iter::Peekable;

#[derive(Debug)]
pub enum Node{
    NdAdd{
        lhs:Box<Node>,
        rhs:Box<Node>
    },
    NdSub{
        lhs:Box<Node>,
        rhs:Box<Node>
    },
    NdMul{
        lhs:Box<Node>,
        rhs:Box<Node>
    },
    NdDiv{
        lhs:Box<Node>,
        rhs:Box<Node>
    },
    NdNum{
        val:usize
    }
}

//TODO below functions are not appropritate because it is not correspond LA.

pub fn expect_num<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>) -> usize{
    //TODO TKNumでないとifの中に入れないのだが、中でパターンマッチしないとだめか？
    if let TkNum(_) = tokenizer.peek().unwrap() {
        match tokenizer.next().unwrap(){
            TkNum(n) => n,
            _ => panic!("Error! expect number,found other")
        }
    }else{
        panic!("Error! expect number,found other");
    }
}

pub fn expect<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>,expect_token:&Token)->(){
    let token = tokenizer.peek().unwrap();

    if token != expect_token {
        panic!("Error! expect number,found other");
    }

    tokenizer.next();
}

pub fn consume<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>,expect_token:&Token)->bool {
    let token = tokenizer.peek().unwrap();

    if token != expect_token {
        false
    }else{
        tokenizer.next();
        true
    }
}

/*
pub fn expr()->Token{
    let node = NdMul();

    loop {
        if comsume('+'){
            node.lhs = Box::new(node);
            node.rhs = Box::new(NdMul());
        }else if consume('-') {
            node.lhs = Box::new(node);
            node.rhs = Box::new(NdMul());
        }else{
            break node;
        }
    }
}
*/


/*
Box::new(NdAdd(lhs,rhs))
Box::new(NdNum(val))
*/
