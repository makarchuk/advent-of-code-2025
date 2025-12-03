use core::num;

use super::task;

pub struct Task;

impl task::Task for Task {
    type TaskInput = Input;

    fn part1(&self, input: Self::TaskInput) -> String {
        let sum: u64 = input
            .batteries
            .iter()
            .map(|battery| battery.top_joltage())
            .sum();
        sum.to_string()
    }

    fn part2(&self, input: Self::TaskInput) -> String {
        let sum: u64 = input
            .batteries
            .iter()
            .map(|battery| battery.top_joltage_of_len(12))
            .sum();
        sum.to_string()
    }
}

pub struct Input {
    batteries: Vec<Battery>,
}

struct Battery {
    banks: Vec<u8>,
}

impl task::TaskInput for Input {
    fn from_str(s: &str) -> Input {
        let batteries = s
            .lines()
            .map(|line| {
                let banks = line
                    .trim()
                    .chars()
                    .map(|num_char| {
                        num_char.to_string().parse::<u8>().expect(
                            format!("Expected valid number for bank, got {}", num_char).as_str(),
                        )
                    })
                    .collect();
                Battery { banks }
            })
            .collect();
        Input { batteries }
    }
}

impl Battery {
    fn top_joltage(&self) -> u64 {
        self.top_joltage_of_len(2)
    }

    fn top_joltage_of_len(&self, n: usize) -> u64 {
        let mut top_digits = self.banks[..n].to_vec();

        for (i, bank) in self.banks.iter().enumerate().skip(1) {
            // how many banks we have at the tail of the battery. We need at least enough banks to fill the top n digits
            let remaining_banks = self.banks.len() - i;

            let start_index = if remaining_banks > n {
                0
            } else {
                n - remaining_banks
            };
            let end_index = n.clamp(0, i);

            for i in start_index..end_index {
                if *bank > top_digits[i] {
                    top_digits[i] = *bank;
                    for j in (i + 1)..n {
                        top_digits[j] = 0;
                    }
                    break;
                }
            }
        }

        top_digits
            .iter()
            .fold(0u64, |acc, digit| acc * 10 + (*digit as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_joltage() {
        struct TestCase {
            battery: Battery,
            top_jolatage_two: u64,
            top_jolatage_twelve: u64,
        }

        let test_cases = vec![
            TestCase {
                battery: Battery {
                    banks: vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
                },
                top_jolatage_two: 98,
                top_jolatage_twelve: 987654321111,
            },
            TestCase {
                battery: Battery {
                    banks: vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
                },
                top_jolatage_two: 89,
                top_jolatage_twelve: 811111111119,
            },
            TestCase {
                battery: Battery {
                    banks: vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
                },
                top_jolatage_two: 78,
                top_jolatage_twelve: 434234234278,
            },
            TestCase {
                battery: Battery {
                    banks: vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
                },
                top_jolatage_two: 92,
                top_jolatage_twelve: 888911112111,
            },
        ];
        for case in test_cases {
            assert_eq!(
                case.battery.top_joltage(),
                case.top_jolatage_two,
                "Failed on battery: {:?}",
                case.battery.banks
            );
            assert_eq!(
                case.battery.top_joltage_of_len(2),
                case.top_jolatage_two,
                "Failed on battery: {:?}",
                case.battery.banks
            );
            assert_eq!(
                case.battery.top_joltage_of_len(12),
                case.top_jolatage_twelve,
                "Failed on battery: {:?}",
                case.battery.banks
            );
        }
    }
}
