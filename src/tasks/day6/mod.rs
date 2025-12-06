pub struct Task {}

impl super::task::Task for Task {
    type TaskInput = Input;

    fn part1(&self, input: Self::TaskInput) -> String {
        input
            .problems
            .iter()
            .map(|p| p.solve())
            .sum::<u64>()
            .to_string()
    }

    fn part2(&self, input: Self::TaskInput) -> String {
        input
            .problems
            .iter()
            .map(|p| p.solve_in_cephalopodal())
            .sum::<u64>()
            .to_string()
    }
}

pub struct Input {
    problems: Vec<Problem>,
}

struct Problem {
    operands: Vec<Num>,
    operator: Operator,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Num {
    digits: Vec<Option<u64>>,
}

impl Num {
    fn as_num(&self) -> u64 {
        self.digits
            .iter()
            .filter_map(|&d| d)
            .fold(0, |acc, d| acc * 10 + d)
    }
}

#[derive(Clone)]
pub enum Operator {
    Add,
    Multiply,
}

impl super::task::TaskInput for Operator {
    fn from_str(s: &str) -> Self {
        match s {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!("Unknown operator"),
        }
    }
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operator {
            Operator::Add => self.operands.iter().map(|n| n.as_num()).sum(),
            Operator::Multiply => self.operands.iter().map(|n| n.as_num()).product(),
        }
    }

    fn solve_in_cephalopodal(&self) -> u64 {
        let mut operands = vec![];

        let max_index = self
            .operands
            .iter()
            .map(|n| n.digits.len())
            .max()
            .unwrap_or(0);

        for i in 0..max_index {
            let num: u64 = self
                .operands
                .iter()
                .filter_map(|op| op.digits[i])
                .fold(0, |acc, d| acc * 10 + d);
            operands.push(num);
        }

        match self.operator {
            Operator::Add => operands.iter().sum(),
            Operator::Multiply => operands.iter().product(),
        }
    }
}

impl super::task::TaskInput for Input {
    fn from_str(s: &str) -> Self {
        let lines = s.lines().collect::<Vec<_>>();

        let operators = lines.last().unwrap();
        let digit_lines = lines[..lines.len() - 1].to_vec();

        let op_ranges = Self::operator_ranges(operators);

        let problems = op_ranges
            .into_iter()
            .map(|(op, range)| {
                let operands = digit_lines
                    .iter()
                    .map(|line| {
                        let num_str = &line[range.clone()];
                        Self::as_num(num_str)
                    })
                    .collect();

                Problem {
                    operands: operands,
                    operator: op,
                }
            })
            .collect::<Vec<_>>();

        Input { problems }
    }
}

impl Input {
    fn as_num(s: &str) -> Num {
        let digits = s
            .chars()
            .map(|ch| {
                if ch.is_ascii_digit() {
                    Some(ch.to_digit(10).unwrap() as u64)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        Num { digits }
    }

    fn operator_ranges(line: &str) -> Vec<(Operator, std::ops::Range<usize>)> {
        let chars = line.chars().collect::<Vec<_>>();
        let mut start_range = 0;
        let mut operator = match Self::operator_from_char(chars[0]) {
            Some(op) => op,
            None => panic!(
                "Invalid operator line, should start with operator. Got: {}",
                line
            ),
        };

        let mut ranges = vec![];

        for (i, &ch) in chars.iter().enumerate().skip(1) {
            if ch != ' ' {
                let next_operator = Self::operator_from_char(ch)
                    .expect(format!("expected an operator, got {}", ch).as_str());
                ranges.push((operator, start_range..i - 1));
                start_range = i;
                operator = next_operator;
            }
        }

        ranges.push((operator, start_range..chars.len()));
        ranges
    }

    fn operator_from_char(ch: char) -> Option<Operator> {
        match ch {
            '+' => Some(Operator::Add),
            '*' => Some(Operator::Multiply),
            _ => panic!("Invalid operator character: {}", ch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::task::{Task as _, TaskInput};
    use super::*;

    const TEST_INPUT: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn test_part2() {
        let input = Input::from_str(TEST_INPUT);
        let task = Task {};
        let result = task.part2(input);
        assert_eq!(result, "3263827");
    }

    #[test]
    fn test_parse() {
        let input = Input::from_str(TEST_INPUT);
        assert_eq!(input.problems.len(), 4);
        assert_eq!(
            input.problems[0].operands,
            vec![
                Num {
                    digits: vec![Some(1), Some(2), Some(3)],
                },
                Num {
                    digits: vec![None, Some(4), Some(5)],
                },
                Num {
                    digits: vec![None, None, Some(6)],
                },
            ],
        );

        assert_eq!(
            input.problems[2].operands,
            vec![
                Num {
                    digits: vec![None, Some(5), Some(1)],
                },
                Num {
                    digits: vec![Some(3), Some(8), Some(7)],
                },
                Num {
                    digits: vec![Some(2), Some(1), Some(5)],
                },
            ],
        );

        assert_eq!(
            input.problems[3].operands,
            vec![
                Num {
                    digits: vec![Some(6), Some(4), None],
                },
                Num {
                    digits: vec![Some(2), Some(3), None],
                },
                Num {
                    digits: vec![Some(3), Some(1), Some(4)],
                },
            ],
        );
    }
}
