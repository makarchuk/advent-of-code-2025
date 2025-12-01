use crate::tasks::task;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Entry {
    direction: Direction,
    distance: u32,
}

pub struct Input {
    entries: Vec<Entry>,
}

impl task::TaskInput for Input {
    fn from_str(s: &str) -> Self {
        let entries = s
            .lines()
            .map(|line| {
                let (dir, dist) = line.split_at(1);
                let direction = match dir {
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!("Invalid direction"),
                };
                let distance = dist.parse::<u32>().unwrap();
                Entry {
                    direction,
                    distance,
                }
            })
            .collect();
        Input { entries }
    }
}

pub struct Task {}

impl task::Task for Task {
    type TaskInput = Input;

    fn part1(&self, input: Self::TaskInput) -> String {
        let mut zeroes_counter = 0;

        let mut position = 50;
        for entry in input.entries {
            let new_raw_position = self.next_raw_position(position, &entry);
            (position, _) = adjust_position(position as i64, new_raw_position);
            if position == 0 {
                zeroes_counter += 1;
            }
        }
        zeroes_counter.to_string()
    }

    fn part2(&self, input: Self::TaskInput) -> String {
        let mut zero_clicks_total = 0;

        let mut position = 50;
        for entry in input.entries {
            let raw_position = self.next_raw_position(position, &entry);
            let (new_position, zero_clicks) = adjust_position(position as i64, raw_position);
            position = new_position;
            if position == 0 {
                zero_clicks_total += 1;
            }
            zero_clicks_total += zero_clicks;
        }
        zero_clicks_total.to_string()
    }
}

impl Task {
    fn next_raw_position(&self, current_position: u32, entry: &Entry) -> i64 {
        match entry.direction {
            Direction::Left => current_position as i64 - entry.distance as i64,
            Direction::Right => current_position as i64 + entry.distance as i64,
        }
    }
}

fn adjust_position(old: i64, new: i64) -> (u32, u32) {
    let mut zero_clicks = 0;
    if new < 0 {
        let mut position = new;
        while position < 0 {
            position += 100;
            zero_clicks += 1;
        }
        if old == 0 {
            zero_clicks -= 1;
        }
        (position as u32, zero_clicks)
    } else if new > 99 {
        let mut position = new;
        while position > 99 {
            position = position - 100;
            zero_clicks += 1;
        }
        if position == 0 && old != 0 {
            zero_clicks -= 1;
        }
        (position as u32, zero_clicks)
    } else {
        (new as u32, zero_clicks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust_position() {
        let tests = vec![
            ((0, 210), (10, 2)),
            ((0, -10), (90, 0)),
            ((50, 60), (60, 0)),
            ((99, 1), (1, 0)),
            ((0, 100), (0, 1)),
            ((95, 155), (55, 1)),
            ((50, 1050), (50, 10)),
            ((52, 100), (0, 0)),
        ];
        for ((old, new), expected) in tests {
            let result = adjust_position(old, new);
            assert_eq!(result, expected, "Failed for old: {}, new: {}", old, new);
        }
    }
}
