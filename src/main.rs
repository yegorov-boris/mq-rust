use std::collections::{HashMap, BTreeMap};

fn main() {
    let input = &[
        "\n; My first program\nmov  a, 5\ninc  a\ncall function\nmsg  '(5+1)/2 = ', a    ; output message\nend\n\nfunction:\n    div  a, 2\n    ret\n",
        // "\nmov   a, 5\nmov   b, a\nmov   c, a\ncall  proc_fact\ncall  print\nend\n\nproc_fact:\n    dec   b\n    mul   c, b\n    cmp   b, 1\n    jne   proc_fact\n    ret\n\nprint:\n    msg   a, '! = ', c ; output text\n    ret\n",
        // "\nmov   a, 8            ; value\nmov   b, 0            ; next\nmov   c, 0            ; counter\nmov   d, 0            ; first\nmov   e, 1            ; second\ncall  proc_fib\ncall  print\nend\n\nproc_fib:\n    cmp   c, 2\n    jl    func_0\n    mov   b, d\n    add   b, e\n    mov   d, e\n    mov   e, b\n    inc   c\n    cmp   c, a\n    jle   proc_fib\n    ret\n\nfunc_0:\n    mov   b, c\n    inc   c\n    jmp   proc_fib\n\nprint:\n    msg   'Term ', a, ' of Fibonacci series is: ', b        ; output text\n    ret\n",
        // "\nmov   a, 11           ; value1\nmov   b, 3            ; value2\ncall  mod_func\nmsg   'mod(', a, ', ', b, ') = ', d        ; output\nend\n\n; Mod function\nmod_func:\n    mov   c, a        ; temp1\n    div   c, b\n    mul   c, b\n    mov   d, a        ; temp2\n    sub   d, c\n    ret\n",
        // "\nmov   a, 81         ; value1\nmov   b, 153        ; value2\ncall  init\ncall  proc_gcd\ncall  print\nend\n\nproc_gcd:\n    cmp   c, d\n    jne   loop\n    ret\n\nloop:\n    cmp   c, d\n    jg    a_bigger\n    jmp   b_bigger\n\na_bigger:\n    sub   c, d\n    jmp   proc_gcd\n\nb_bigger:\n    sub   d, c\n    jmp   proc_gcd\n\ninit:\n    cmp   a, 0\n    jl    a_abs\n    cmp   b, 0\n    jl    b_abs\n    mov   c, a            ; temp1\n    mov   d, b            ; temp2\n    ret\n\na_abs:\n    mul   a, -1\n    jmp   init\n\nb_abs:\n    mul   b, -1\n    jmp   init\n\nprint:\n    msg   'gcd(', a, ', ', b, ') = ', c\n    ret\n",
        // "\ncall  func1\ncall  print\nend\n\nfunc1:\n    call  func2\n    ret\n\nfunc2:\n    ret\n\nprint:\n    msg 'This program should return null'\n",
        // "\nmov   a, 2            ; value1\nmov   b, 10           ; value2\nmov   c, a            ; temp1\nmov   d, b            ; temp2\ncall  proc_func\ncall  print\nend\n\nproc_func:\n    cmp   d, 1\n    je    continue\n    mul   c, a\n    dec   d\n    call  proc_func\n\ncontinue:\n    ret\n\nprint:\n    msg a, '^', b, ' = ', c\n    ret\n",
    ];
    for &s in input {
        println!("{:?}", AssemblerInterpreter::interpret(s));
    }
}

struct Line {
    name: String,
    args: Vec<String>,
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
            )
            .filter(|s| !s.is_empty())
            .map(|line| {
                let command: &str = line
                    .chars()
                    .take_while(|&c| c >= 'a' && c <= 'z' || c == ':')
                    .collect();
                if command.chars().last().unwrap() == ':' {
                    return Line {
                        name: "label".to_string(),
                        args: vec![],
                    }
                }
                if command == "msg" {
                    let mut a = String::new();
                    let mut a_s = String::new();
                    let mut l = Line {
                        name: command.to_string(),
                        args: vec![],
                    };
                    for c in line.chars().skip(command.len()) {
                        match c {
                            '\'' => {
                                a_s.push(c);
                            },
                            'a'..='z' | '0'..='9' => {
                                if a_s.is_empty() {
                                    a_s.push(c);
                                } else {
                                    a.push(c);
                                }
                            },
                            _ => {
                                if !a_s.is_empty() {
                                    l.args.push(a_s.clone());
                                    a_s.clear();
                                }
                                if !a.is_empty() {
                                    l.args.push(a.clone());
                                    a.clear();
                                }
                            },
                        };
                    }
                    return l;
                }
                let mut arg = String::new();
                let mut l = Line {
                    name: command.to_string(),
                    args: vec![],
                };
                for c in line.chars().skip(command.len()) {
                    match c {
                        '0'..='9' | 'a'..='z' => {
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
                l
            })
            .collect();
        let labels: HashMap<String, usize> = HashMap::from_iter(lines
            .iter()
            .enumerate()
            .filter(|p| p.1.name == "label")
            .map(|(i, line)| (i, line.name.clone()))
        );
        let mut registers: BTreeMap<char, String> = BTreeMap::new();
        let mut stack: Vec<usize> = vec![0];
        let mut output = String::new();
        let mut cmp_x: i64 = 0;
        let mut cmp_y: i64 = 0;
        while let Some(p) = stack.pop() {
            let line = &lines[p];
            let name = &line.name[..];
            match name {
                "end" => {
                    return Some(output);
                },
                "msg" => {
                    output = line.args
                        .iter()
                        .map(|a| {
                            let r = a.chars()[0];
                            if r == '\'' {
                                return a.clone();
                            }
                            return registers
                                .get(r)
                                .unwrap()
                                .clone();
                        })
                        .collect();
                },
                "mov" => {
                    let x = &line.args[0];
                    let x_first = x.chars().next().unwrap();
                    let y = line.args[1].chars().next().unwrap();
                    let v_x = registers.get(&x_first);
                    let mut v = registers.get_mut(&y).unwrap();
                    *v = if x_first >= '0' && x_first <= '9' {
                        x.clone()
                    } else {
                        v_x.unwrap().clone()
                    };
                },
                "inc" => {
                    let x = line.args[0].chars().next().unwrap();
                    let mut v = registers.get_mut(&x).unwrap();
                    *v = (v.parse::<i64>().unwrap() + 1).to_string();
                },
                "dec" => {
                    let x = line.args[0].chars().next().unwrap();
                    let mut v = registers.get_mut(&x).unwrap();
                    *v = (v.parse::<i64>().unwrap() - 1).to_string();
                },
                "add" => {

                },
                "sub" => {

                },
                "mul" => {

                },
                "div" => {

                },
                "cmp" => {

                },
                "jne" => {

                },
                "je" => {

                },
                "jge" => {

                },
                "jg" => {

                },
                "jle" => {

                },
                "jl" => {

                },
                "call" | "jmp" => {
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
