advent_of_code::solution!(19);

pub fn _part_one_brute(input: &str) -> Option<usize> {
    // Initially calculated the solution in a more brute-froce manner
    // But a analytical solution exists, see fn part_one()
    let n = input.trim().parse::<usize>().unwrap();
    let mut alive = vec![true; n];
    let mut remaining = n;
    let mut i = 0;

    while remaining > 1 {
        if !alive[i] {
            i = (i + 1) % n;
            continue;
        }
        // next alive target
        let mut j = (i + 1) % n;
        while !alive[j] {
            j = (j + 1) % n;
        }

        alive[j] = false;
        remaining -= 1;
        i = j;
    }

    Some(alive.iter().position(|&x| x).unwrap() + 1)
}

pub fn part_one(input: &str) -> Option<usize> {
    // Mathematically, this is the Josephus problem
    let n = input.trim().parse::<usize>().unwrap();
    let p = n.ilog2();
    Some(2 * (n - 2usize.pow(p)) + 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let n = input.trim().parse::<usize>().unwrap();

    // Reduces to a ternary structure/pattern
    // Find largest power of 3
    let mut p = 1;
    while p * 3 <= n {
        p *= 3;
    }
    // Mathematical pattern:
    if n == p {
        Some(n)
    } else if n <= 2 * p {
        Some(n - p)
    } else {
        Some(2 * n - 3 * p)
    }
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
        assert_eq!(result, Some(2));
    }
}
