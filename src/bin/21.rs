use std::collections::HashMap;
advent_of_code::solution!(21);

#[derive(Debug, Clone, Copy)]
enum Instruction {
    SwapPosition { x: usize, y: usize },
    SwapLetter { x: char, y: char },
    RotateLeft { x: usize },
    RotateRight { x: usize },
    RotatePosition { x: char },
    ReversePosition { x: usize, y: usize },
    MovePosition { x: usize, y: usize },
    InverseRotatePosition { x: char },
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let words: Vec<&str> = line.split_whitespace().collect();
            match (words[0], words[1]) {
                ("swap", "position") => Instruction::SwapPosition {
                    x: words[2].parse().unwrap(),
                    y: words[5].parse().unwrap(),
                },
                ("swap", "letter") => Instruction::SwapLetter {
                    x: words[2].chars().next().unwrap(),
                    y: words[5].chars().next().unwrap(),
                },
                ("rotate", "left") => Instruction::RotateLeft {
                    x: words[2].parse().unwrap(),
                },
                ("rotate", "right") => Instruction::RotateRight {
                    x: words[2].parse().unwrap(),
                },
                ("rotate", "based") => Instruction::RotatePosition {
                    x: words[6].chars().next().unwrap(),
                },
                ("reverse", "positions") => Instruction::ReversePosition {
                    x: words[2].parse().unwrap(),
                    y: words[4].parse().unwrap(),
                },
                ("move", "position") => Instruction::MovePosition {
                    x: words[2].parse().unwrap(),
                    y: words[5].parse().unwrap(),
                },
                _ => panic!("Unknown instruction: {}", line),
            }
        })
        .collect()
}

fn get_password() -> String {
    #[cfg(test)]
    {
        "abcde".to_owned()
    }
    #[cfg(not(test))]
    {
        "abcdefgh".to_owned()
    }
}

fn swap_letter(v: &mut [char], x: char, y: char) {
    for e in v.iter_mut() {
        if *e == x {
            *e = y;
        } else if *e == y {
            *e = x;
        }
    }
}

fn swap_position(v: &mut [char], x: usize, y: usize) {
    v.swap(x, y)
}

fn rotate_left(v: &mut [char], x: usize) {
    v.rotate_left(x)
}

fn rotate_right(v: &mut [char], x: usize) {
    v.rotate_right(x)
}

fn rotate_position(v: &mut [char], x: char) {
    let tmp = v.iter().position(|e| *e == x).unwrap();
    let idx = (1 + tmp + if tmp >= 4 { 1 } else { 0 }) % v.len();
    v.rotate_right(idx);
}

fn reverse_position(v: &mut Vec<char>, x: usize, y: usize) {
    let tmp = v.clone();
    let mut rev = tmp[x..=y].iter().rev();
    for i in x..=y {
        v[i] = *rev.next().unwrap();
    }
}

fn move_position(v: &mut Vec<char>, x: usize, y: usize) {
    let el = v.remove(x);
    v.insert(y, el);
}

fn apply_instruction(ins: Instruction, v: &mut Vec<char>, map: Option<&HashMap<usize, usize>>) {
    match ins {
        Instruction::SwapPosition { x, y } => swap_position(v, x, y),
        Instruction::SwapLetter { x, y } => swap_letter(v, x, y),
        Instruction::RotateLeft { x } => rotate_left(v, x),
        Instruction::RotateRight { x } => rotate_right(v, x),
        Instruction::RotatePosition { x } => rotate_position(v, x),
        Instruction::ReversePosition { x, y } => reverse_position(v, x, y),
        Instruction::MovePosition { x, y } => move_position(v, x, y),
        Instruction::InverseRotatePosition { x } => inverse_rotate_position(v, x, map),
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let instructions = parse_input(input);
    let pass: Vec<char> = get_password().chars().collect();
    let mut current = pass.clone();
    for ins in instructions {
        apply_instruction(ins, &mut current, None);
    }
    Some(current.iter().collect())
}

fn inverse_rotate_position(v: &mut [char], x: char, map: Option<&HashMap<usize, usize>>) {
    let map = map.unwrap();
    let new_idx = v.iter().position(|e| *e == x).unwrap();
    let original_idx = map.get(&new_idx).unwrap();
    let rotation = (new_idx + v.len() - original_idx) % v.len();
    v.rotate_left(rotation);
}

fn inverse_instruction_set(v: &[Instruction]) -> Vec<Instruction> {
    let mut result = Vec::with_capacity(v.len());
    for i in v.iter().rev() {
        let ni = match i {
            Instruction::RotateLeft { x } => Instruction::RotateRight { x: *x },
            Instruction::RotateRight { x } => Instruction::RotateLeft { x: *x },
            Instruction::MovePosition { x, y } => Instruction::MovePosition { x: *y, y: *x },
            Instruction::RotatePosition { x } => Instruction::InverseRotatePosition { x: *x },
            _ => *i,
        };
        result.push(ni);
    }

    result
}

pub fn part_two(input: &str) -> Option<String> {
    // Load instructions and inverse
    let instructions = inverse_instruction_set(&parse_input(input));
    let pass: Vec<char> = "fbgdceah".chars().collect();
    let mut current = pass.clone();

    // Normal map of original index to new index as if we are not inversing
    let mut orig_map = HashMap::new();
    (0..pass.len()).for_each(|i| {
        orig_map.insert(i, (i + (1 + i + if i >= 4 { 1 } else { 0 })) % pass.len());
    });
    // Inverse the map
    let inverted_map: HashMap<usize, usize> = orig_map.iter().map(|(&k, &v)| (v, k)).collect();

    for ins in instructions {
        apply_instruction(ins, &mut current, Some(&inverted_map));
    }
    Some(current.iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("decab".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("efghdabc".to_owned()));
    }
}
