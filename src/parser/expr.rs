use super::*;
use crate::types::node::Node::*;
use crate::types::node::*;
use crate::types::token::TokenKind::*;
use crate::types::token::OperatorKind::*;
use crate::types::token::DelimitorKind::*;

use crate::types::error::ParseError;
use crate::types::error::ParseErrorKind::*;

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
                        Opr(Assign) => Ok(new_node!(self.next_token(),node,self.assign()?)),
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
                    Opr(Eq) => node = new_node!(self.next_token(),node,self.relational()?),
                    Opr(Neq) => node = new_node!(self.next_token(),node,self.relational()?),
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
                    Opr(Lt) => node = new_node!(self.next_token(),node,self.add()?),
                    Opr(Leq) => node = new_node!(self.next_token(),node,self.add()?),
                    Opr(Gt) => node = new_node!(self.next_token(),self.add()?,node),
                    Opr(Geq) => node = new_node!(self.next_token(),self.add()?,node),
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
                    Opr(Add) => node = new_node!(self.next_token(),node,self.mul()?),
                    Opr(Sub) => node = new_node!(self.next_token(),node,self.mul()?),
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
                    Opr(Mul) => node=new_node!(self.next_token(),node,self.unary()?),
                    Opr(Div) => node=new_node!(self.next_token(),node,self.unary()?),
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
                    self.next_token();
                    self.primary()
                },
                Opr(Sub) => Ok(new_node!(self.next_token(),NdNum(0),self.primary()?)),
                _ => self.primary(),
            }
        }else{self.primary()}
    }

    // primary = num | ident | "(" expr ")"
    pub(super) fn primary(&self) -> Result<Node,ParseError> {
        self.look_ahead().ok_or(self.make_error(LackExpr)).and_then(|tok|match tok.val {
            Delim(Rc) => {
                self.next_token();
                let node = self.expr()?;
                self.look_ahead().ok_or(self.make_error(Eof)).and_then(|tok|match tok.val{
                        Delim(Lc) => Ok(node),
                        _ => self.raise_error(UnclosedDelimitor,tok.pos),
                    }
                )
            },
            Id(name) => {
                self.next_token();
                let result = self.symbol_table.borrow().get(&name).cloned();

                if let Some(lvar) = result {
                    Ok(NdLVar(lvar.1))
                } else {
                    self.set_var(name);
                    Ok(NdLVar(self.offset.get()))
                }
            },
            Num(n) => {
                self.next_token();
                Ok(NdNum(n))
            }
            _ => self.raise_error(UnexpectedToken,tok.pos),
        })
   }
}