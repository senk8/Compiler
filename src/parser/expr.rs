use super::*;
use crate::types::node::Node::*;
use crate::types::node::*;
use crate::types::token::TokenKind::*;
use crate::types::error::ParseError;
use crate::types::error::ParseError::*;

macro_rules! new_node {
    ($nx:expr,$lhs:expr,$rhs:expr) => {
        match $nx?{
            Token(Plus) => NdAdd(Box::new($lhs), Box::new($rhs)),
            Token(Minus) => NdSub(Box::new($lhs), Box::new($rhs)),
            Token(Mul) => NdMul(Box::new($lhs), Box::new($rhs)),
            Token(Div) => NdDiv(Box::new($lhs), Box::new($rhs)),
            Token(Assign) => NdAssign(Box::new($lhs), Box::new($rhs)),
            Token(Lt) => NdLt(Box::new($lhs), Box::new($rhs)),
            Token(Gt) => NdLt(Box::new($lhs), Box::new($rhs)),
            Token(Leq) => NdLeq(Box::new($lhs), Box::new($rhs)),
            Token(Geq) => NdLeq(Box::new($lhs), Box::new($rhs)),
            Token(Eq) => NdEq(Box::new($lhs), Box::new($rhs)),
            Token(Neq) => NdNeq(Box::new($lhs), Box::new($rhs)),
            _ => unreachable!(),
        };
    };
}

impl<'a> Parser<'a> {
    //expr = assign
    pub(super) fn expr(&mut self) -> Result<Node,ParseError> {
        self.assign()
    }

    //assign = equality ( "=" assign )?
    pub(super) fn assign(&mut self) -> Result<Node,ParseError> {
        self.equality().and_then(|node|match self.look_ahead() {
            Ok(Token(Assign)) => Ok(new_node!(self.next_token(),node,self.assign()?)),
            _ => Ok(node),
        })
    }

    // equality = relational ("==" relational | "!=" relational)*
    pub(super) fn equality(&mut self) -> Result<Node,ParseError> {
        self.relational().and_then(|mut node|loop{
           match self.look_ahead() {
                Ok(Token(Eq)) => node = new_node!(self.next_token(),node,self.relational()?),
                Ok(Token(Neq)) => node = new_node!(self.next_token(),node,self.relational()?),
                _ => break Ok(node),
            }
        })
    }

    //relational = add ("<" add | "<=" add | ">" add| ">=" add) *
    pub(super) fn relational(&mut self) -> Result<Node,ParseError> {

        self.add().and_then(|mut node|loop { 
            match self.look_ahead() {
                Ok(Token(Lt)) => node = new_node!(self.next_token(),node,self.add()?),
                Ok(Token(Leq)) => node = new_node!(self.next_token(),node,self.add()?),
                Ok(Token(Gt)) => node = new_node!(self.next_token(),self.add()?,node),
                Ok(Token(Geq)) => node = new_node!(self.next_token(),self.add()?,node),
                _ => break Ok(node),
            }
        })

    }

    // add    = mul ("+" mul | "-" mul)*
    pub(super) fn add(&mut self) -> Result<Node,ParseError> {
        self.mul().and_then(|mut node|loop { 
            match self.look_ahead() {
                Ok(Token(Plus)) => node = new_node!(self.next_token(),node,self.mul()?),
                Ok(Token(Minus)) => node = new_node!(self.next_token(),node,self.mul()?),
                _ => break Ok(node),
            }
        })
    }

    // mul     = unary ("*" unary | "/" unary)*
    pub(super) fn mul(&mut self) -> Result<Node,ParseError> {
        self.unary().and_then(|mut node|loop { 
            match self.look_ahead() {
                Ok(Token(Mul)) => node=new_node!(self.next_token(),node,self.unary()?),
                Ok(Token(Div)) => node=new_node!(self.next_token(),node,self.unary()?),
                _ => break Ok(node),
            }
        })
    }

