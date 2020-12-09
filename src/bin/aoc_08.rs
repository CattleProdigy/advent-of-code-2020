use std::collections::HashSet;
use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum OpCode {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, PartialEq, Clone)]
struct Instruction {
    op: OpCode,
    operand0: Option<i64>,
}

impl FromStr for Instruction {
    type Err = String;
    fn from_str(input: &str) -> Result<Instruction, Self::Err> {
        let mut iter = input.split(" ");
        let op_code = iter.next().ok_or("No opcode").map(&OpCode::from_str)?;
        let operand0 = iter
            .next()
            .ok_or("No operand0")
            .map(&str::parse::<i64>)?
            .map_err(|x| format!("Couldn't parse operand0, got: {}", x.to_string()));

        Ok(Instruction {
            op: op_code?,
            operand0: Some(operand0?),
        })
    }
}

impl FromStr for OpCode {
    type Err = String;
    fn from_str(input: &str) -> Result<OpCode, Self::Err> {
        match input {
            "acc" => Ok(OpCode::Acc),
            "jmp" => Ok(OpCode::Jmp),
            "nop" => Ok(OpCode::Nop),
            _ => Err(format!("Unknown OpCode: {}", input)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct ProgramState {
    prog_counter: usize,
    acc: i64,
}

impl ProgramState {
    fn run(&mut self, int: &Instruction) {
        match int.op {
            OpCode::Acc => self.accumulate(int.operand0.unwrap()),
            OpCode::Jmp => self.jump(int.operand0.unwrap()),
            OpCode::Nop => (),
        }
        self.prog_counter = self
            .prog_counter
            .checked_add(1)
            .expect("Underflow in program counter");
    }

    fn jump(&mut self, offset: i64) {
        if offset >= 0 {
            self.prog_counter = self
                .prog_counter
                .checked_add(offset as usize)
                .expect("Overflow in program counter");
        } else {
            self.prog_counter = self
                .prog_counter
                .checked_sub(-offset as usize)
                .expect("Underflow in program counter");
        }
        // counteract the otherwise +1
        self.prog_counter = self
            .prog_counter
            .checked_sub(1)
            .expect("Underflow in program counter");
    }

    fn accumulate(&mut self, operand: i64) {
        self.acc += operand;
    }
}

impl Default for ProgramState {
    fn default() -> Self {
        ProgramState {
            prog_counter: 0,
            acc: 0,
        }
    }
}

fn parse(program_str: &str) -> Result<Vec<Instruction>, String> {
    program_str
        .lines()
        .map(|x| Instruction::from_str(&x))
        .collect()
}

fn run(prog: &Vec<Instruction>) -> ProgramState {
    let mut prog_state = ProgramState::default();
    let terminate_addr = prog.len();
    loop {
        if prog_state.prog_counter == terminate_addr {
            return prog_state;
        }
        prog_state.run(&prog[prog_state.prog_counter]);
    }
}

/**
 * Runs the program from a default program state and
 * calls the callback with the program state after
 * each instruction is ran. The program counter will
 * contain the index of the next instruction that will
 * be ran and the current state of other registers.
 *
 * The callback should return true to interrupt execution
 * and false otherwise.
 *
 * Returns the last program state and whether the program
 * terminated naturally or was interrupted by a callback.
 */
fn run_with_callback(
    prog: &Vec<Instruction>,
    step_callback: &mut impl FnMut(&ProgramState) -> bool,
) -> (ProgramState, bool) {
    let mut prog_state = ProgramState::default();
    let terminate_addr = prog.len();
    loop {
        if prog_state.prog_counter == terminate_addr {
            return (prog_state, true);
        }
        prog_state.run(&prog[prog_state.prog_counter]);
        if step_callback(&prog_state) {
            return (prog_state, false);
        }
    }
}

fn part_1(prog: &Vec<Instruction>) -> i64 {
    let mut visited_pcs = HashSet::<usize>::new();
    visited_pcs.insert(0);
    let (last_program_state, _terminated) = run_with_callback(prog, &mut |prog_state| {
        if visited_pcs.contains(&prog_state.prog_counter) {
            return true;
        }
        visited_pcs.insert(prog_state.prog_counter);
        return false;
    });

    last_program_state.acc
}

fn part_2(prog: &Vec<Instruction>) -> Option<i64> {
    let nj_inds: Vec<usize> = prog
        .iter()
        .enumerate()
        .filter(|(_, int)| int.op == OpCode::Nop || int.op == OpCode::Jmp)
        .map(|(i, _int)| i)
        .collect();

    for i in nj_inds.iter() {
        let mut modified_program = prog.to_vec();
        let int_to_mod = &mut modified_program[*i];
        match int_to_mod.op {
            OpCode::Acc => panic!("this shouldn't happen"),
            OpCode::Jmp => {
                *int_to_mod = Instruction {
                    op: OpCode::Nop,
                    operand0: int_to_mod.operand0,
                }
            }
            OpCode::Nop => {
                *int_to_mod = Instruction {
                    op: OpCode::Jmp,
                    operand0: int_to_mod.operand0,
                }
            }
        };
        let mut seen_program_states = HashSet::<usize>::new();
        seen_program_states.insert(0);
        let (last_program_state, terminated) =
            run_with_callback(&modified_program, &mut |prog_state| {
                if seen_program_states.contains(&prog_state.prog_counter) {
                    return true;
                }
                seen_program_states.insert(prog_state.prog_counter);
                return false;
            });

        if terminated {
            return Some(last_program_state.acc);
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("We need exactly one arguemnt, got: {}", args.len() - 1);
    }

    let file_contents = fs::read_to_string(&args[1]).expect("couldn't read the file");
    let prog = parse(&file_contents).unwrap();

    println!("part 1: {}", part_1(&prog));
    println!("part 2: {:?}", part_2(&prog));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let example1 = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;

        let parse_res = parse(&example1);
        assert!(parse_res.is_ok());
        let prog = parse_res.unwrap();
        assert_eq!(part_1(&prog), 5);

        let mut adjusted_prog = prog.to_vec();
        let second_from_end_idx = adjusted_prog.len() - 2;
        adjusted_prog[second_from_end_idx] = Instruction {
            op: OpCode::Nop,
            operand0: Some(-4),
        };
        let (ps, terminated) = run(&adjusted_prog);
        assert!(terminated);
        assert_eq!(ps.acc, 8);
    }
}
