use std::intrinsics::unreachable;
use crate::B::{Div, Mul, Plus, Pow};

fn main() {
    println!("{}", diff("5"));
}

#[derive(Debug)]
enum L {X, N(i64)}

#[derive(Debug, Copy)]
enum U {Exp, Ln, Sin, Cos, Tan}

#[derive(Debug, Copy, Eq)]
enum B {Plus, Minus, Mul, Div, Pow}

#[derive(Debug, Clone)]
enum T {
    L,
    NU(U, Box<T>),
    NB(B, Box<T>, Box<T>),
}

impl U {
    fn from_char(c: char) -> Self {
        match c {
            'e' => U::Exp,
            'l' => U::Ln,
            's' => U::Sin,
            'c' => U::Cos,
            _ => U::Tan,
        }
    }

    fn to_string(&self) -> String {
        (match self {
            U::Exp => "exp",
            U::Ln => "ln",
            U::Sin => "sin",
            U::Cos => "cos",
            _ => "tan",
        }).to_string()
    }
}

impl B {
    fn from_char(c: char) -> Self {
        match c {
            '+' => B::Plus,
            '-' => B::Minus,
            '*' => B::Mul,
            '/' => B::Div,
            _ => B::Pow,
        }
    }

    fn to_string(&self) -> String {
        (match self {
            B::Plus => "+",
            B::Minus => "-",
            B::Mul => "*",
            B::Div => "/",
            _ => "^",
        }).to_string()
    }

    fn calc(&self, a: i64, b: i64) -> i64 {
        match self {
            B::Plus => a + b,
            B::Minus => a - b,
            B::Mul => a * b,
            B::Div => a / b,
            _ => a.pow(b as u32),
        }
    }
}

impl T {
    fn from_str(expr: &str) -> Self {
        parse(expr.chars().collect()).1
    }

    fn do_diff(&self) -> Self {
        match self {
            T::L::N => T::L::N(0),
            T::L::X => T::L::N(1),
            T::NU(U::Exp, l) => T::NB(B::Mul, l.clone(), Box::new(l.do_diff())),
            T::NU(U::Ln, l) => T::NB(B::Div, Box::new(l.do_diff()), l.clone()),
            T::NU(U::Sin, l) => T::NB(
                B::Mul,
                Box::new(T::NU(U::Cos, l.clone())),
                Box::new(l.do_diff()),
            ),
            T::NU(U::Cos, l) => T::NB(
                B::Mul,
                T::L::N(-1),
                Box::new(T::NB(
                    B::Mul,
                    Box::new(T::NU(U::Sin, l.clone())),
                    Box::new(l.do_diff()),
                )),
            ),
            T::NU(U::Tan, l) => T::NB(
                B::Div,
                Box::new(l.do_diff()),
                Box::new(T::NB(
                    B::Pow,
                    Box::new(T::NU(
                        U::Cos,
                        l.clone(),
                    )),
                    Box::new(T::L::N(2)),
                )),
            ),
            T::NB(B::Plus, l, r) => T::NB(
                B::Plus,
                Box::new(l.do_diff()),
                Box::new(r.do_diff()),
            ),
            T::NB(B::Minus, l, r) => T::NB(
                B::Minus,
                Box::new(l.do_diff()),
                Box::new(r.do_diff()),
            ),
            T::NB(B::Mul, l, r) => T::NB(
                B::Plus,
                Box::new(T::NB(
                    B::Mul,
                    Box::new(l.do_diff()),
                    r.clone(),
                )),
                Box::new(T::NB(
                    B::Mul,
                    Box::new(r.do_diff()),
                    l.clone(),
                )),
            ),
            T::NB(B::Div, l, r) => T::NB(
                B::Div,
                Box::new(T::NB(
                    B::Minus,
                    Box::new(T::NB(
                        B::Mul,
                        Box::new(l.do_diff()),
                        r.clone(),
                    )),
                    Box::new(T::NB(
                        B::Mul,
                        Box::new(r.do_diff()),
                        l.clone(),
                    )),
                )),
                Box::new(T::NB(
                    B::Pow,
                    r.clone(),
                    T::L::N(2),
                )),
            ),
            T::NB(B::Pow, l, r) => T::NB(
                B::Mul,
                Box::new(T::NB(
                B::Mul,
                r.clone(),
                Box::new(T::NB(
                    B::Pow,
                    l.clone(),
                    Box::new(T::NB(
                        B::Minus,
                        r.clone(),
                        Box::new(T::L::N(1)),
                        )),
                    )),
                )),
                Box::new(l.do_diff()),
            ),
        }
    }

