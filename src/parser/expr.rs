use super::*;
use crate::types::token::TokenKind::*;
use crate::types::node::*;
use crate::types::node::Node::*;

impl<'a> Parser<'a>{
    //expr = assign
    pub(super) fn expr(&mut self)->Node{
        self.assign()
    }

    //assign = equality ( "=" assign )?
    pub(super) fn assign(&mut self)->Node{
        let mut node = self.equality();
        if self.consume_token(Assign) {
            node = NdAssign(Box::new(node),Box::new(self.assign()));
        }
        node
    }

    // equality = relational ("==" relational | "!=" relational)*
    pub(super) fn equality(&mut self)->Node{
        let mut node = self.relational();

        loop{
            if self.consume_token(Eq){
                node = NdEq(Box::new(node),Box::new(self.relational()));
            }else if self.consume_token(Neq){
                node = NdNeq(Box::new(node),Box::new(self.relational()));
            }else{
                break node;
            }
        }
    }

    //relational = add ("<" add | "<=" add | ">" add| ">=" add) *
    pub(super) fn relational(&mut self)->Node{
        let mut node = self.add();

        loop{
            if self.consume_token(Lt){
                node = NdLt(Box::new(node),Box::new(self.add()));
            }else if self.consume_token(Leq){
                node = NdLeq(Box::new(node),Box::new(self.add()));
            }else if self.consume_token(Gt){
                node = NdLt(Box::new(self.add()),Box::new(node));
            }else if self.consume_token(Geq){
                node = NdLeq(Box::new(self.add()),Box::new(node));
            }else{
                break node;
            }
        }
    }

    // This function represent following grammar.
    // add    = mul ("+" mul | "-" mul)*
    pub(super) fn add(&mut self)->Node{
        let mut node = self.mul();

        loop {
            if self.consume_token(Plus){
                node = NdAdd(Box::new(node),Box::new(self.mul()));
            }else if self.consume_token(Minus) {
                node = NdSub(Box::new(node),Box::new(self.mul()));
            }else{
                break node;
            }
        }
    }

    // This function represent following grammar.
    // mul     = unary ("*" unary | "/" unary)*
    pub(super) fn mul(&mut self)->Node{
        let mut node = self.unary();

        loop {
            if self.consume_token(Mul){
                node = NdMul(Box::new(node),Box::new(self.unary()));
            }else if self.consume_token(Div) {
                node = NdDiv(Box::new(node),Box::new(self.unary()));
            }else{
                break node;
            }
        }
    }

    // This function represent following grammar.
    // primary = num | ident | "(" expr ")"*
    pub(super) fn primary(&mut self)->Node{
        if self.consume_token(Rc) {
            let node = self.expr();
            self.expect(Lc);
            node
        }else if let Some(name) = self.take_ident(){
            let result = self.symbol_table.borrow().get(&name).cloned();

            if let Some(lvar)= result {
                NdLVar(lvar.1)
            }else{
                self.set_var(name);
                NdLVar(self.offset)
            }
        }else{
            NdNum(self.take_num().expect("Error! expect number,found other"))
        }
    }

    // This function represents following grammar.
    // unary    = ("+" | "-")?  primary
    pub(super) fn unary(&mut self)->Node{
        if self.consume_token(Plus){
            self.primary()
        }else if self.consume_token(Minus){
            NdSub(Box::new(NdNum(0)),Box::new(self.primary()))
        }else{
            self.primary()
        }
    }

}












