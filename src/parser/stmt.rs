use super::*;
use crate::types::token::TokenKind::*;
use crate::types::token::Symbol::*;
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

        let node = if self.consume_keyword(Return) {
            NdReturn(Box::new(self.expr()))
        }else{
            self.expr()
        };

        self.expect(Semicolon);
        node
    }
}
