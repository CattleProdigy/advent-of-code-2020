use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
struct ProgramState {
    addr_mask: usize,
    float_mask: usize,
    memory: HashMap<usize, usize>,
}

#[derive(Debug, Clone)]
enum Instruction {
    Mask { addr_mask: usize, float_mask: usize },
    Assignment { addr: usize, value: usize },
}

fn run_p2(insts: &Vec<Instruction>) -> usize {
    let mut writes: Vec<(usize, usize, usize)> = Vec::new();
    let mut ps = ProgramState {
        addr_mask: 0,
        float_mask: 0,
        memory: HashMap::new(),
    };

    for i in insts.iter() {
        match i {
            Instruction::Mask {
                addr_mask,
                float_mask,
            } => {
                ps.addr_mask = *addr_mask;
                ps.float_mask = *float_mask;
            }
            Instruction::Assignment { addr, value } => {
                writes.push((
                    *value,
                    ps.float_mask,
                    (addr & !ps.float_mask) | ps.addr_mask,
                ));
            }
        }
    }

    for (v, f, addr) in writes.iter() {
        let mut floating_bits: Vec<usize> = Vec::new();
        {
            let mut f_shift = *f;
            for i in 0..64 {
                if f_shift == 0 {
                    break;
                }
                if f_shift & 1 == 1 {
                    floating_bits.push(i);
                }

                f_shift = f_shift >> 1;
            }
        }

        let num_perms = 1 << floating_bits.len();
        for i in 0..num_perms {
            let mut set_bits: Vec<usize> = Vec::new();
            let mut clear_bits: Vec<usize> = Vec::new();
            {
                let mut i_shift = i;
                for i in 0..floating_bits.len() {
                    if i_shift & 1 == 1 {
                        set_bits.push(i);
                    } else if i_shift & 1 == 0 {
                        clear_bits.push(i);
                    }

                    i_shift = i_shift >> 1;
                }
            }
            let mut addr_mut = *addr;
            for s in set_bits {
                addr_mut |= 1 << floating_bits[s];
            }
            for c in clear_bits {
                addr_mut &= !(1 << floating_bits[c]);
            }
            ps.memory.insert(addr_mut, *v);
        }
    }

    let mut mem_sum = 0;
    for (_, v) in ps.memory.iter() {
        mem_sum += v;
    }

    mem_sum
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
                let float_mask_str = mask_slice.replace('1', "0").replace('X', "1");
                let float_mask = usize::from_str_radix(&float_mask_str, 2).unwrap();
                let addr_mask_str = mask_slice.replace('X', "0");
                let addr_mask = usize::from_str_radix(&addr_mask_str, 2).unwrap();
                Instruction::Mask {
                    addr_mask: addr_mask,
                    float_mask: float_mask,
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

    let mem_sum_p2 = run_p2(&insts);
    println!("p2: {}", mem_sum_p2);
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
