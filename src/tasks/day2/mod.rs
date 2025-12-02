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
        unimplemented!()
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
}
