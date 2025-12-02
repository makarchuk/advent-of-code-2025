use crate::tasks::task::TaskInput;

pub struct Task;

impl crate::tasks::task::Task for Task {
    type TaskInput = Input;

    fn part1(&self, input: Self::TaskInput) -> String {
        let sum: u64 = input
            .ranges
            .iter()
            .map(|range| range.clone().filter(|&id| !is_valid(id)).sum::<u64>())
            .sum();
        sum.to_string()
    }

    fn part2(&self, input: Self::TaskInput) -> String {
        let sum: u64 = input
            .ranges
            .iter()
            .map(|range| {
                range
                    .clone()
                    .filter(|&id| !is_valid_for_p2(id))
                    .sum::<u64>()
            })
            .sum();
        sum.to_string()
    }
}

pub struct Input {
    ranges: Vec<std::ops::Range<u64>>,
}

impl TaskInput for Input {
    fn from_str(s: &str) -> Input {
        let ranges = s
            .split(",")
            .map(|part| {
                let bounds = part.trim().split('-').collect::<Vec<_>>();
                let start = bounds
                    .first()
                    .expect("Expected start of range")
                    .parse::<u64>()
                    .expect("Expected valid number for start of range");
                let end = bounds
                    .last()
                    .expect("Expected end of range")
                    .parse::<u64>()
                    .unwrap();
                start..end
            })
            .collect();
        Input { ranges }
    }
}

fn is_valid(id: u64) -> bool {
    let num_digits = id.to_string().len();

    if num_digits % 2 != 0 {
        return true;
    }

    let half_digits = num_digits / 2;

    let mask = 10u64.pow(half_digits as u32);

    let (div, rem) = (id / mask, id % mask);
    div != rem
}

fn is_valid_for_p2(id: u64) -> bool {
    let num_digits = id.to_string().len();

    for i in 1..=num_digits / 2 {
        if num_digits % i != 0 {
            continue;
        }

        let mask = 10u64.pow(i as u32);
        let repeats = num_digits / i;
        let repeated_part = make_n_repeats(id % mask, repeats, mask);
        if repeated_part == id {
            return false;
        }
    }

    true
}

fn make_n_repeats(num: u64, repeats: usize, mask: u64) -> u64 {
    let mut result = 0;
    for _ in 0..repeats {
        result = result * mask + num;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(!is_valid(123123));
        assert!(!is_valid(456456));
        assert!(is_valid(123456));
        assert!(is_valid(12345));
        assert!(is_valid(1231234));
    }

    #[test]
    fn test_make_n_repeats() {
        assert_eq!(make_n_repeats(12, 3, 100), 121212);
        assert_eq!(make_n_repeats(5, 4, 10), 5555);
        assert_eq!(make_n_repeats(9, 2, 10), 99);
    }

    #[test]
    fn test_is_valid_p2() {
        assert!(!is_valid_for_p2(123123));
        assert!(!is_valid_for_p2(456456));
        assert!(is_valid_for_p2(123456));
        assert!(is_valid_for_p2(12345));
        assert!(is_valid_for_p2(1231234));
        assert!(!is_valid_for_p2(121212));
        assert!(!is_valid_for_p2(1111));
    }
}