    // unary    = ("+" | "-")?  primary
    pub(super) fn unary(&mut self) -> Result<Node,ParseError> {
        self.look_ahead().and_then(|tok|match tok {
                Token(Plus) => {
                    self.next_token()?;
                    self.primary()
                },
                Token(Minus) => Ok(new_node!(self.next_token(),NdNum(0),self.primary()?)),
                _ => self.primary(),
            }
        )
    }

    // primary = num | ident | "(" expr ")"
    pub(super) fn primary(&mut self) -> Result<Node,ParseError> {
        self.look_ahead().and_then(|tok|match tok {
            Token(Rc) => {
                self.next_token()?;
                let node = self.expr()?;
                match self.look_ahead()?{
                    Token(Lc) => Ok(node),
                    _ => Err(UnclosedDelimitor),
                }
            },
            Id(name) => {
                self.next_token()?;
                let result = self.symbol_table.borrow().get(&name).cloned();

                if let Some(lvar) = result {
                    Ok(NdLVar(lvar.1))
                } else {
                    self.set_var(name);
                    Ok(NdLVar(self.offset))
                }
            },
            Num(n) => {
                self.next_token()?;
                Ok(NdNum(n))
            }
            _ => Err(UnexpectedToken),
        })
   }
}

    /*

        self.relational().and_then(|mut node|loop{
            if let Ok(_) = self.expect_token(Eq) {
                node = NdEq(Box::new(node), Box::new(self.relational()?));
            } else if let Ok(_) = self.expect_token(Neq) {
                node = NdNeq(Box::new(node), Box::new(self.relational()?));
            } else {
                break Ok(node);
            }
        )    
 
        self.add().and_then(|mut node|loop{
            if let Ok(_) = self.expect_token(Lt) {
                node = NdLt(Box::new(node), Box::new(self.add()?));
            } else if let Ok(_) = self.expect_token(Leq) {
                node = NdLeq(Box::new(node), Box::new(self.add()?));
            } else if let Ok(_) = self.expect_token(Gt) {
                node = NdLt(Box::new(self.add()?), Box::new(node));
            } else if let Ok(_) = self.expect_token(Geq) {
                node = NdLeq(Box::new(self.add()?), Box::new(node));
            } else {
                break Ok(node);
            }
        })
 */
        /* 
        self.equality().and_then(|node|
            if let Ok(_) = self.expect_token(Assign) {
                Ok(NdAssign(Box::new(node), Box::new(self.assign()?)))
            }else{
                Ok(node)
            }
        )
        */

        /*
        self.mul().and_then(|mut node|loop{
            if let Ok(_) = self.expect_token(Plus) {
                node = NdAdd(Box::new(node), Box::new(self.mul()?));
            } else if let Ok(_) = self.expect_token(Minus) {
                node = NdSub(Box::new(node), Box::new(self.mul()?));
            } else {
                break Ok(node);
            }
        })
        */

        /*
        self.unary().and_then(|mut node|loop{
            if let Ok(_) = self.expect_token(Mul) {
                node = NdMul(Box::new(node), Box::new(self.unary()?));
            } else if let Ok(_) = self.expect_token(Div) {
                node = NdDiv(Box::new(node), Box::new(self.unary()?));
            } else {
                break Ok(node);
            }
        })
        */

        /*
        if let Ok(_) = self.expect_token(Plus) {
            self.primary()
        } else if let Ok(_) = self.expect_token(Minus) {
            Ok(NdSub(Box::new(NdNum(0)), 
                     Box::new(self.primary()?))
            )
        } else {
            self.primary()
        }
        */
        /*
        if let Ok(_) = self.expect_delimitor(Rc) {
            let node = self.expr()?;
            self.expect_delimitor(Lc)?;
            Ok(node)
        } else if let Ok(name) = self.expect_id() {
            let result = self.symbol_table.borrow().get(&name).cloned();

            if let Some(lvar) = result {
                Ok(NdLVar(lvar.1))
            } else {
                self.set_var(name);
                Ok(NdLVar(self.offset))
            }
        } else {
            Ok(NdNum(self.expect_num()?))
        }
        */
 