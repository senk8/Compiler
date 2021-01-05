use super::*;
use crate::types::node::Node::*;
use crate::types::node::*;
use crate::types::token::TokenKind::*;
use crate::types::token::OperatorKind::*;
use crate::types::token::DelimitorKind::*;

use crate::types::error::ParseError;
use crate::types::error::ParseErrorKind::*;

/*
macro_rules! new_node {
    ($nx:expr,$lhs:expr,$rhs:expr) => {
        match $nx.unwrap().val{
            Opr(Add) => NdAdd(Box::new($lhs), Box::new($rhs)),
            Opr(Sub) => NdSub(Box::new($lhs), Box::new($rhs)),
            Opr(Mul) => NdMul(Box::new($lhs), Box::new($rhs)),
            Opr(Div) => NdDiv(Box::new($lhs), Box::new($rhs)),
            Opr(Assign) => NdAssign(Box::new($lhs), Box::new($rhs)),
            Opr(Lt) => NdLt(Box::new($lhs), Box::new($rhs)),
            Opr(Gt) => NdLt(Box::new($lhs), Box::new($rhs)),
            Opr(Leq) => NdLeq(Box::new($lhs), Box::new($rhs)),
            Opr(Geq) => NdLeq(Box::new($lhs), Box::new($rhs)),
            Opr(Eq) => NdEq(Box::new($lhs), Box::new($rhs)),
            Opr(Neq) => NdNeq(Box::new($lhs), Box::new($rhs)),
            _ => unreachable!(),
        }
    };
}
*/

macro_rules! node {
    ($parser:expr,$f:ident,$lhs:expr,$rhs:expr) => {
        {
            let _ = $parser.consume();
            $f(Box::new($lhs),Box::new($rhs))
        }
    };
    ($parser:expr,$f:ident,$lhs:expr) => {
        {
            let _ = parser.consume();
            $f(Box::new($lhs))
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
        self.equality().and_then(|node|
                /* choice "=" assign or None */
                if let Some(tok) = self.look_ahead(){
                    match tok.val{
                        Opr(Assign) => Ok(node!(self,NdAssign,node,self.assign()?)),
                        _ => Ok(node),
                    }
                }else{ Ok(node) }
            )
    }

    // equality = relational ("==" relational | "!=" relational)*
    pub(super) fn equality(&self) -> Result<Node,ParseError> {
        self.relational().and_then(|mut node|loop{
            /* choice "=" assign or None */
            if let Some(tok) = self.look_ahead() {
                match tok.val{
                    Opr(Eq) => node = node!(self,NdEq,node,self.relational()?),
                    Opr(Neq) => node = node!(self,NdNeq,node,self.relational()?),
                    _ => break Ok(node),
                }
            }else {break Ok(node)}
        })
    }

    //relational = add ("<" add | "<=" add | ">" add| ">=" add) *
    pub(super) fn relational(&self) -> Result<Node,ParseError> {

        self.add().and_then(|mut node|loop { 
            if let Some(tok) = self.look_ahead() {
                /* choice "=" assign or None */
                match tok.val {
                    Opr(Lt) => node = node!(self,NdLt,node,self.add()?),
                    Opr(Leq) => node = node!(self,NdLeq,node,self.add()?),
                    Opr(Gt) => node = node!(self,NdLt,self.add()?,node),
                    Opr(Geq) => node = node!(self,NdLeq,self.add()?,node),
                    _ => break Ok(node),
                }
            }else{ break Ok(node); }
       })

    }

    // add    = mul ("+" mul | "-" mul)*
    pub(super) fn add(&self) -> Result<Node,ParseError> {
        self.mul().and_then(|mut node|loop { 
            if let Some(tok) = self.look_ahead(){
                /* choice "=" assign or None */
                match tok.val {
                    Opr(Add) => node = node!(self,NdAdd,node,self.mul()?),
                    Opr(Sub) => node = node!(self,NdSub,node,self.mul()?),
                    _ => break Ok(node),
                }
            }else{ break Ok(node); }
        })
    }

    // mul     = unary ("*" unary | "/" unary)*
    pub(super) fn mul(&self) -> Result<Node,ParseError> {
        self.unary().and_then(|mut node|loop { 
            if let Some(tok) = self.look_ahead(){
                match tok.val {
                    Opr(Mul) => node=node!(self,NdMul,node,self.unary()?),
                    Opr(Div) => node=node!(self,NdDiv,node,self.unary()?),
                    _ => break Ok(node), 
                }
            }else{ break Ok(node); }
        })
    }

    // unary    = ("+" | "-")?  primary
    pub(super) fn unary(&self) -> Result<Node,ParseError> {
        if let Some(tok) = self.look_ahead() {
            match tok.val {
                Opr(Add) => {
                    self.consume();
                    self.primary()
                },
                Opr(Sub) => Ok(node!(self,NdSub,NdNum(0),self.primary()?)),
                _ => self.primary(),
            }
        }else{self.primary()}
    }

    // primary = num | ident | "(" expr ")"
    pub(super) fn primary(&self) -> Result<Node,ParseError> {
        self.look_ahead().ok_or(self.make_error(LackExpr)).and_then(|tok|match tok.val {
            Delim(Rc) => {
                self.consume();
                let node = self.expr()?;
                self.look_ahead().ok_or(self.make_error(Eof)).and_then(|tok|match tok.val{
                        Delim(Lc) => Ok(node),
                        _ => self.raise_error(UnclosedDelimitor,tok.pos),
                    }
                )
            },
            Id(name) => {
                self.consume();
                let result = self.symbol_table.borrow().get(&name).cloned();

                if let Some(lvar) = result {
                    Ok(NdLVar(lvar.1))
                } else {
                    self.set_var(name);
                    Ok(NdLVar(self.offset.get()))
                }
            },
            Num(n) => {
                self.consume();
                Ok(NdNum(n))
            }
            _ => self.raise_error(UnexpectedToken,tok.pos),
        })
   }
}

