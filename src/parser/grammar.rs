use std::iter::Peekable;

use super::Parser;
use super::Lexer;

use crate::types::error::ParseError;
use crate::types::error::ParseError::*;
use crate::types::node::Node;
use crate::types::node::Node::*;
use crate::types::token::DelimitorKind::*;
use crate::types::token::KeywordKind::*;
use crate::types::token::OperatorKind::*;
use crate::types::token::TokenKind::*;
use crate::types::token::TypeKind::*;
use crate::types::variable::VarAnnot;


impl Parser {
    // program = decl *
    pub(super) fn program(&mut self,lexer:&mut Peekable<Lexer>) -> Result<Vec<Node>, ParseError> {
        let mut trees = vec![];

        while let Some(_) = self.look_ahead(lexer) {
            trees.push(self.decl(lexer)?);
            self.symbol_table.clear();
        }

        Ok(trees)
    }

    // decl = type ident ( ( type ident "," )* ) "{" stmt * "}"
    fn decl(&mut self,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
        /* 引数コンパイルしたら同時にローカル変数の定義を行う。*/

        let _ = self.take_type(lexer).ok_or_else(||
            UnexpectedToken(lexer.next().unwrap())
        )?;

        if let Some(Id(name)) = self.take_id(lexer) {
            self.expect(lexer,Delim(Lparen))?;

            let mut args = vec![];
            if !self.choice(lexer,Delim(Rparen)) {
                loop {

                    if let Some(ty) = self.take_type(lexer){

                        let token = self.take_token(lexer).ok_or(Eof)?;

                        if let (Id(name), _) = token {
                            self.set_var(name, ty);
                            args.push(NdLVar(self.offset));

                            if !self.choice(lexer,Delim(Comma)) {
                                self.expect(lexer,Delim(Rparen))?;
                                break;
                            }

                        } else {
                            return Err(UnexpectedToken(token));
                        }
                    }else{
                        panic!("Expect Type fonund");
                    }
                }
            };

            self.expect(lexer,Delim(Lbrace))?;

            let mut nodes = Vec::new();

            while !self.choice(lexer,Delim(Rbrace)) {
                nodes.push(self.stmt(lexer)?);
            }

            Ok(NdDecl(name, args, Box::new(NdBlock(nodes))))
        } else {
            /* TODO: it will be unused look_ahead.unwrap() */ 
            Err(UnexpectedToken(self.look_ahead(lexer).unwrap()))
        }
    }

    /// stmt = expr ";"
    /// | "{" stmt* "}""
    /// | "return" expr ";"
    /// | "if" "(" expr ")" stmt ("else" stmt)?
    /// | "while" "(" expr ")" stmt
    /// | "for" "(" expr? ";" expr? ";" expr? ")" stmt
    fn stmt(&mut self,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
        /* choice expr or return */

        if self.choice(lexer,Key(Return)) {
            let node = NdReturn(Box::new(self.expr(lexer)?));
            self.expect(lexer,Delim(Semicolon))?;
            Ok(node)
        } else if self.choice(lexer,Delim(Lbrace)) {
            let mut nodes = Vec::new();
            while !self.choice(lexer,Delim(Rbrace)) {
                nodes.push(self.stmt(lexer)?);
            }
            Ok(NdBlock(nodes))
        } else if self.choice(lexer,Key(If)) {
            self.expect(lexer,Delim(Lparen))?;
            let first = self.expr(lexer)?;
            self.expect(lexer,Delim(Rparen))?;
            let second = self.stmt(lexer)?;

            if self.choice(lexer,Key(Else)) {
                let third = self.stmt(lexer)?;
                Ok(NdIfElse(Box::new(first), Box::new(second), Box::new(third)))
            } else {
                Ok(NdIf(Box::new(first), Box::new(second)))
            }
        } else if self.choice(lexer,Key(While)) {
            self.expect(lexer,Delim(Lparen))?;
            let first = self.expr(lexer)?;
            self.expect(lexer,Delim(Rparen))?;
            let second = self.stmt(lexer)?;
            Ok(NdWhile(Box::new(first), Box::new(second)))
        } else if self.choice(lexer,Key(For)) {
            self.expect(lexer,Delim(Lparen))?;
            let first = self.expr(lexer)?;
            self.expect(lexer,Delim(Semicolon))?;
            let second = self.expr(lexer)?;
            self.expect(lexer,Delim(Semicolon))?;
            let third = self.expr(lexer)?;
            self.expect(lexer,Delim(Rparen))?;
            let fourth = self.stmt(lexer)?;
            Ok(NdFor(
                Box::new(first),
                Box::new(second),
                Box::new(third),
                Box::new(fourth),
            ))
        } else {
            let node = self.expr(lexer)?;
            self.expect(lexer,Delim(Semicolon))?;
            Ok(node)
        }
    }
}

