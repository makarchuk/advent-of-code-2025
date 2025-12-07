use std::collections::{HashMap, HashSet};

use super::task::TaskInput;
use crate::grid::Grid;
use crate::point::Point;

pub struct Task {}

impl super::task::Task for Task {
    type TaskInput = Field;

    fn part1(&self, input: Self::TaskInput) -> String {
        let mut field = input;

        for _ in 0..field.grid.size().y {
            field = field.beam_step();
        }

        field.splits_count.to_string()
    }

    fn part2(&self, input: Self::TaskInput) -> String {
        let field = input;

        let mut timelines: HashMap<Point, u64> =
            field.beam_locations.iter().map(|&loc| (loc, 1)).collect();

        for _ in 0..field.grid.size().y - 1 {
            let mut new_timelines = HashMap::new();

            for (&loc, &count) in &timelines {
                let possible_locations = field.possible_beam_locations(loc);
                for new_loc in possible_locations {
                    *new_timelines.entry(new_loc).or_insert(0) += count;
                }
            }

            timelines = new_timelines;
        }

        timelines.values().sum::<u64>().to_string()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Splitter,
}

pub struct Field {
    grid: Grid<Cell>,
    beam_locations: HashSet<Point>,
    splits_count: u64,
}

impl TaskInput for Field {
    fn from_str(input: &str) -> Self {
        let input_grid: Grid<InputCell> = Grid::from_str(input);

        let mut beam_locations = HashSet::new();
        let mut grid = vec![];

        for (p, cell) in input_grid.iter() {
            while p.y >= grid.len() as i64 {
                grid.push(vec![Cell::Empty; input_grid.size().x as usize]);
            }

            match cell {
                InputCell::Cell(c) => {
                    grid[p.y as usize][p.x as usize] = c;
                }
                InputCell::Start => {
                    grid[p.y as usize][p.x as usize] = Cell::Empty;
                    beam_locations.insert(Point { x: p.x, y: p.y });
                }
            }
        }

        Field {
            grid: Grid::new(grid),
            beam_locations,
            splits_count: 0,
        }
    }
}

impl Field {
    fn possible_beam_locations(&self, point: Point) -> HashSet<Point> {
        let bellow = Point {
            x: point.x,
            y: point.y + 1,
        };
        let mut locations = HashSet::new();
        match self.grid.get(bellow) {
            Some(Cell::Empty) => {
                locations.insert(bellow);
            }
            Some(Cell::Splitter) => {
                if point.x > 0 {
                    locations.insert(Point {
                        x: point.x - 1,
                        y: point.y + 1,
                    });
                }
                if point.x < self.grid.size().x - 1 {
                    locations.insert(Point {
                        x: point.x + 1,
                        y: point.y + 1,
                    });
                }
            }
            None => {}
        };

        locations
    }

    fn beam_step(&self) -> Self {
        let mut new_beam_locations = HashSet::new();
        let mut new_splits_count = self.splits_count;

        for &loc in &self.beam_locations {
            let below = Point {
                x: loc.x,
                y: loc.y + 1,
            };

            match self.grid.get(below) {
                Some(Cell::Empty) => {
                    new_beam_locations.insert(below);
                }
                Some(Cell::Splitter) => {
                    new_splits_count += 1;
                    if loc.x > 0 {
                        new_beam_locations.insert(Point {
                            x: loc.x - 1,
                            y: loc.y + 1,
                        });
                    }
                    if loc.x < self.grid.size().x - 1 {
                        new_beam_locations.insert(Point {
                            x: loc.x + 1,
                            y: loc.y + 1,
                        });
                    }
                }
                None => {}
            }
        }

        Field {
            grid: self.grid.clone(),
            beam_locations: new_beam_locations,
            splits_count: new_splits_count,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum InputCell {
    Cell(Cell),
    Start,
}

impl TaskInput for InputCell {
    fn from_str(s: &str) -> Self {
        match s {
            "." => Self::Cell(Cell::Empty),
            "^" => Self::Cell(Cell::Splitter),
            "S" => Self::Start,
            _ => panic!("Invalid input value: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::task::Task as _;

    const TEST_INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn test_part1() {
        let task = Task {};
        let input = Field::from_str(TEST_INPUT);
        let result = task.part1(input);
        assert_eq!(result, "21");
    }

    #[test]
    fn test_part2() {
        let task = Task {};
        let input = Field::from_str(TEST_INPUT);
        let result = task.part2(input);
        assert_eq!(result, "40");
    }
}
