use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::collections::{HashMap, BTreeMap};

fn main() {
    let input = &[
        "\n; My first program\nmov  a, 5\ninc  a\ncall function\nmsg  '(5+1)/2 = ', a    ; output message\nend\n\nfunction:\n    div  a, 2\n    ret\n",
        "\nmov   a, 5\nmov   b, a\nmov   c, a\ncall  proc_fact\ncall  print\nend\n\nproc_fact:\n    dec   b\n    mul   c, b\n    cmp   b, 1\n    jne   proc_fact\n    ret\n\nprint:\n    msg   a, '! = ', c ; output text\n    ret\n",
        "\nmov   a, 8            ; value\nmov   b, 0            ; next\nmov   c, 0            ; counter\nmov   d, 0            ; first\nmov   e, 1            ; second\ncall  proc_fib\ncall  print\nend\n\nproc_fib:\n    cmp   c, 2\n    jl    func_0\n    mov   b, d\n    add   b, e\n    mov   d, e\n    mov   e, b\n    inc   c\n    cmp   c, a\n    jle   proc_fib\n    ret\n\nfunc_0:\n    mov   b, c\n    inc   c\n    jmp   proc_fib\n\nprint:\n    msg   'Term ', a, ' of Fibonacci series is: ', b        ; output text\n    ret\n",
        "\nmov   a, 11           ; value1\nmov   b, 3            ; value2\ncall  mod_func\nmsg   'mod(', a, ', ', b, ') = ', d        ; output\nend\n\n; Mod function\nmod_func:\n    mov   c, a        ; temp1\n    div   c, b\n    mul   c, b\n    mov   d, a        ; temp2\n    sub   d, c\n    ret\n",
        "\nmov   a, 81         ; value1\nmov   b, 153        ; value2\ncall  init\ncall  proc_gcd\ncall  print\nend\n\nproc_gcd:\n    cmp   c, d\n    jne   loop\n    ret\n\nloop:\n    cmp   c, d\n    jg    a_bigger\n    jmp   b_bigger\n\na_bigger:\n    sub   c, d\n    jmp   proc_gcd\n\nb_bigger:\n    sub   d, c\n    jmp   proc_gcd\n\ninit:\n    cmp   a, 0\n    jl    a_abs\n    cmp   b, 0\n    jl    b_abs\n    mov   c, a            ; temp1\n    mov   d, b            ; temp2\n    ret\n\na_abs:\n    mul   a, -1\n    jmp   init\n\nb_abs:\n    mul   b, -1\n    jmp   init\n\nprint:\n    msg   'gcd(', a, ', ', b, ') = ', c\n    ret\n",
        "\ncall  func1\ncall  print\nend\n\nfunc1:\n    call  func2\n    ret\n\nfunc2:\n    ret\n\nprint:\n    msg 'This program should return null'\n",
        "\nmov   a, 2            ; value1\nmov   b, 10           ; value2\nmov   c, a            ; temp1\nmov   d, b            ; temp2\ncall  proc_func\ncall  print\nend\n\nproc_func:\n    cmp   d, 1\n    je    continue\n    mul   c, a\n    dec   d\n    call  proc_func\n\ncontinue:\n    ret\n\nprint:\n    msg a, '^', b, ' = ', c\n    ret\n",
    ];
    for &s in input {
        println!("{:?}", AssemblerInterpreter::interpret(s));
    }
}

type R = BTreeMap<char, String>;

type F = Box<dyn FnOnce(i64, i64) -> i64>;

#[derive(Debug)]
struct Line {
    name: String,
    args: Vec<String>,
}

impl Line {
    fn get_char(&self, i: usize) -> char {
        self.args[i].chars().next().unwrap()
    }
}

pub struct AssemblerInterpreter {
}