    fn simplify(&self) -> Self {
        match self {
            T::L::X => T::L::X,
            T::L::N(n) => T::L::N(n),
            T::NU(op, l) => T::NU(*op, Box::new(l.simplify())),
            T::NB(op, l, r) => {
                let l_s = l.simplify();
                let r_s = r.simplify();
                let l_is_num = matches!(l_s, T::L::N);
                let r_is_num = matches!(r_s, T::L::N);
                if l_is_num && r_is_num {
                    T::L::N(op.calc(l_s.get_value(), r_s.get_value()))
                } else if (l_is_num && l_s.get_value() == 0 || r_is_num && r_s.get_value() == 0) && *op == B::Mul {
                    T::L::N(0)
                } else if l_is_num && l_s.get_value() == 1 && *op == B::Mul {
                    r_s
                } else if r_is_num && r_s.get_value() == 1 && *op == B::Mul {
                    l_s
                } else if r_is_num && r_s.get_value() == 0 && *op == B::Pow {
                    T::L::N(1)
                } else if r_is_num && r_s.get_value() == 1 && *op == B::Pow {
                    l_s
                } else {
                    T::NB(*op, Box::new(l_s), Box::new(r_s))
                }
            },
        }
    }

    fn to_string(&self) -> String {
        self.do_to_string(false)
    }

    fn do_to_string(&self, parens: bool) -> String {
        match self {
            T::L::X => "x".to_string(),
            T::L::N(n) => n.to_string(),
            _ => {
                let inner = match self {
                    T::NU(op, l) => format!(
                        "{} {}",
                        op.to_string(),
                        l.do_to_string(true),
                    ),
                    T::NB(op, l, r) => format!(
                        "{} {} {}",
                        op.to_string(),
                        l.do_to_string(true),
                        r.do_to_string(true),
                    ),
                    _ => unreachable!(),
                };
                if parens {
                    format!("({inner})")
                } else {
                    inner
                }
            },
        }
    }

    fn get_value(&self) -> i64 {
        match self {
            T::L::N(n) => n,
            _ => unreachable!(),
        }
    }
}

fn diff(expr: &str) -> String {
    T::from_str(exp)
        .do_diff()
        .simplify()
        .to_string()
    // println!("{:?}", ast);
}

fn parse(input: &[char]) -> (usize, T) {
    match input[0] {
        '0'..='9' => parse_num(input),
        '+' | '*' | '/' | '^' => parse_b(input),
        '-' => (if matches(input[1], '0'..='9') {parse_num} else {parse_b})(input),
        'e' | 'l' | 's' | 'c' | 't' => parse_u(input),
        ' ' | '(' | ')' => parse_next(input),
        _ => (1, T::L::X),
    }
}

fn parse_num(input: &[char]) -> (usize, T) {
    let s: String = input
        .iter()
        .take_while(|&c| matches!(*c, '-' | '0'..='9'))
        .collect();
    (s.len(), T::L::N(s.parse::<i64>().unwrap()))
}

fn parse_b(input: &[char]) -> (usize, T) {
    let (p, l) = parse(&input[2..]);
    let (pp, r) = parse(&input[2+p..]);
    let op_type = B::from_char(input[0]);
    (2+p+pp, T::NB(op_type, Box::new(l), Box::new(r)))
}

fn parse_u(input: &[char]) -> (usize, T) {
    let op = U::from_char(input[0]);
    let skip = op.to_string().len();
    let (p, l) = parse(&input[skip..]);
    (skip+p, T::NU(op, Box::new(l)))
}

fn parse_next(input: &[char]) -> (usize, T) {
    let (p, ast) = parse(&input[1..]);
    (p+1, ast)
}
