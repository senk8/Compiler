
use crate::tokenizer::Tokenizer;
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

/*
struct Parser<'a>{
    tokenizer:&'a Tokenizer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(tk:&'a Tokenizer)->Parser<'a>{
        Parser{ tokenizer:tk }
    }

    pub fn expect_num(&mut self) -> usize{
        let token = self.next().unwrap();
        match token {
           TkNum(n) => n,
           _ => panic!("Error! expect number,found other")
        }
    }

    pub fn expect(&mut self,op:char) {
        let token = self.next().unwrap();
        match token {
           TkPlus => if op!='+' { panic!("Error! expect number,found other")},
           TkMinus => if op!='-' { panic!("Error! expect number,found other")},
           _ => panic!("Error! expect number,found other")
        }
    }
}
*/
/*
    pub fn consume(&mut self,op:char)->bool {
        let token = self.next().unwrap();
        match token {
            TkPlus => if op=='+' { return true } else { return false},
            TkMinus => if op=='-' { return true } else { return false},
            _ => panic!("Error! expect number,found other")
        }
        token = token->next;
        return true;
    }
*/


/*
fn expr(){
    mul();
    loop{
        ma

    }
}
*/