impl AssemblerInterpreter {
    pub fn interpret(input: &str) -> Option<String> {
        let lines: Vec<Line> = input
            .split('\n')
            .map(|s| s
                .chars()
                .take_while(|c| *c != ';')
                .collect::<String>()
                .trim()
                .to_string()
            )
            .filter(|s| !s.is_empty())
            .map(|line| {
                let command: String = line
                    .chars()
                    .take_while(|&c| matches!(c, '0'..='9' | 'a'..='z' | '_' | ':'))
                    .collect();
                if command.chars().last().unwrap() == ':' {
                    return Line {
                        name: "label".to_string(),
                        args: vec![command.chars().take_while(|&c| c != ':').collect()],
                    }
                }
                if command == String::from("msg") {
                    let mut a = String::new();
                    let mut q = false;
                    let mut l = Line {
                        name: command.clone(),
                        args: vec![],
                    };
                    for c in line.chars().skip(command.len()) {
                        match c {
                            '\'' => {
                                a.push(c);
                                q = !q;
                            },
                            'a'..='z' => {
                                a.push(c);
                            },
                            _ => {
                                if q {
                                    a.push(c);
                                } else if !a.is_empty() {
                                    l.args.push(a.clone());
                                    a.clear();
                                }
                            },
                        };
                    }
                    if !a.is_empty() {
                        l.args.push(a);
                    }
                    return l;
                }
                let mut arg = String::new();
                let mut l = Line {
                    name: command.clone(),
                    args: vec![],
                };
                for c in line.chars().skip(command.len()) {
                    match c {
                        '0'..='9' | 'a'..='z' | '_' => {
                            arg.push(c);
                        },
                        _ => {
                            if !arg.is_empty() {
                                l.args.push(arg.clone());
                                arg.clear();
                            }
                        },
                    };
                }
                if !arg.is_empty() {
                    l.args.push(arg);
                }
                l
            })
            .collect();
        let labels: HashMap<String, usize> = HashMap::from_iter(lines
            .iter()
            .enumerate()
            .filter(|p| p.1.name == "label")
            .map(|(i, line)| (line.args[0].clone(), i + 1))
        );
        let mut registers: R = BTreeMap::new();
        let mut stack: Vec<usize> = vec![0];
        let mut output = String::new();
        let mut cmp = Equal;
        while let Some(p) = stack.pop() {
            if p >= lines.len() {
                break
            }
            let line = &lines[p];
            let name = &line.name[..];
            match name {
                "end" => {
                    return Some(output);
                },
                "msg" => {
                    output = line.args
                        .iter()
                        .enumerate()
                        .map(|(i, a)| {
                            if line.get_char(i) != '\'' {
                                return get_reg_or_num(&registers, line, i)
                            }

                            a
                                .chars()
                                .skip(1)
                                .take_while(|c| *c != '\'')
                                .collect()
                        })
                        .collect();
                },
                "mov" => {
                    let y = get_reg_or_num(&registers, line, 1);
                    *registers.entry(line.get_char(0)).or_insert(y) = y.clone();
                },
                "inc" => {
                    let v = registers.get_mut(&line.get_char(0)).unwrap();
                    *v = (v.parse::<i64>().unwrap() + 1).to_string();
                },
                "dec" => {
                    let v = registers.get_mut(&line.get_char(0)).unwrap();
                    *v = (v.parse::<i64>().unwrap() - 1).to_string();
                },
                "add" => {
                    let result = bin_op(Box::new(|a, b| a + b), &registers, line);
                    *registers.entry(line.get_char(0)).or_insert(result) = result.clone();
                },
                "sub" => {
                    let result = bin_op(Box::new(|a, b| a - b), &registers, line);
                    *registers.entry(line.get_char(0)).or_insert(result) = result.clone();
                },
                "mul" => {
                    let result = bin_op(Box::new(|a, b| a * b), &registers, line);
                    *registers.entry(line.get_char(0)).or_insert(result) = result.clone();
                },
                "div" => {
                    let result = bin_op(Box::new(|a, b| a / b), &registers, line);
                    *registers.entry(line.get_char(0)).or_insert(result) = result.clone();
                },
                "cmp" => {
                    let x = get_reg_or_num(&registers, &line, 0).parse::<i64>().unwrap();
                    let y = get_reg_or_num(&registers, &line, 1).parse::<i64>().unwrap();
                    cmp = x.cmp(&y);
                },
                "jne" | "je" | "jge" | "jg" | "jle" | "jl" | "jmp" => {
                    if jmp_condition(cmp, name) {
                        stack.push(*labels.get(&line.args[0]).unwrap());
                    } else {
                        stack.push(p+1);
                    }
                },
                "call" => {
                    stack.push(p + 1);
                    stack.push(*labels.get(&line.args[0]).unwrap());
                },
                _ => {},
            };
            if !matches!(name, "jmp" | "jne" | "je" | "jge" | "jg" | "jle" | "jl" | "call" | "ret") {
                stack.push(p+1);
            }
        }
        None
    }
}

fn get_reg_or_num(r: &R, line: &Line, i: usize) -> String {
    let x = &line.args[i];
    let x_first = line.get_char(i);
    if x_first >= '0' && x_first <= '9' {
        x.clone()
    } else {
        r.get(&x_first).unwrap().clone()
    }
}

fn lift(f: F, a: String, b: String) -> String {
    f(a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()).to_string()
}

fn bin_op(f: F, r: &R, line: &Line) -> String {
    lift(
        f,
        get_reg_or_num(r, line, 0),
        get_reg_or_num(r, line, 1),
    )
}

fn jmp_condition(cmp: Ordering, cmd: &str) -> bool {
    match cmd {
        "jne" => cmp != Equal,
        "je" => cmp == Equal,
        "jge" => cmp > Less,
        "jg" => cmp == Greater,
        "jle" => cmp < Greater,
        "jl" => cmp == Less,
        _ => true,
    }
}
