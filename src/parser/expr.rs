use super::*;
use crate::types::node::Node::*;
use crate::types::node::*;
use crate::types::token::DelimitorKind::*;
use crate::types::token::OperatorKind::*;
use crate::types::token::TokenKind::*;

use crate::types::error::ParseError;

macro_rules! node {
    ($parser:expr,$f:ident,$lhs:expr,$rhs:expr) => {{
        let _ = $parser.consume();
        $f(Box::new($lhs), Box::new($rhs))
    }};
    ($parser:expr,$f:ident,$lhs:expr) => {{
        let _ = parser.consume();
        $f(Box::new($lhs))
    }};
}

impl<'a> Parser<'a> {
    //expr = assign
    pub(super) fn expr(&self) -> Result<Node, ParseError> {
        self.assign()
    }

    //assign = equality ( "=" assign )?
    pub(super) fn assign(&self) -> Result<Node, ParseError> {
        self.equality().and_then(|node|
                /* choice "=" assign or Epsilon */
                match self.look_ahead().map(|tk|tk.0){
                        Some(Opr(Assign)) => Ok(node!(self,NdAssign,node,self.assign()?)),
                        _ => Ok(node), // Parser infer what it is consumed by other non-teminal .
                })
    }

    // equality = relational ("==" relational | "!=" relational)*
    pub(super) fn equality(&self) -> Result<Node, ParseError> {
        self.relational().and_then(|mut node| loop {
            /* choice "=" assign or epsilon */
            match self.look_ahead().map(|tk| tk.0) {
                Some(Opr(Eq)) => node = node!(self, NdEq, node, self.relational()?),
                Some(Opr(Neq)) => node = node!(self, NdNeq, node, self.relational()?),
                _ => break Ok(node),
            }
        })
    }

    //relational = add ("<" add | "<=" add | ">" add| ">=" add) *
    pub(super) fn relational(&self) -> Result<Node, ParseError> {
        self.add().and_then(|mut node| loop {
            match self.look_ahead().map(|tk| tk.0) {
                /* choice "=" assign or None */
                Some(Opr(Lt)) => node = node!(self, NdLt, node, self.add()?),
                Some(Opr(Leq)) => node = node!(self, NdLeq, node, self.add()?),
                Some(Opr(Gt)) => node = node!(self, NdLt, self.add()?, node),
                Some(Opr(Geq)) => node = node!(self, NdLeq, self.add()?, node),
                _ => break Ok(node),
            }
        })
    }

    // add    = mul ("+" mul | "-" mul)*
    pub(super) fn add(&self) -> Result<Node, ParseError> {
        self.mul().and_then(|mut node| loop {
            match self.look_ahead().map(|tk| tk.0) {
                /* choice "=" assign or None */
                Some(Opr(Add)) => node = node!(self, NdAdd, node, self.mul()?),
                Some(Opr(Sub)) => node = node!(self, NdSub, node, self.mul()?),
                _ => break Ok(node),
            }
        })
    }

    // mul     = unary ("*" unary | "/" unary)*
    pub(super) fn mul(&self) -> Result<Node, ParseError> {
        self.unary().and_then(|mut node| loop {
            match self.look_ahead().map(|tk| tk.0) {
                Some(Opr(Mul)) => node = node!(self, NdMul, node, self.unary()?),
                Some(Opr(Div)) => node = node!(self, NdDiv, node, self.unary()?),
                _ => break Ok(node),
            }
        })
    }

    // unary    = ("+" | "-")?  primary
    pub(super) fn unary(&self) -> Result<Node, ParseError> {
        match self.look_ahead().map(|tk| tk.0) {
            Some(Opr(Add)) => {
                self.consume();
                self.primary()
            }
            Some(Opr(Sub)) => Ok(node!(self, NdSub, NdNum(0), self.primary()?)),
            _ => self.primary(),
        }
    }

    // primary = num | ident | "(" expr ")" | ident ( "(" ")" )?
    pub(super) fn primary(&self) -> Result<Node, ParseError> {
        use crate::types::error::ParseError::*;
        self.look_ahead()
            .ok_or(Eof(Pos(0,0)))
            .and_then(|tok| match tok.0 {
                Num(n) => {
                    self.consume();
                    Ok(NdNum(n))
                }
                Id(name) => {
                    self.consume();
                    self.look_ahead()
                        .ok_or(Eof(Pos(0,0)))
                        .and_then(|tok| match tok.0{
                            Delim(Rc) => {
                                let result = self.symbol_table.borrow().get(&name).cloned();

                                if let Some(lvar) = result {
                                    Ok(NdLVar(lvar.1))
                                } else {
                                    self.set_var(name);
                                    Ok(NdLVar(self.offset.get()))
                                }
                            },
                            _ => {
                                let result = self.symbol_table.borrow().get(&name).cloned();

                                if let Some(lvar) = result {
                                    Ok(NdLVar(lvar.1))
                                } else {
                                    self.set_var(name);
                                    Ok(NdLVar(self.offset.get()))
                                }
                            }
                       })
               }
                Delim(Rc) => {
                    self.consume();
                    let node = self.expr()?;
                    self.look_ahead()
                        .ok_or(Eof(Pos(0,0)))
                        .and_then(|tok| match tok.0 {
                            Delim(Lc) => Ok(node),
                            _ => Err(UnexpectedDelimitor(tok.1)),
                        })
                }
                _ => Err(UnexpectedToken(tok.1)),
            })
    }
}
