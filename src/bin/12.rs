advent_of_code::solution!(12);

#[derive(Debug, Clone, Copy)]
enum Instruction {
    CopyNumeric { val: i64, register: usize },
    CopyRegister { from: usize, register: usize },
    Inc { register: usize },
    Dec { register: usize },
    JumpRegister { register: usize, val: i64 },
    JumpNumeric { non_zero: bool, val: i64 },
}

fn register_index(c: &str) -> usize {
    match c {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!(),
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "inc" => Instruction::Inc {
                    register: register_index(parts[1]),
                },
                "dec" => Instruction::Dec {
                    register: register_index(parts[1]),
                },
                "jnz" => {
                    let parsed = parts[1].parse::<i64>();
                    if let Ok(t) = parsed {
                        Instruction::JumpNumeric {
                            non_zero: t != 0,
                            val: parts[2].parse::<i64>().unwrap(),
                        }
                    } else {
                        Instruction::JumpRegister {
                            register: register_index(parts[1]),
                            val: parts[2].parse::<i64>().unwrap(),
                        }
                    }
                }
                "cpy" => {
                    let parsed = parts[1].parse::<i64>();
                    if let Ok(val) = parsed {
                        Instruction::CopyNumeric {
                            val,
                            register: register_index(parts[2]),
                        }
                    } else {
                        Instruction::CopyRegister {
                            from: register_index(parts[1]),
                            register: register_index(parts[2]),
                        }
                    }
                }
                _ => panic!(),
            }
        })
        .collect()
}

fn apply_instruction(idx: usize, ins: &[Instruction], registers: &mut [i64]) -> usize {
    match ins[idx] {
        Instruction::CopyRegister { from, register } => registers[register] = registers[from],
        Instruction::CopyNumeric { val, register } => registers[register] = val,
        Instruction::Inc { register } => registers[register] += 1,
        Instruction::Dec { register } => registers[register] -= 1,
        Instruction::JumpNumeric { non_zero, val } => {
            if non_zero {
                return idx.saturating_add_signed(val as isize);
            }
        }
        Instruction::JumpRegister { register, val } => {
            if registers[register] != 0 {
                return idx.saturating_add_signed(val as isize);
            }
        }
    }
    idx + 1
}

pub fn part_one(input: &str) -> Option<i64> {
    let parsed = parse_input(input);
    let mut registers = vec![0; 4];
    let mut idx = 0;
    while idx < parsed.len() {
        idx = apply_instruction(idx, &parsed, &mut registers)
    }
    Some(registers[0])
}

pub fn part_two(input: &str) -> Option<i64> {
    let parsed = parse_input(input);
    let mut registers = vec![0; 4];
    registers[2] = 1;
    let mut idx = 0;
    while idx < parsed.len() {
        idx = apply_instruction(idx, &parsed, &mut registers)
    }
    Some(registers[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }
}
