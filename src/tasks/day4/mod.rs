pub struct Task {}

impl super::task::Task for Task {
    type TaskInput = crate::grid::Grid<Cell>;

    fn part1(&self, input: Self::TaskInput) -> String {
        input
            .iter()
            .filter(|(point, cell)| {
                if *cell != Cell::Roll {
                    return false;
                }

                is_accessible(&input, *point)
            })
            .count()
            .to_string()
    }

    fn part2(&self, input: Self::TaskInput) -> String {
        let mut grid = input;
        let mut total_removed = 0;
        loop {
            let mut current_pass_accessible = 0;
            for point in grid.size().iter_inside() {
                let cell = match grid.get(point) {
                    Some(cell) => cell,
                    None => continue,
                };
                if cell != Cell::Roll {
                    continue;
                }

                if is_accessible(&grid, point) {
                    grid.replace(point, Cell::Empty);
                    current_pass_accessible += 1;
                }
            }
            if current_pass_accessible == 0 {
                break;
            }
            total_removed += current_pass_accessible;
        }
        total_removed.to_string()
    }
}

pub fn is_accessible(grid: &crate::grid::Grid<Cell>, point: crate::point::Point) -> bool {
    point
        .neighbours()
        .filter_map(|neighbour| match grid.get(neighbour) {
            Some(Cell::Roll) => Some(()),
            _ => None,
        })
        .count()
        < 4
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Roll,
}

impl super::task::TaskInput for Cell {
    fn from_str(s: &str) -> Self {
        match s {
            "." => Cell::Empty,
            "@" => Cell::Roll,
            _ => panic!("Unexpected cell character: {}", s),
        }
    }
}
