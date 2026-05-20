use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
advent_of_code::solution!(11);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pair {
    chip: u8,
    rtg: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    elevator: u8,
    pairs: Vec<Pair>,
}

fn parse_input(input: &str) -> State {
    let mut map: HashMap<&str, (Option<u8>, Option<u8>)> = HashMap::new();
    let re_rtg = Regex::new(r"(\w+) generator").unwrap();
    let re_mc = Regex::new(r"(\w+)-compatible").unwrap();

    for (floor, line) in input.lines().enumerate() {
        let floor = floor as u8;
        for (_, [key]) in re_mc.captures_iter(line).map(|c| c.extract()) {
            map.entry(key).or_insert((None, None)).0 = Some(floor);
        }
        for (_, [key]) in re_rtg.captures_iter(line).map(|c| c.extract()) {
            map.entry(key).or_insert((None, None)).1 = Some(floor);
        }
    }

    let mut pairs: Vec<Pair> = map
        .values()
        .map(|(chip, rtg)| Pair {
            chip: chip.unwrap(),
            rtg: rtg.unwrap(),
        })
        .collect();
    pairs.sort_unstable();
    State { elevator: 0, pairs }
}

fn is_target_state(state: &State) -> bool {
    state.pairs.iter().all(|p| p.chip == 3 && p.rtg == 3)
}

fn is_valid_state(state: &State) -> bool {
    for p in &state.pairs {
        let chip_floor = p.chip;

        let has_any_generator = state.pairs.iter().any(|other| other.rtg == chip_floor);

        let has_own_generator = p.rtg == chip_floor;

        if has_any_generator && !has_own_generator {
            return false;
        }
    }

    true
}

fn make_uniform(state: &mut State) {
    state.pairs.sort();
}

fn anything_below(state: &State) -> bool {
    state
        .pairs
        .iter()
        .any(|p| p.chip < state.elevator || p.rtg < state.elevator)
}

fn next_possible_states(state: &State) -> HashSet<State> {
    let mut items_current_floor: Vec<(usize, u8)> = vec![];
    for (i, p) in state.pairs.iter().enumerate() {
        if p.chip == state.elevator {
            items_current_floor.push((i, 0)); // chip
        }
        if p.rtg == state.elevator {
            items_current_floor.push((i, 1)); // generator
        }
    }
    let mut result = HashSet::new();

    for dir in [-1, 1] {
        // There's never any point in moving things down to empty floors, just creates loops
        if !anything_below(state) && dir == -1 {
            continue;
        }
        let next_floor = state.elevator as i32 + dir;
        if !(0..=3).contains(&next_floor) {
            continue;
        }
        let next_floor = next_floor as u8;

        // Pick 1 or 2 items
        for i in 0..items_current_floor.len() {
            for j in i..items_current_floor.len() {
                let mut next = state.clone();
                next.elevator = next_floor;

                let mut add = |(idx, kind): (usize, u8)| {
                    //chip
                    if kind == 0 {
                        next.pairs[idx].chip = next_floor;
                    } else {
                        next.pairs[idx].rtg = next_floor;
                    }
                };
                // Always at least one item is moved
                add(items_current_floor[i]);
                if i != j {
                    // 2 item cases
                    add(items_current_floor[j]);
                }
                if is_valid_state(&next) {
                    make_uniform(&mut next);
                    result.insert(next);
                }
            }
        }
    }
    result
}

fn bfs(mut start: State) -> Option<usize> {
    if !is_valid_state(&start) {
        return None;
    }
    let mut q = VecDeque::new();
    let mut seen: HashSet<State> = HashSet::new();
    make_uniform(&mut start);

    q.push_back((start.clone(), 0));
    seen.insert(start);

    while let Some((state, dist)) = q.pop_front() {
        if is_target_state(&state) {
            return Some(dist);
        }
        for n in next_possible_states(&state) {
            if seen.insert(n.clone()) {
                q.push_back((n, dist + 1));
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let start_state = parse_input(input);
    bfs(start_state)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut start_state = parse_input(input);
    start_state.pairs.push(Pair { chip: 0, rtg: 0 });
    start_state.pairs.push(Pair { chip: 0, rtg: 0 });
    bfs(start_state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
