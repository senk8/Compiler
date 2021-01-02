use super::*;
use crate::types::node::Node::*;
use crate::types::node::*;
use crate::types::token::TokenKind::*;
use crate::types::error::ParseError;
use crate::types::error::ParseError::*;

macro_rules! new_node {
    ($nx:expr,$lhs:expr,$rhs:expr) => {
        match $nx{
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
        }
    };
}

impl<'a> Parser<'a> {
    //expr = assign
    pub(super) fn expr(&self) -> Result<Node,ParseError> {
        self.assign()
    }

    //assign = equality ( "=" assign )?
    pub(super) fn assign(&self) -> Result<Node,ParseError> {
        self.equality().and_then(|node|match self.look_ahead()? {
            Token(Assign) => Ok(new_node!(self.next_token()?,node,self.assign()?)),
            _ => Ok(node),
        })
    }

    // equality = relational ("==" relational | "!=" relational)*
    pub(super) fn equality(&self) -> Result<Node,ParseError> {
        self.relational().and_then(|mut node|loop{
           match self.look_ahead()? {
                Token(Eq) => node = new_node!(self.next_token()?,node,self.relational()?),
                Token(Neq) => node = new_node!(self.next_token()?,node,self.relational()?),
                _ => break Ok(node),
            }
        })
    }

    //relational = add ("<" add | "<=" add | ">" add| ">=" add) *
    pub(super) fn relational(&self) -> Result<Node,ParseError> {

        self.add().and_then(|mut node|loop { 
            match self.look_ahead()? {
                Token(Lt) => node = new_node!(self.next_token()?,node,self.add()?),
                Token(Leq) => node = new_node!(self.next_token()?,node,self.add()?),
                Token(Gt) => node = new_node!(self.next_token()?,self.add()?,node),
                Token(Geq) => node = new_node!(self.next_token()?,self.add()?,node),
                _ => break Ok(node),
            }
        })

    }

    // add    = mul ("+" mul | "-" mul)*
    pub(super) fn add(&self) -> Result<Node,ParseError> {
        self.mul().and_then(|mut node|loop { 
            match self.look_ahead()? {
                Token(Plus) => node = new_node!(self.next_token()?,node,self.mul()?),
                Token(Minus) => node = new_node!(self.next_token()?,node,self.mul()?),
                _ => break Ok(node),
            }
        })
    }

    // mul     = unary ("*" unary | "/" unary)*
    pub(super) fn mul(&self) -> Result<Node,ParseError> {
        self.unary().and_then(|mut node|loop { 
            match self.look_ahead()? {
                Token(Mul) => node=new_node!(self.next_token()?,node,self.unary()?),
                Token(Div) => node=new_node!(self.next_token()?,node,self.unary()?),
                _ => break Ok(node),
            }
        })
    }

    // unary    = ("+" | "-")?  primary
    pub(super) fn unary(&self) -> Result<Node,ParseError> {
        self.look_ahead().and_then(|tok|match tok {
                Token(Plus) => {
                    self.next_token()?;
                    self.primary()
                },
                Token(Minus) => Ok(new_node!(self.next_token()?,NdNum(0),self.primary()?)),
                _ => self.primary(),
            }
        )
    }

    // primary = num | ident | "(" expr ")"
    pub(super) fn primary(&self) -> Result<Node,ParseError> {
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
                    Ok(NdLVar(self.offset.get()))
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