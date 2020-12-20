use std::env;
use std::fs;

fn parse_impl(input: &str) -> (usize, usize) {
    let mut left_operand: usize = 0;
    let mut op: Option<char> = Some('+');
    let mut ci_iter = input.char_indices();

    while let Some((i, c)) = ci_iter.next() {
        match c {
            ' ' => (),
            '0'..='9' => {
                match op.unwrap() {
                    '+' => {
                        left_operand = left_operand + c.to_digit(10).unwrap() as usize;
                    }
                    '*' => {
                        left_operand = left_operand * c.to_digit(10).unwrap() as usize;
                    }
                    _ => {
                        panic!("unknown opcode: {}", op.unwrap());
                    }
                }
                op = None;
            }
            '+' | '*' => {
                if op.is_some() {
                    panic!("op is populated");
                }
                op = Some(c);
            }
            '(' => {
                // slice and recurse
                let (val, new_i) = parse_impl(&input[i + 1..]);
                ci_iter.nth(new_i);
                match op.unwrap() {
                    '+' => {
                        left_operand = left_operand + val;
                    }
                    '*' => {
                        left_operand = left_operand * val;
                    }
                    _ => {
                        panic!("unknown opcode: {}", op.unwrap());
                    }
                }
                op = None;
            }
            ')' => {
                return (left_operand, i);
            }

            _ => {
                panic!("encountered unknown char: {}", c);
            }
        };
    }

    (left_operand, input.len())
}

fn parse(input: &str) -> usize {
    let (val, _) = parse_impl(&input);
    println!("{}", val);
    val
}

#[derive(Debug, Clone, PartialEq)]
enum Sym {
    Op { op: char },
    Val { val: usize },
}

fn eval_flat_expr_p2(expr: &Vec<Sym>) -> usize {
    let add_count = expr
        .iter()
        .map(|x| match x {
            Sym::Op { op } => {
                if *op == '+' {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        })
        .sum::<usize>();

    let mut expr_so_far = expr.clone();

    for _ in 0..add_count {
        let (add_idx, _) = expr_so_far
            .iter()
            .enumerate()
            .find(|&(_, x)| *x == Sym::Op { op: '+' })
            .unwrap();

        let val0 = match expr_so_far[add_idx - 1] {
            Sym::Val { val } => Some(val),
            _ => None,
        }
        .unwrap();
        let val1 = match expr_so_far[add_idx + 1] {
            Sym::Val { val } => Some(val),
            _ => None,
        }
        .unwrap();

        let mut new_vec = expr_so_far.drain(0..add_idx - 1).collect::<Vec<_>>();

        new_vec.push(Sym::Val { val: val0 + val1 });
        new_vec.extend(expr_so_far.drain(3..).collect::<Vec<_>>());
        expr_so_far = new_vec;
    }

    let res = expr_so_far
        .iter()
        .filter_map(|x| match x {
            Sym::Val { val } => Some(val),
            _ => None,
        })
        .product::<usize>();
    res
}

fn parse_impl2_p2(input: &str) -> (Vec<Sym>, usize) {
    let mut ci_iter = input.char_indices();
    let mut sym_stack: Vec<Sym> = Vec::new();
    while let Some((i, c)) = ci_iter.next() {
        match c {
            ' ' => (),
            '0'..='9' => {
                sym_stack.push(Sym::Val {
                    val: c.to_digit(10).unwrap() as usize,
                });
            }
            '+' | '*' => {
                sym_stack.push(Sym::Op { op: c });
            }
            '(' => {
                // slice and recurse
                let (sub_expr, new_i) = parse_impl2_p2(&input[i + 1..]);
                let sub_expr_val = eval_flat_expr_p2(&sub_expr);
                sym_stack.push(Sym::Val { val: sub_expr_val });

                ci_iter.nth(new_i);
            }
            ')' => {
                return (sym_stack, i);
            }

            _ => {
                panic!("encountered unknown char: {}", c);
            }
        };
    }

    (sym_stack, input.len())
}

fn parse_p2(input: &str) -> usize {
    let (flat_expr, _) = parse_impl2_p2(&input);
    let val = eval_flat_expr_p2(&flat_expr);
    println!("{}", val);
    val
}

fn main() {
    println!("{}", parse_p2("1 + 2 * 3 + 4 * 5 + 6"));
    println!("{}", parse_p2("1 + (2 * 3) + (4 * (5 + 6))"));
    println!("{}", parse_p2("2 * 3 + (4 * 5)"));
    println!("{}", parse_p2("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    println!("{}", parse_p2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
    println!(
        "{}",
        parse_p2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
    );

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");

    println!(
        "p1: {}",
        file_contents
            .lines()
            .map(|l| {
                println!("=============");
                parse_p2(l) as usize
            })
            .sum::<usize>()
    );
}
