use pratt_parser::{Lexer, Token, S};

pub mod pratt_parser;

fn expr(input: &str) -> S {
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer)
}

fn expr_bp(lexer: &mut Lexer) -> S {
    let token = lexer.next();
    println!("token: {:?}", token);
    let lhs = match token {
        Token::Atom(it) => S::Atom(it),
        t => panic!("unrecognized token: {:?}", t),
    };

    loop {
        let op = match lexer.next() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };

        let (left, right) = infix_binding(op);
        todo!()
    }
    
    lhs

    /*return format
    S::Cons(
        'a',
        vec![
            S::Atom('b'),
            S::Cons('c', vec![S::Atom('d'), S::Atom('e')]),
        ],
    )
    */
}

fn infix_binding(op: char) -> (u8, u8) {
    match op {
        '+' | '-' => (1,2),
        '*' | '/' => (3,4),
        _ => panic!("bad op: {:?}", op),
    }
}
fn main() {
   let s = expr("1+2*3");

   println!("{}", s);
}
