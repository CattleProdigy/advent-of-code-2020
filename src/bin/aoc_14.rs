use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
struct ProgramState {
    old_mask: usize,
    new_mask: usize,
    memory: HashMap<usize, usize>,
}

#[derive(Debug, Clone)]
enum Instruction {
    Mask { old_mask: usize, new_mask: usize },
    Assignment { addr: usize, value: usize },
}

fn run(insts: &Vec<Instruction>) -> ProgramState {
    let mut ps = ProgramState {
        old_mask: 0,
        new_mask: 0,
        memory: HashMap::new(),
    };

    println!("{:?}", ps);
    for i in insts.iter() {
        match i {
            Instruction::Mask { old_mask, new_mask } => {
                ps.old_mask = *old_mask;
                ps.new_mask = *new_mask;
            }
            Instruction::Assignment { addr, value } => {
                let mem_val = ps.memory.entry(*addr).or_insert(0);
                let new_val = (ps.old_mask & value) | (ps.new_mask);
                *mem_val = new_val;
            }
        }
        println!("{:?}", ps);
    }

    ps
}

fn parse(input_str: &str) -> Vec<Instruction> {
    input_str
        .lines()
        .map(|l| {
            if l.starts_with("mem") {
                let open_brace = l.find('[').unwrap();
                let close_brace = l.find(']').unwrap();
                let addr_slice = &l[open_brace + 1..close_brace];
                let addr = addr_slice.parse::<usize>().unwrap();

                let equals_sign = l.find('=').unwrap();
                let val_slice = &l[equals_sign + 2..];
                let val = val_slice.parse::<usize>().unwrap();
                Instruction::Assignment {
                    addr: addr,
                    value: val,
                }
            } else if l.starts_with("mask") {
                let equals_sign = l.find('=').unwrap();
                let mask_slice = &l[equals_sign + 2..];
                let old_mask_str = mask_slice.replace('X', "1");
                let old_mask = usize::from_str_radix(&old_mask_str, 2).unwrap();
                let new_mask_str = mask_slice.replace('X', "0");
                let new_mask = usize::from_str_radix(&new_mask_str, 2).unwrap();
                Instruction::Mask {
                    old_mask: old_mask,
                    new_mask: new_mask,
                }
            } else {
                panic!("Unknown opcode: {}", l);
            }
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");
    let insts = parse(&file_contents);
    let end_ps = run(&insts);

    let mem_sum_p1 = end_ps.memory.values().sum::<usize>();
    println!("p1: {}", mem_sum_p1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example1 = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;

        let insts = parse(&example1);
        println!("{:?}", insts);
        run(&insts);
        assert_eq!(0, 1);
    }
}
