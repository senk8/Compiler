use super::*;
use crate::types::node::Node::*;
use crate::types::node::*;
use crate::types::token::DelimitorKind::*;
use crate::types::token::OperatorKind::*;
use crate::types::token::TokenKind::*;

use crate::types::error::ParseError;

/*
macro_rules! option {
    ($kind:expr,$do:expr)=>{
        if self.choice($kind) {
            $do
        }else{
            Ok(node)
        }
    }
}

macro_rules! star {
    ($kind:expr,$do:expr)=>{
        loop {
            else{
                Ok(node)
            }
        }
    }
}
 */

impl<'a> Parser<'a> {
    //expr = assign
    pub(super) fn expr(&mut self) -> Result<Node, ParseError> {
        self.assign()
    }

    //assign = equality ( "=" assign )?
    pub(super) fn assign(&mut self) -> Result<Node, ParseError> {
        let node = self.equality()?;

        if self.choice(Opr(Assign)) {
            Ok(NdAssign(Box::new(node), Box::new(self.assign()?)))
        } else {
            Ok(node)
        }
    }

    // equality = relational ("==" relational | "!=" relational)*
    pub(super) fn equality(&mut self) -> Result<Node, ParseError> {
        let mut node = self.relational()?;

        loop {
            if self.choice(Opr(Eq)) {
                node = NdEq(Box::new(node), Box::new(self.relational()?));
            } else if self.choice(Opr(Neq)) {
                node = NdNeq(Box::new(node), Box::new(self.relational()?));
            } else {
                break Ok(node);
            }
        }
    }

    //relational = add ("<" add | "<=" add | ">" add| ">=" add) *
    pub(super) fn relational(&mut self) -> Result<Node, ParseError> {
        let mut node = self.add()?;

        loop {
            if self.choice(Opr(Lt)) {
                node = NdLt(Box::new(node), Box::new(self.add()?));
            } else if self.choice(Opr(Leq)) {
                node = NdLeq(Box::new(node), Box::new(self.add()?));
            } else if self.choice(Opr(Gt)) {
                node = NdLt(Box::new(self.add()?), Box::new(node));
            } else if self.choice(Opr(Geq)) {
                node = NdLeq(Box::new(self.add()?), Box::new(node));
            } else {
                break Ok(node);
            }
        }
    }

    // add    = mul ("+" mul | "-" mul)*
    pub(super) fn add(&mut self) -> Result<Node, ParseError> {
        let mut node = self.mul()?;

        loop {
            if self.choice(Opr(Add)) {
                node = NdAdd(Box::new(node), Box::new(self.mul()?));
            } else if self.choice(Opr(Sub)) {
                node = NdSub(Box::new(node), Box::new(self.mul()?));
            } else {
                break Ok(node);
            }
        }
    }

    // mul     = unary ("*" unary | "/" unary)*
    pub(super) fn mul(&mut self) -> Result<Node, ParseError> {
        let mut node = self.unary()?;

        loop {
            if self.choice(Opr(Star)) {
                node = NdMul(Box::new(node), Box::new(self.unary()?));
            } else if self.choice(Opr(Div)) {
                node = NdDiv(Box::new(node), Box::new(self.unary()?));
            } else {
                break Ok(node);
            }
        }
    }

    /*
    unary    = "+" primary 
            |  "-" primary
            |  "*" primary
            |  "&" primary
            |  primary
    */
    pub(super) fn unary(&mut self) -> Result<Node, ParseError> {
        if self.choice(Opr(Add)) {
            self.primary()
        } else if self.choice(Opr(Sub)) {
            Ok(NdSub(Box::new(NdNum(0)), Box::new(self.primary()?)))
        } else if self.choice(Opr(Star)) {
            Ok(NdDeref(Box::new(self.primary()?)))
        } else if self.choice(Opr(Amp)) {
            Ok(NdRef(Box::new(self.primary()?)))
        } else {
            self.primary()
        }
    }

    // primary = num | ident | "(" expr ")" | ident ( "(" argument ")" )?
    // argument = (expr ( "," expr )* ) ?
    pub(super) fn primary(&mut self) -> Result<Node, ParseError> {
        use crate::types::error::ParseError::*;

        if let Some(Num(n)) = self.take_num() {
            Ok(NdNum(n))
        } else if let Some(Id(name)) = self.take_id() {
            if self.choice(Delim(Lc)) {
                let mut args = vec![];

                /* exprにマッチすることを先読みできないので、")"がないかどうかを選択肢にしている。 */
                if !self.choice(Delim(Rc)) {
                    args.push(self.expr()?);
                    loop {
                        if self.choice(Delim(Comma)) {
                            args.push(self.expr()?);
                        } else {
                            break;
                        };
                    }
                    self.expect(Delim(Rc))?;
                }

                Ok(NdCall(name.to_string(), args))
            } else {
                let result = self.symbol_table.get(&name).cloned();

                if let Some(lvar) = result {
                    Ok(NdLVar(lvar.1))
                } else {
                    self.set_var(name);
                    Ok(NdLVar(self.offset))
                }
            }
        } else if self.choice(Delim(Lc)) {
            let node = self.expr()?;
            self.expect(Delim(Rc))?;
            Ok(node)
        } else {
            Err(UnexpectedToken(self.look_ahead().unwrap().1))
        }
    }
}