impl Parser {
    //expr = assign | type ident
    fn expr(&mut self,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
        if let Some(ty) = self.take_type(lexer){
            let token = self.take_token(lexer).ok_or(Eof)?;

            if let (Id(name), _) = token {
                self.set_var(name, ty);
                Ok(NdVdecl(self.offset))
            } else {
                Err(UnexpectedToken(token))
            }
        } else {
            self.assign(lexer)
        }
    }

/*
        if self.choice(Type(Int)) {

            let ty = self.type_helper();

            dbg!("{:?}",&ty);

            let token = self.take_token().ok_or(Eof)?;

            if let (Id(name), _) = token {
                self.set_var(name, ty);
                Ok(NdVdecl(self.offset))
            } else {
                Err(UnexpectedToken(token))
            }
 */

    //assign = equality ( "=" assign )?
    fn assign(&mut self,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
        let node = self.equality(lexer)?;

        if self.choice(lexer,Opr(Assign)) {
            Ok(NdAssign(Box::new(node), Box::new(self.assign(lexer)?)))
        } else {
            Ok(node)
        }
    }

    // equality = relational ("==" relational | "!=" relational)*
    fn equality(&mut self,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
        let mut node = self.relational(lexer)?;

        loop {
            if self.choice(lexer,Opr(Eq)) {
                node = NdEq(Box::new(node), Box::new(self.relational(lexer)?));
            } else if self.choice(lexer,Opr(Neq)) {
                node = NdNeq(Box::new(node), Box::new(self.relational(lexer)?));
            } else {
                break Ok(node);
            }
        }
    }

    //relational = add ("<" add | "<=" add | ">" add| ">=" add) *
    fn relational(&mut self,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
        let mut node = self.add(lexer)?;

        loop {
            if self.choice(lexer,Opr(Lt)) {
                node = NdLt(Box::new(node), Box::new(self.add(lexer)?));
            } else if self.choice(lexer,Opr(Leq)) {
                node = NdLeq(Box::new(node), Box::new(self.add(lexer)?));
            } else if self.choice(lexer,Opr(Gt)) {
                node = NdLt(Box::new(self.add(lexer)?), Box::new(node));
            } else if self.choice(lexer,Opr(Geq)) {
                node = NdLeq(Box::new(self.add(lexer)?), Box::new(node));
            } else {
                break Ok(node);
            }
        }
    }

    // add    = mul ("+" mul | "-" mul)*
    fn add(&mut self,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
        let mut node = self.mul(lexer)?;

        loop {
            if self.choice(lexer,Opr(Add)) {
                node = NdAdd(Box::new(node), Box::new(self.mul(lexer)?));
            } else if self.choice(lexer,Opr(Sub)) {
                node = NdSub(Box::new(node), Box::new(self.mul(lexer)?));
            } else {
                break Ok(node);
            }
        }
    }

    // mul     = unary ("*" unary | "/" unary)*
    fn mul(&mut self,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
        let mut node = self.unary(lexer)?;

        loop {
            if self.choice(lexer,Opr(Star)) {
                node = NdMul(Box::new(node), Box::new(self.unary(lexer)?));
            } else if self.choice(lexer,Opr(Div)) {
                node = NdDiv(Box::new(node), Box::new(self.unary(lexer)?));
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
            |  "sizeof" primary
            |  primary
    */
    fn unary(&mut self,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
        if self.choice(lexer,Opr(Add)) {
            self.primary(lexer)
        } else if self.choice(lexer,Opr(Sub)) {
            Ok(NdSub(Box::new(NdNum(0)), Box::new(self.primary(lexer)?)))
        } else if self.choice(lexer,Opr(Star)) {
            Ok(NdDeref(Box::new(self.primary(lexer)?)))
        } else if self.choice(lexer,Opr(Amp)) {
            Ok(NdRef(Box::new(self.primary(lexer)?)))
        /*
        } else if self.choice(Opr(Sizeof)) {
            let node = self.primary()?;

            match node {
                NdNum(_) => NdNum(4),
                Nd(_) => NdNum(4),
            }

            Ok(NdSizeof())
        */
        } else {
            self.primary(lexer)
        }
    }

    // primary = num | ident | "(" expr ")" | ident ( "(" argument ")" )?
    // argument = (expr ( "," expr )* ) ?
    fn primary(&mut self,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
        if let Some(Num(n)) = self.take_num(lexer) {
            Ok(NdNum(n))
        } else if let Some(Id(name)) = self.take_id(lexer) {
            if self.choice(lexer,Delim(Lparen)) {
                let mut args = vec![];

                /* exprにマッチすることを先読みできないので、")"がないかどうかを選択肢にしている。 */
                if !self.choice(lexer,Delim(Rparen)) {
                    args.push(self.expr(lexer)?);
                    loop {
                        if self.choice(lexer,Delim(Comma)) {
                            args.push(self.expr(lexer)?);
                        } else {
                            break;
                        };
                    }
                    self.expect(lexer,Delim(Rparen))?;
                }

                Ok(NdCall(name.to_string(), args))
            } else {
                let result = self.symbol_table.get(&name).cloned();

                if let Some(lvar) = result {
                    Ok(NdLVar(lvar.0))
                } else {
                    Err(UndefinedSymbol(lexer.next().unwrap()))
                }
            }
        } else if self.choice(lexer,Delim(Lparen)) {
            let node = self.expr(lexer)?;
            self.expect(lexer,Delim(Rparen))?;
            Ok(node)
        } else {
            Err(UnexpectedToken(lexer.next().unwrap()))
        }
    }
}
