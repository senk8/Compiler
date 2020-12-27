use super::*;
use crate::types::token::TokenKind::*;
use crate::types::node::*;

impl<'a> Parser<'a>{

    // program = stmt *
    pub(super) fn program(&mut self)->Vec<Node>{
        let mut trees = vec![];

        while let Some(_) = self.look_ahead(){
            trees.push(self.stmt());
        }

        trees
    }

    /* stmt = expr ";" | "return" expr ";" */
    pub(super) fn stmt(&mut self)->Node{

        /*
        let node = if consume_keywoed(Return) {
            NdReturn(expr(tokenizer));
        }else{
            expr(tokenizer);
        };
        */

        let node = self.expr();
        self.expect(Semicolon);
        node
    }
}
