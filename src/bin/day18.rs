use advtools::input;
use advtools::itertools::{put_back, PutBack};

#[derive(Debug)]
enum Expr {
    Invalid,
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn parse_atom(tok: &mut PutBack<impl Iterator<Item=char>>, prec: bool) -> Expr {
    let mut numbuf = String::new();
    while let Some(ch) = tok.next() {
        match ch {
            '0'..='9' => numbuf.push(ch),
            '(' => return parse_expr(tok, prec),
            '+' | '*' | ')' => { tok.put_back(ch); break; }
            ' ' => {}
            _ => unimplemented!("{}", ch)
        }
    }
    Expr::Num(numbuf.parse().expect("invalid number"))
}

fn parse_add(tok: &mut PutBack<impl Iterator<Item=char>>) -> Expr {
    let mut lhs = Expr::Invalid;
    while let Some(ch) = tok.next() {
        match ch {
            '+' => lhs = Expr::Add(Box::new(lhs), Box::new(parse_atom(tok, true))),
            ')' | '*' => { tok.put_back(ch); break; }
            ' ' => {}
            _ => { tok.put_back(ch); lhs = parse_atom(tok, true); }
        }
    }
    lhs
}

fn parse_expr(tok: &mut PutBack<impl Iterator<Item=char>>, prec: bool) -> Expr {
    let parse_inner = |tok: &mut _| if prec { parse_add(tok) } else { parse_atom(tok, prec) };
    let mut lhs = Expr::Invalid;
    while let Some(ch) = tok.next() {
        match ch {
            '*' => lhs = Expr::Mul(Box::new(lhs), Box::new(parse_inner(tok))),
            '+' if !prec => lhs = Expr::Add(Box::new(lhs), Box::new(parse_inner(tok))),
            ')' => break,
            ' ' => {},
            _ => { tok.put_back(ch); lhs = parse_inner(tok); }
        }
    }
    lhs
}

fn eval(expr: Expr) -> i64 {
    match expr {
        Expr::Num(i) => i,
        Expr::Add(a, b) => eval(*a) + eval(*b),
        Expr::Mul(a, b) => eval(*a) * eval(*b),
        Expr::Invalid => panic!("invalid expression")
    }
}

fn main() {
    let sum: i64 = input::lines()
        .map(|line| eval(parse_expr(&mut put_back(line.chars()), false)))
        .sum();
    advtools::verify("No precedence", sum, 3885386961962i64);

    let sum: i64 = input::lines()
        .map(|line| eval(parse_expr(&mut put_back(line.chars()), true)))
        .sum();
    advtools::verify("+ before *", sum, 112899558798666i64);
}
