use std::iter::Peekable;

use crate::Parser;
use crate::Lexer;

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



// decl = type ident ( ( type ident "," )* ) "{" stmt * "}"
pub(super) fn decl(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
    /* 引数コンパイルしたら同時にローカル変数の定義を行う。*/

    let _ = parser.take_type(lexer).ok_or_else(||
        UnexpectedToken(lexer.next().unwrap())
    )?;

    if let Some(Id(name)) = parser.take_id(lexer) {
        parser.expect(lexer,Delim(Lparen))?;

        let mut args = vec![];
        if !parser.choice(lexer,Delim(Rparen)) {
            loop {

                if let Some(ty) = parser.take_type(lexer){

                    let token = parser.take_token(lexer).ok_or(Eof)?;

                    if let (Id(name), _) = token {
                        parser.set_var(name, ty);
                        args.push(NdLVar(parser.offset));

                        if !parser.choice(lexer,Delim(Comma)) {
                            parser.expect(lexer,Delim(Rparen))?;
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

        parser.expect(lexer,Delim(Lbrace))?;

        let mut nodes = Vec::new();

        while !parser.choice(lexer,Delim(Rbrace)) {
            nodes.push(stmt(parser,lexer)?);
        }

        Ok(NdDecl(name, args, Box::new(NdBlock(nodes))))
    } else {
        /* TODO: it will be unused look_ahead.unwrap() */ 
        Err(UnexpectedToken(parser.look_ahead(lexer).unwrap()))
    }
}



//expr = assign | type ident
fn expr(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
    if let Some(ty) = parser.take_type(lexer){
        let token = parser.take_token(lexer).ok_or(Eof)?;

        if let (Id(name), _) = token {
            parser.set_var(name, ty);
            Ok(NdVdecl(parser.offset))
        } else {
            Err(UnexpectedToken(token))
        }
    } else {
        assign(parser,lexer)
    }
}

//assign = equality ( "=" assign )?
fn assign(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
    let node = equality(parser,lexer)?;

    if parser.choice(lexer,Opr(Assign)) {
        Ok(NdAssign(Box::new(node), Box::new(assign(parser,lexer)?)))
    } else {
        Ok(node)
    }
}

// equality = relational ("==" relational | "!=" relational)*
fn equality(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
    let mut node = relational(parser,lexer)?;

    loop {
        if parser.choice(lexer,Opr(Eq)) {
            node = NdEq(Box::new(node), Box::new(relational(parser,lexer)?));
        } else if parser.choice(lexer,Opr(Neq)) {
            node = NdNeq(Box::new(node), Box::new(relational(parser,lexer)?));
        } else {
            break Ok(node);
        }
    }
}

//relational = add ("<" add | "<=" add | ">" add| ">=" add) *
fn relational(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
    let mut node = add(parser,lexer)?;

    loop {
        if parser.choice(lexer,Opr(Lt)) {
            node = NdLt(Box::new(node), Box::new(add(parser,lexer)?));
        } else if parser.choice(lexer,Opr(Leq)) {
            node = NdLeq(Box::new(node), Box::new(add(parser,lexer)?));
        } else if parser.choice(lexer,Opr(Gt)) {
            node = NdLt(Box::new(add(parser,lexer)?), Box::new(node));
        } else if parser.choice(lexer,Opr(Geq)) {
            node = NdLeq(Box::new(add(parser,lexer)?), Box::new(node));
        } else {
            break Ok(node);
        }
    }
}

// add    = mul ("+" mul | "-" mul)*
fn add(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
    let mut node = mul(parser,lexer)?;

    loop {
        if parser.choice(lexer,Opr(Add)) {
            node = NdAdd(Box::new(node), Box::new(mul(parser,lexer)?));
        } else if parser.choice(lexer,Opr(Sub)) {
            node = NdSub(Box::new(node), Box::new(mul(parser,lexer)?));
        } else {
            break Ok(node);
        }
    }
}

// mul     = unary ("*" unary | "/" unary)*
fn mul(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
    let mut node = unary(parser,lexer)?;

    loop {
        if parser.choice(lexer,Opr(Star)) {
            node = NdMul(Box::new(node), Box::new(unary(parser,lexer)?));
        } else if parser.choice(lexer,Opr(Div)) {
            node = NdDiv(Box::new(node), Box::new(unary(parser,lexer)?));
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
fn unary(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
    if parser.choice(lexer,Opr(Add)) {
        primary(parser,lexer)
    } else if parser.choice(lexer,Opr(Sub)) {
        Ok(NdSub(Box::new(NdNum(0)), Box::new(primary(parser,lexer)?)))
    } else if parser.choice(lexer,Opr(Star)) {
        Ok(NdDeref(Box::new(primary(parser,lexer)?)))
    } else if parser.choice(lexer,Opr(Amp)) {
        Ok(NdRef(Box::new(primary(parser,lexer)?)))
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
        primary(parser,lexer)
    }
}

// primary = num | ident | "(" expr ")" | ident ( "(" argument ")" )?
// argument = (expr ( "," expr )* ) ?
fn primary(parser:&mut Parser,lexer:&mut Peekable<Lexer>) -> Result<Node, ParseError> {
    if let Some(Num(n)) = parser.take_num(lexer) {
        Ok(NdNum(n))
    } else if let Some(Id(name)) = parser.take_id(lexer) {
        if parser.choice(lexer,Delim(Lparen)) {
            let mut args = vec![];

            /* exprにマッチすることを先読みできないので、")"がないかどうかを選択肢にしている。 */
            if !parser.choice(lexer,Delim(Rparen)) {
                args.push(expr(parser,lexer)?);
                loop {
                    if parser.choice(lexer,Delim(Comma)) {
                        args.push(expr(parser,lexer)?);
                    } else {
                        break;
                    };
                }
                parser.expect(lexer,Delim(Rparen))?;
            }

            Ok(NdCall(name.to_string(), args))
        } else {
            let result = parser.symbol_table.get(&name).cloned();

            if let Some(lvar) = result {
                Ok(NdLVar(lvar.0))
            } else {
                Err(UndefinedSymbol(lexer.next().unwrap()))
            }
        }
    } else if parser.choice(lexer,Delim(Lparen)) {
        let node = expr(parser,lexer)?;
        parser.expect(lexer,Delim(Rparen))?;
        Ok(node)
    } else {
        Err(UnexpectedToken(lexer.next().unwrap()))
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::types::node::Node::*;
    use anyhow::Result;

    #[allow(unused_macros)]
    macro_rules! node {
        ($f:ident,$lhs:expr,$rhs:expr) => {
            $f(Box::new($lhs), Box::new($rhs))
        };
        ($f:ident,$lhs:expr) => {
            $f(Box::new($lhs))
        };
    }

    #[test]
    fn test_parse_arithmetic() -> Result<()> {
        let input = "2+1;";
        let mut lexer = Lexer::new(input.as_bytes()).peekable();
        let mut parser = Parser::new();

        let result = expr(&mut parser,&mut lexer)?;

        dbg!(&result);

        assert_eq!(result, 
            node!(NdAdd,NdNum(2),NdNum(1))
        );

        Ok(())
    }


}