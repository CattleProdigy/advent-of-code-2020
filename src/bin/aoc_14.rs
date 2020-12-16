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
// fn run_p2(insts: &Vec<Instruction>) -> ProgramState {
//     let mut ps = ProgramState {
//         old_mask: 0,
//         new_mask: 0,
//         memory: HashMap::new(),
//     };
//
//     println!("{:?}", ps);
//     for i in insts.iter() {
//         match i {
//             Instruction::Mask { old_mask, new_mask } => {
//                 ps.old_mask = *old_mask;
//                 ps.new_mask = *new_mask;
//             }
//             Instruction::Assignment { addr, value } => {
//                 let mut x_bits: Vec<usize> = Vec::new();
//                 {
//                     let mut x_ones = ps.old_mask;
//                     for i in 0..36 {
//                         if x_ones & 1 == 1 {
//                             x_bits.push(i);
//                         }
//                         x_ones = x_ones >> 1;
//                     }
//                 }
//
//                 let base_addr = (addr | ps.new_mask) & (!ps.old_mask);
//
//                 let floating_gen_max = 1 << x_bits.len();
//
//                 println!("assign {} mem addrs", floating_gen_max);
//                 for fg in 0..floating_gen_max {
//                     let mut fg_shifter = fg;
//                     let mut fg_bits: Vec<usize> = Vec::new();
//                     for _ in 0..x_bits.len() {
//                         fg_bits.push(fg_shifter & 1);
//                         fg_shifter = fg_shifter >> 1;
//                     }
//
//                     let mut new_addr = base_addr;
//                     for (bit_pos, bit_val) in x_bits.iter().zip(fg_bits.iter()) {
//                         new_addr = (new_addr & !(1 << bit_pos)) | (bit_val << bit_pos);
//                     }
//
//                     let mem_val = ps.memory.entry(new_addr).or_insert(0);
//                     *mem_val = *value;
//                 }
//             }
//         }
//         // println!("{:?}", ps);
//     }
//
//     ps
// }

fn run_p2(insts: &Vec<Instruction>) -> usize {
    let mut writes: Vec<(usize, usize, usize)> = Vec::new();
    let mut ps = ProgramState {
        old_mask: 0,
        new_mask: 0,
        memory: HashMap::new(),
    };

    for i in insts.iter() {
        match i {
            Instruction::Mask { old_mask, new_mask } => {
                ps.old_mask = *old_mask;
                ps.new_mask = *new_mask;
            }
            Instruction::Assignment { addr, value } => {
                writes.push((*value, ps.old_mask, ((addr & !ps.old_mask) | ps.new_mask)));
            }
        }
    }

    // 1: 0b00X1 fl:0010 fx:0001
    // 2: 0b00XX fl:0011 fx:0000
    //  400
    //
    //

    // 1: 1     0bX0X1 1010 0001
    // 2: 100   0b0XXX 0111 0000
    // 3: 10000 0b01XX 0011 0100
    //
    // 1) 4 * 10000 = 40000
    // 2) new effective addr = 0b00XX
    //    2^2 * 100 = 400;
    // 3) 4 possible
    //     new effective addr = 0b10X1
    //     2^1 * 1 = 2
    //     40402

    println!("writes {:?}", writes);
    // start from the back
    // mask off all the values get overwritten later,
    // get remaining floating bits, add assigned value * 2^floating bits
    //
    // (1) if any fixed bits differ, they don't overlap, no cancellation
    // (2) if a bit is floating and later a fixed bit writes, floating becomes flipped fix
    // (2) if a bit is fixed and later a floating writes, floating becomes flipped fix
    let mut memory_sum: usize = 0;
    for (i, write) in writes.iter().enumerate().rev() {
        let value = write.0;
        let mut floating_mask = write.1;
        let mut fixed_mask = write.2;
        let mut overwritten = false;
        println!("{}", value);
        println!("fl: {:036b}", floating_mask);
        println!("fx: {:036b}", fixed_mask);
        let floating_bits = floating_mask.count_ones();
        println!(" b: {}", floating_bits);
        println!("=======");

        for later_write in (&writes[i + 1..]).iter() {
            let later_floating_mask = later_write.1;
            let later_fixed_mask = later_write.2;

            let tsmask = 0xfffffff000000000;
            if floating_mask & tsmask > 0
                || fixed_mask & tsmask > 0
                || later_floating_mask & tsmask > 0
                || later_fixed_mask & tsmask > 0
            {
                panic!("something happened");
            }
            println!("\t v: {}", later_write.0);
            println!("\tfl: {:036b}", later_floating_mask);
            println!("\tfx: {:036b}", later_fixed_mask);

            // skip there's no overlap
            if (!later_floating_mask & !floating_mask & later_fixed_mask)
                != (!later_floating_mask & !floating_mask & fixed_mask)
            {
                println!("skipping");
                continue;
            }

            // if new floating mask is subset of later floating mask
            // then we're overwritten entirely

            fixed_mask |= (!later_floating_mask & floating_mask) & (!later_fixed_mask);
            floating_mask &= !(!later_floating_mask & floating_mask);
            println!("\tnl: {:036b}", floating_mask);
            println!("\tnx: {:036b}", fixed_mask);
            let floating_bits = floating_mask.count_ones();
            println!("\t b: {}", floating_bits);

            if (!later_floating_mask & !floating_mask & later_fixed_mask)
                == (!later_floating_mask & !floating_mask & fixed_mask)
            {
                if !later_floating_mask & floating_mask == 0 {
                    println!("overwritten");
                    overwritten = true;
                    break;
                }
            }
        }

        if overwritten {
            continue;
        }
        let floating_bits = floating_mask.count_ones();

        memory_sum += (value) * (1 << floating_bits);
    }

    memory_sum
}

fn run(insts: &Vec<Instruction>) -> ProgramState {
    let mut ps = ProgramState {
        old_mask: 0,
        new_mask: 0,
        memory: HashMap::new(),
    };

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
                let old_mask_str = mask_slice.replace('1', "0").replace('X', "1");
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
