use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OpCode {
    Acc,
    Jmp,
    Nop,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Instruction {
    op: OpCode,
    arg: i64,
}

impl Instruction {
    fn create_from(s: &str) -> Instruction {
        match (&s[0..3], s[4..].parse::<i64>().unwrap()) {
            ("acc", n) => Instruction{ op: OpCode::Acc, arg: n },
            ("jmp", n) => Instruction{ op: OpCode::Jmp, arg: n },
            ("nop", n) => Instruction{ op: OpCode::Nop, arg: n },
            _ => { panic!() },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExitStatus {
    Completed(i64),
    ErrorInfiniteLoop(i64),
    ErrorInvalidJump,
}

fn execute(code: &[Instruction]) -> ExitStatus {
    let mut accumulator = 0i64;
    let mut ptr = 0usize;
    let mut visited = HashSet::new();

    while ptr < code.len() {
        if !visited.insert(ptr) {
            // infinite loop detected
            return ExitStatus::ErrorInfiniteLoop(accumulator);
        }

        let Instruction{ op, arg } = code[ptr];
        match op {
            OpCode::Acc => {
                accumulator += arg;
                ptr += 1;
            },
            OpCode::Jmp if arg < 0 => {
                match ptr.checked_sub(-arg as usize) {
                    Some(p) => { ptr = p; },
                    None => { return ExitStatus::ErrorInvalidJump; },
                }
            },
            OpCode::Jmp /* if arg >= 0 */ => {
                match ptr.checked_add(arg as usize) {
                    Some(p) => { ptr = p; },
                    None => { return ExitStatus::ErrorInvalidJump; },
                }
            },
            OpCode::Nop => {
                ptr += 1;
            }
        }
    }

    ExitStatus::Completed(accumulator)
}

fn solve(input: &str) -> (Option<i64>, Option<i64>) {
    let code: Vec<_> = input.lines().map(|s| Instruction::create_from(s.trim())).collect();

    let mut result_corrupted = None;
    if let ExitStatus::ErrorInfiniteLoop(res) = execute(&code) {
        result_corrupted = Some(res);
    }

    let mut result_fixed = None;
    for i in 0..code.len() {
        let mut new_code = code.clone();

        match new_code[i].op {
            OpCode::Jmp => { new_code[i].op = OpCode::Nop; },
            OpCode::Nop => { new_code[i].op = OpCode::Jmp; },
            _ => { continue; },
        }

        //println!("Result after changing instruction {}: {:?}", i, execute(&new_code));

        if let ExitStatus::Completed(res) = execute(&new_code) {
            result_fixed = Some(res);
            break;
        }
    }

    (result_corrupted, result_fixed)
}

fn main() {
    let input = std::fs::read_to_string("input/08.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        assert_eq!(solve("\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"), (Some(5), Some(8)));
    }
}
