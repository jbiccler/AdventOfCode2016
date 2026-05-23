advent_of_code::solution!(23);

#[derive(Clone, Copy, Debug)]
enum Value {
    Register(usize),
    Integer(i64),
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
    Tgl(Value),
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
                "inc" => Instruction::Inc(Value::Register(register_index(parts[1]))),
                "dec" => Instruction::Dec(Value::Register(register_index(parts[1]))),
                "jnz" => {
                    let parsed1 = parts[1].parse::<i64>();
                    let parsed2 = parts[2].parse::<i64>();
                    match (parsed1, parsed2) {
                        (Ok(x), Ok(y)) => Instruction::Jnz(Value::Integer(x), Value::Integer(y)),
                        (Ok(x), Err(_)) => Instruction::Jnz(
                            Value::Integer(x),
                            Value::Register(register_index(parts[2])),
                        ),
                        (Err(_), Ok(y)) => Instruction::Jnz(
                            Value::Register(register_index(parts[1])),
                            Value::Integer(y),
                        ),
                        (Err(_), Err(_)) => Instruction::Jnz(
                            Value::Register(register_index(parts[1])),
                            Value::Register(register_index(parts[2])),
                        ),
                    }
                }
                "cpy" => {
                    let parsed = parts[1].parse::<i64>();
                    if let Ok(val) = parsed {
                        Instruction::Cpy(
                            Value::Integer(val),
                            Value::Register(register_index(parts[2])),
                        )
                    } else {
                        Instruction::Cpy(
                            Value::Register(register_index(parts[1])),
                            Value::Register(register_index(parts[2])),
                        )
                    }
                }
                "tgl" => {
                    let parsed = parts[1].parse::<i64>();
                    if let Ok(val) = parsed {
                        Instruction::Tgl(Value::Integer(val))
                    } else {
                        Instruction::Tgl(Value::Register(register_index(parts[1])))
                    }
                }
                _ => panic!(),
            }
        })
        .collect()
}

fn apply_instruction(idx: usize, ins: &mut [Instruction], registers: &mut [i64]) -> usize {
    // Multiplication optimization:
    // cpy b c
    // inc a
    // dec c
    // jnz c -2
    // dec d
    // jnz d -5
    // => a += b * d; c = 0; d = 0;
    if idx + 5 < ins.len() {
        if let (
            Instruction::Cpy(x, Value::Register(c)),
            Instruction::Inc(Value::Register(a)),
            Instruction::Dec(Value::Register(c2)),
            Instruction::Jnz(Value::Register(c3), Value::Integer(-2)),
            Instruction::Dec(Value::Register(d)),
            Instruction::Jnz(Value::Register(d2), Value::Integer(-5)),
        ) = (
            ins[idx],
            ins[idx + 1],
            ins[idx + 2],
            ins[idx + 3],
            ins[idx + 4],
            ins[idx + 5],
        ) {
            if c == c2 && c == c3 && d == d2 && a != c && a != d && c != d {
                let val_x = match x {
                    Value::Register(r) => registers[r],
                    Value::Integer(i) => i,
                };
                registers[a] += val_x * registers[d];
                registers[c] = 0;
                registers[d] = 0;
                return idx + 6;
            }
        }
    }

    match ins[idx] {
        Instruction::Cpy(x, y) => match (x, y) {
            (Value::Integer(i), Value::Register(r)) => registers[r] = i,
            (Value::Register(from), Value::Register(r)) => registers[r] = registers[from],
            _ => (),
        },
        Instruction::Inc(x) => match x {
            Value::Register(r) => registers[r] += 1,
            _ => (),
        },
        Instruction::Dec(x) => match x {
            Value::Register(r) => registers[r] -= 1,
            _ => (),
        },
        Instruction::Jnz(x, y) => {
            let val_x = match x {
                Value::Register(r) => registers[r],
                Value::Integer(i) => i,
            };
            let val_y = match y {
                Value::Register(r) => registers[r],
                Value::Integer(i) => i,
            };
            if val_x != 0 {
                return idx.saturating_add_signed(val_y as isize);
            }
        }
        Instruction::Tgl(t) => {
            let val = match t {
                Value::Register(r) => registers[r],
                Value::Integer(i) => i,
            };
            let target_idx = idx.saturating_add_signed(val as isize);
            if target_idx < ins.len() {
                let target = ins[target_idx];

                match target {
                    Instruction::Inc(x) => ins[target_idx] = Instruction::Dec(x),
                    Instruction::Dec(x) => ins[target_idx] = Instruction::Inc(x),
                    Instruction::Tgl(x) => ins[target_idx] = Instruction::Inc(x),
                    Instruction::Jnz(x, y) => ins[target_idx] = Instruction::Cpy(x, y),
                    Instruction::Cpy(x, y) => ins[target_idx] = Instruction::Jnz(x, y),
                }
            }
        }
    }
    idx + 1
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut parsed = parse_input(input);
    let mut registers = vec![0; 4];
    registers[0] = 7;
    let mut idx = 0;
    while idx < parsed.len() {
        idx = apply_instruction(idx, &mut parsed, &mut registers);
    }
    Some(registers[0])
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut parsed = parse_input(input);
    let mut registers = vec![0; 4];
    registers[0] = 12;
    let mut idx = 0;
    while idx < parsed.len() {
        idx = apply_instruction(idx, &mut parsed, &mut registers);
    }
    Some(registers[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
