use super::Parser;

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

impl<'a> Parser<'a> {
    // program = decl *
    pub(super) fn program(&mut self) -> Result<Vec<Node>, ParseError> {
        let mut trees = vec![];

        while let Some(_) = self.look_ahead() {
            trees.push(self.decl()?);
            self.symbol_table.clear();
        }

        Ok(trees)
    }

    // decl = type ident ( ( type ident "," )* ) "{" stmt * "}"
    fn decl(&mut self) -> Result<Node, ParseError> {
        /* 引数コンパイルしたら同時にローカル変数の定義を行う。*/

        self.expect(Type(Int))?;

        if let Some(Id(name)) = self.take_id() {
            self.expect(Delim(Lparen))?;

            let mut args = vec![];
            if !self.choice(Delim(Rparen)) {
                loop {
                    /*
                    if let Some(Key(Int)) = self.take_type(){

                        let var = match self.take_id() {
                            Some(Id(name)) => name,
                            _ => panic!("unexpect!"),
                        };

                        self.set_var(var,);
                        args.push(NdLVar(self.offset));
                        if !self.choice(Delim(Comma)) {
                            self.expect(Delim(Rc))?;
                            break;
                        }
                    }
                    */

                    self.expect(Type(Int))?;

                    let ty = VarAnnot { ty: Int, ptr: None };

                    let var = match self.take_id() {
                        Some(Id(name)) => name,
                        _ => panic!("unexpect!"),
                    };

                    self.set_var(var, ty);
                    args.push(NdLVar(self.offset));
                    if !self.choice(Delim(Comma)) {
                        self.expect(Delim(Rparen))?;
                        break;
                    }
                }
            };

            self.expect(Delim(Lbrace))?;

            let mut nodes = Vec::new();

            while !self.choice(Delim(Rbrace)) {
                nodes.push(self.stmt()?);
            }

            Ok(NdDecl(name, args, Box::new(NdBlock(nodes))))
        } else {
            /* TODO: it will be unused look_ahead.unwrap() */ 
            Err(UnexpectedToken(self.look_ahead().unwrap()))
        }
    }

    /// stmt = expr ";"
    /// | "{" stmt* "}""
    /// | "return" expr ";"
    /// | "if" "(" expr ")" stmt ("else" stmt)?
    /// | "while" "(" expr ")" stmt
    /// | "for" "(" expr? ";" expr? ";" expr? ")" stmt
    fn stmt(&mut self) -> Result<Node, ParseError> {
        /* choice expr or return */

        if self.choice(Key(Return)) {
            let node = NdReturn(Box::new(self.expr()?));
            self.expect(Delim(Semicolon))?;
            Ok(node)
        } else if self.choice(Delim(Lbrace)) {
            let mut nodes = Vec::new();
            while !self.choice(Delim(Rbrace)) {
                nodes.push(self.stmt()?);
            }
            Ok(NdBlock(nodes))
        } else if self.choice(Key(If)) {
            self.expect(Delim(Lparen))?;
            let first = self.expr()?;
            self.expect(Delim(Rparen))?;
            let second = self.stmt()?;

            if self.choice(Key(Else)) {
                let third = self.stmt()?;
                Ok(NdIfElse(Box::new(first), Box::new(second), Box::new(third)))
            } else {
                Ok(NdIf(Box::new(first), Box::new(second)))
            }
        } else if self.choice(Key(While)) {
            self.expect(Delim(Lparen))?;
            let first = self.expr()?;
            self.expect(Delim(Rparen))?;
            let second = self.stmt()?;
            Ok(NdWhile(Box::new(first), Box::new(second)))
        } else if self.choice(Key(For)) {
            self.expect(Delim(Lparen))?;
            let first = self.expr()?;
            self.expect(Delim(Semicolon))?;
            let second = self.expr()?;
            self.expect(Delim(Semicolon))?;
            let third = self.expr()?;
            self.expect(Delim(Rparen))?;
            let fourth = self.stmt()?;
            Ok(NdFor(
                Box::new(first),
                Box::new(second),
                Box::new(third),
                Box::new(fourth),
            ))
        } else {
            let node = self.expr()?;
            self.expect(Delim(Semicolon))?;
            Ok(node)
        }
    }
}

impl<'a> Parser<'a> {
    //expr = assign | typeident ident
    //typeident = type '*' *

    /*
    pub(super) fn typeident(&mut self) -> Result<Node,ParseError>{
        if let self.take_type(){

        }

        let mut ty = Type {
            ty: TypeKind::Int,
            ptr: None,
        };

        while self.choice(Opr(Star)){
            ty = Type {
                ty: TypeKind::Pointer,
                ptr:Some(Box::new(ty))
            };
        }

        ty
    }
    */

    fn expr(&mut self) -> Result<Node, ParseError> {
        if self.choice(Type(Int)) {
            let token = self.take_token().ok_or(Eof)?;

            let mut ty = VarAnnot { ty: Int, ptr: None };

            while self.choice(Opr(Star)) {
                ty = VarAnnot {
                    ty: Pointer,
                    ptr: Some(Box::new(ty)),
                };
            }

            if let (Id(name), _) = token {
                self.set_var(name, ty);
                Ok(NdVdecl(self.offset))
            } else {
                Err(UnexpectedToken(token))
            }
        } else {
            self.assign()
        }
    }

    //assign = equality ( "=" assign )?
    fn assign(&mut self) -> Result<Node, ParseError> {
        let node = self.equality()?;

        if self.choice(Opr(Assign)) {
            Ok(NdAssign(Box::new(node), Box::new(self.assign()?)))
        } else {
            Ok(node)
        }
    }

    // equality = relational ("==" relational | "!=" relational)*
    fn equality(&mut self) -> Result<Node, ParseError> {
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
    fn relational(&mut self) -> Result<Node, ParseError> {
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
    fn add(&mut self) -> Result<Node, ParseError> {
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
    fn mul(&mut self) -> Result<Node, ParseError> {
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
    fn unary(&mut self) -> Result<Node, ParseError> {
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
    fn primary(&mut self) -> Result<Node, ParseError> {
        if let Some(Num(n)) = self.take_num() {
            Ok(NdNum(n))
        } else if let Some(Id(name)) = self.take_id() {
            if self.choice(Delim(Lparen)) {
                let mut args = vec![];

                /* exprにマッチすることを先読みできないので、")"がないかどうかを選択肢にしている。 */
                if !self.choice(Delim(Rparen)) {
                    args.push(self.expr()?);
                    loop {
                        if self.choice(Delim(Comma)) {
                            args.push(self.expr()?);
                        } else {
                            break;
                        };
                    }
                    self.expect(Delim(Rparen))?;
                }

                Ok(NdCall(name.to_string(), args))
            } else {
                let result = self.symbol_table.get(&name).cloned();

                if let Some(lvar) = result {
                    Ok(NdLVar(lvar.0))
                } else {
                    Err(UndefinedSymbol(self.lexer.next().unwrap()))
                }
            }
        } else if self.choice(Delim(Lparen)) {
            let node = self.expr()?;
            self.expect(Delim(Rparen))?;
            Ok(node)
        } else {
            Err(UnexpectedToken(self.lexer.next().unwrap()))
        }
    }
}
