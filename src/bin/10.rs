use std::collections::VecDeque;

use regex::Regex;
advent_of_code::solution!(10);

#[derive(Debug, Copy, Clone)]
struct Bot {
    id: usize,
    high: Option<u64>,
    low: Option<u64>,
}

impl Bot {
    fn insert(&mut self, val: u64) -> Option<usize> {
        match self.high {
            None => match self.low {
                None => self.low = Some(val),
                Some(l) => {
                    if val > l {
                        self.high = Some(val)
                    } else {
                        self.high = self.low;
                        self.low = Some(val);
                    }
                    if (Some(17), Some(61)) == (self.low, self.high) {
                        return Some(self.id);
                    }
                }
            },
            Some(h) => match self.low {
                None => {
                    if val <= h {
                        self.low = Some(val);
                        if (Some(17), Some(61)) == (self.low, self.high) {
                            return Some(self.id);
                        }
                    }
                }
                Some(_l) => {
                    // Both high and low already exist, no more room to store an extra value
                    panic!();
                }
            },
        }
        None
    }
    fn clear_low(&mut self) {
        self.low = None;
    }
    fn clear_high(&mut self) {
        self.high = None;
    }
}

#[derive(Debug, Copy, Clone)]
struct Output {
    _id: usize,
    val: Option<u64>,
}

#[derive(Debug, Copy, Clone)]
enum Target {
    BotTarget { id: usize },
    OutputTarget { id: usize },
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    id: usize,
    target1: Target,
    target2: Target,
}

fn parse_input(input: &str) -> (Vec<Bot>, Vec<Output>, VecDeque<Instruction>) {
    // First parse the instructions and track the highest bot and output ID;
    let mut max_output_id = 0;
    let mut max_bot_id = 0;

    // Perf: regex could really be avoided here but simpler like this.
    let re_ins =
        Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)")
            .unwrap();

    let ins: Vec<Instruction> = input
        .trim()
        .lines()
        .filter(|line| line.starts_with("bot "))
        .map(|line| {
            let caps = re_ins.captures(line).unwrap();
            let id = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let target1_str = caps.get(2).unwrap().as_str();
            let target1_id = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let target2_str = caps.get(4).unwrap().as_str();
            let target2_id = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();

            let target1 = match target1_str {
                "output" => {
                    max_output_id = max_output_id.max(target1_id);
                    Target::OutputTarget { id: target1_id }
                }
                "bot" => {
                    max_bot_id = max_bot_id.max(target1_id);
                    Target::BotTarget { id: target1_id }
                }
                _ => unreachable!(),
            };
            let target2 = match target2_str {
                "output" => {
                    max_output_id = max_output_id.max(target2_id);
                    Target::OutputTarget { id: target2_id }
                }
                "bot" => {
                    max_bot_id = max_bot_id.max(target2_id);
                    Target::BotTarget { id: target2_id }
                }
                _ => unreachable!(),
            };

            Instruction {
                id,
                target1,
                target2,
            }
        })
        .collect();
    let ins = VecDeque::from(ins);

    let mut bots: Vec<Bot> = (0..=max_bot_id)
        .map(|i| Bot {
            id: i,
            high: None,
            low: None,
        })
        .collect();

    // Perf: regex could really be avoided here but simpler like this.
    let re_start = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();

    input
        .trim()
        .lines()
        .filter(|line| line.starts_with("value "))
        .for_each(|line| {
            let caps = re_start.captures(line).unwrap();
            let val = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let id = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            bots[id].insert(val);
        });

    let outputs = (0..=max_output_id)
        .map(|i| Output { _id: i, val: None })
        .collect();

    (bots, outputs, ins)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut bots, mut outputs, mut ins) = parse_input(input);
    while let Some(i) = ins.pop_front() {
        if bots[i.id].low.is_none() || bots[i.id].high.is_none() {
            // Invalid instruction for now
            ins.push_back(i);
            continue;
        }
        // Process the 'low' target
        match i.target1 {
            Target::BotTarget { id: target_id } => {
                let val = bots[i.id].low.unwrap();
                if let Some(result) = bots[target_id].insert(val) {
                    return Some(result);
                }
                bots[i.id].clear_low();
            }
            Target::OutputTarget { id: target_id } => {
                outputs[target_id].val = bots[i.id].low;
                bots[i.id].clear_low();
            }
        };
        // Process the 'high' target
        match i.target2 {
            Target::BotTarget { id: target_id } => {
                let val = bots[i.id].high.unwrap();
                if let Some(result) = bots[target_id].insert(val) {
                    return Some(result);
                }
                bots[i.id].clear_high();
            }
            Target::OutputTarget { id: target_id } => {
                outputs[target_id].val = bots[i.id].high;
                bots[i.id].clear_high();
            }
        };
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut bots, mut outputs, mut ins) = parse_input(input);
    while let Some(i) = ins.pop_front() {
        if bots[i.id].low.is_none() || bots[i.id].high.is_none() {
            ins.push_back(i);
            continue;
        }
        if !outputs[0].val.is_none() && !outputs[1].val.is_none() && !outputs[2].val.is_none() {
            return Some(outputs.iter().take(3).map(|o| o.val.unwrap()).product());
        }
        match i.target1 {
            Target::BotTarget { id: target_id } => {
                let val = bots[i.id].low.unwrap();
                bots[target_id].insert(val);
                bots[i.id].clear_low();
            }
            Target::OutputTarget { id: target_id } => {
                outputs[target_id].val = bots[i.id].low;
                bots[i.id].clear_low();
            }
        };
        match i.target2 {
            Target::BotTarget { id: target_id } => {
                let val = bots[i.id].high.unwrap();
                bots[target_id].insert(val);
                bots[i.id].clear_high();
            }
            Target::OutputTarget { id: target_id } => {
                outputs[target_id].val = bots[i.id].high;
                bots[i.id].clear_high();
            }
        };
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
