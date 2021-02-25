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

        /*
        self.equality().and_then(|node|
                /* choice "=" assign or Epsilon */
                match self.look_ahead().map(|tk|tk.0){
                        Some(Opr(Assign)) => Ok(node!(self,NdAssign,node,self.assign()?)),
                        _ => Ok(node), // Parser infer what it is consumed by other non-teminal .
                })

        */
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

        /*
        self.relational().and_then(|mut node| loop {
            /* choice "=" assign or epsilon */
            match self.look_ahead().map(|tk| tk.0) {
                Some(Opr(Eq)) => node = node!(self, NdEq, node, self.relational()?),
                Some(Opr(Neq)) => node = node!(self, NdNeq, node, self.relational()?),
                _ => break Ok(node),
            }
        })
        */
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

        /*
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
        */
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

        /*
        self.mul().and_then(|mut node| loop {
            match self.look_ahead().map(|tk| tk.0) {
                /* choice "=" assign or None */
                Some(Opr(Add)) => node = node!(self, NdAdd, node, self.mul()?),
                Some(Opr(Sub)) => node = node!(self, NdSub, node, self.mul()?),
                _ => break Ok(node),
            }
        })
        */
    }

    // mul     = unary ("*" unary | "/" unary)*
    pub(super) fn mul(&mut self) -> Result<Node, ParseError> {
        let mut node = self.unary()?;

        loop {
            if self.choice(Opr(Mul)) {
                node = NdMul(Box::new(node), Box::new(self.unary()?));
            } else if self.choice(Opr(Div)) {
                node = NdDiv(Box::new(node), Box::new(self.unary()?));
            } else {
                break Ok(node);
            }
        }

        /*
        self.unary().and_then(|mut node| loop {
            match self.look_ahead().map(|tk| tk.0) {
                Some(Opr(Mul)) => node = node!(self, NdMul, node, self.unary()?),
                Some(Opr(Div)) => node = node!(self, NdDiv, node, self.unary()?),
                _ => break Ok(node),
            }
        })
        */
    }

    // unary    = ("+" | "-")?  primary
    pub(super) fn unary(&mut self) -> Result<Node, ParseError> {
        if self.choice(Opr(Add)) {
            self.primary()
        } else if self.choice(Opr(Sub)) {
            Ok(NdSub(Box::new(NdNum(0)), Box::new(self.primary()?)))
        } else {
            self.primary()
        }

        /*
        match self.look_ahead().map(|tk| tk.0) {
            Some(Opr(Add)) => {
                self.consume();
                self.primary()
            }
            Some(Opr(Sub)) => Ok(node!(self, NdSub, NdNum(0), self.primary()?)),
            _ => self.primary(),
        }
        */
    }

    // primary = num | ident | "(" expr ")" | ident ( "("  ( expr "," )*  ")" )?
    pub(super) fn primary(&mut self) -> Result<Node, ParseError> {
        use crate::types::error::ParseError::*;

        /*
        if self.choice(Num(n)){
            Ok(NdNum(n))
        }else if self.choice(Ident(name)){
            if self.choice(Delim(Lc)){
                let mut args = vec![];

                if let Some((Delim(Rc),_)) = self.look_ahead() {
                    self.consume();
                }else{
                    args.push(self.expr()?);

                    while let Some((Delim(Comma),_))= self.look_ahead(){
                        self.consume();
                        args.push(self.expr()?);
                    }

                    self.expect_tk(Delim(Rc))?;
                }

                Ok(NdCall(name.to_string(),args))
            }else{
                let result = self.symbol_table.get(&name).cloned();

                if let Some(lvar) = result {
                    Ok(NdLVar(lvar.1))
                } else {
                    self.set_var(name);
                    Ok(NdLVar(self.offset))
                }
            }
        }else if self.choice(Delim(Lc)){
            let node = self.expr()?;
            self.expect(Delim(Rc))?;
            node
        }else{
            Err(UnexpectedToken(tok.1))
        }
        */
        self.look_ahead()
            .ok_or(MissingExpression(Default::default()))
            .and_then(|tok| match tok.0 {
                Num(n) => {
                    self.consume();
                    Ok(NdNum(n))
                }
                Id(name) => {
                    self.consume();
                    match self.look_ahead().map(|tok| tok.0) {
                        Some(Delim(Lc)) => {
                            self.consume();

                            let mut args = vec![];

                            if let Some((Delim(Rc), _)) = self.look_ahead() {
                                self.consume();
                            } else {
                                args.push(self.expr()?);

                                while let Some((Delim(Comma), _)) = self.look_ahead() {
                                    self.consume();
                                    args.push(self.expr()?);
                                }

                                self.expect(Delim(Rc))?;
                            }

                            Ok(NdCall(name.to_string(), args))

                            /*
                            if self.choice(Rc) {

                            }else{
                                while let Ok(node) = self.expr() { nodes.push(node);

                                    self.expect_tk(Delim(Comma))?;
                                    if self.consume(Rc) {
                                        break;
                                    }
                                }
                            }
                            */
                        }
                        _ => {
                            let result = self.symbol_table.get(&name).cloned();

                            if let Some(lvar) = result {
                                Ok(NdLVar(lvar.1))
                            } else {
                                self.set_var(name);
                                Ok(NdLVar(self.offset))
                            }
                        }
                    }
                }
                Delim(Lc) => {
                    self.consume();
                    let node = self.expr()?;
                    self.look_ahead()
                        .ok_or(MissingDelimitor(Default::default()))
                        .and_then(|tok| match tok.0 {
                            Delim(Rc) => {
                                self.consume();
                                Ok(node)
                            }
                            _ => Err(UnexpectedDelimitor(tok.1)),
                        })
                }
                _ => Err(UnexpectedToken(tok.1)),
            })
    }
}
