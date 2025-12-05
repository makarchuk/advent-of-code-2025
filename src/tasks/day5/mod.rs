pub struct Task {}

impl super::task::Task for Task {
    type TaskInput = Input;
    fn part1(&self, input: Self::TaskInput) -> String {
        let mut valid_count = 0;

        for &id in &input.ids {
            if input.fresh_ranges.iter().any(|range| range.contains(&id)) {
                valid_count += 1;
            }
        }

        valid_count.to_string()
    }

    fn part2(&self, input: Self::TaskInput) -> String {
        input.simplify_ranges().total_fresh().to_string()
    }
}

pub struct Input {
    fresh_ranges: Vec<std::ops::RangeInclusive<u64>>,
    ids: Vec<u64>,
}

impl Input {
    fn mergeble(a: &std::ops::RangeInclusive<u64>, b: &std::ops::RangeInclusive<u64>) -> bool {
        b.end() + 1 >= *a.start() && *b.start() <= a.end() + 1
    }

    fn merge_ranges(
        a: &std::ops::RangeInclusive<u64>,
        b: &std::ops::RangeInclusive<u64>,
    ) -> std::ops::RangeInclusive<u64> {
        *(a.start()).min(b.start())..=*(a.end()).max(b.end())
    }

    fn simplify_ranges(self) -> Self {
        let mut ranges = self.fresh_ranges;

        let mut changes = true;
        while changes {
            changes = false;
            let mut new_ranges = vec![];

            for range in ranges.iter() {
                let mut merge_found = false;
                for (i, other_range) in new_ranges.iter().enumerate() {
                    if Self::mergeble(range, other_range) {
                        let merged = Self::merge_ranges(range, other_range);
                        new_ranges[i] = merged;
                        merge_found = true;
                        break;
                    }
                }
                if !merge_found {
                    new_ranges.push(range.clone());
                } else {
                    changes = true;
                }
            }
            ranges = new_ranges;
        }
        Self {
            fresh_ranges: ranges,
            ids: self.ids,
        }
    }

    fn total_fresh(&self) -> u64 {
        self.fresh_ranges
            .iter()
            .map(|r| r.end() - r.start() + 1)
            .sum()
    }
}

impl super::task::TaskInput for Input {
    fn from_str(s: &str) -> Input {
        let (ranges, ids) = s.split_once("\n\n").unwrap();

        let fresh_ranges = ranges
            .lines()
            .map(|l| {
                let (start, end) = l.split_once("-").unwrap();

                let start = start
                    .parse::<u64>()
                    .expect("Expected valid number for start of range");
                let end = end
                    .parse::<u64>()
                    .expect("Expected valid number for end of range");
                start..=end
            })
            .collect();

        let ids = ids.lines().map(|n| n.parse::<u64>().unwrap()).collect();

        Input { fresh_ranges, ids }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify_ranges() {
        let input = Input {
            fresh_ranges: vec![1..=5, 6..=10, 15..=20, 18..=25, 30..=35],
            ids: vec![],
        };

        let simplified_input = input.simplify_ranges();
        assert_eq!(
            vec![1..=10, 15..=25, 30..=35],
            simplified_input.fresh_ranges
        );
    }
}
