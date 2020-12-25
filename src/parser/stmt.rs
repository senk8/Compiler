


pub fn program<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Vector<Node>{
    let mut trees = vec![];

    loop {
        if consume(tokenizer,&TkMinus) {
            trees.push(stmt(tokenizer));
        }else{
            break trees;
        }
    }
}

pub fn stmt<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    let node = expr(tokenizer);
    expect(tokenizer,&TkSemicolon);
    node
}

pub fn expr<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    assign(tokenizer)
}

pub fn assign<'a>(tokenizer:&mut Peekable<Tokenizer<'a>>)->Node{
    let mut node = equality(tokenizer);
    if consume(tokenizer,) {
        node = NdAssign(node,assign(tokenizer));
    }
    node
}