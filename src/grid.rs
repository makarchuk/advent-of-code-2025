use crate::{point::Point, tasks::task::TaskInput};

#[derive(Debug, Clone)]
pub struct Grid<T: Copy> {
    width: i64,
    height: i64,
    cells: Vec<Vec<T>>,
}

impl<T: Copy> Grid<T> {
    pub fn new(values: Vec<Vec<T>>) -> Self {
        assert!(!values.is_empty(), "Grid cannot be empty");
        let width = values[0].len() as i64;
        for row in &values {
            assert!(
                row.len() as i64 == width,
                "All rows in the grid must have the same length"
            );
        }

        let height = values.len() as i64;

        Grid {
            width,
            height,
            cells: values,
        }
    }

    pub fn get(&self, point: Point) -> Option<T> {
        if point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height {
            Some(self.cells[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    pub fn replace(&mut self, point: Point, value: T) {
        if point.x >= 0 && point.x < self.width && point.y >= 0 && point.y < self.height {
            self.cells[point.y as usize][point.x as usize] = value;
        }
    }

    pub fn size(&self) -> Point {
        Point {
            x: self.width,
            y: self.height,
        }
    }
}

pub struct GridIterator<'a, T: Copy> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<T: Copy> Grid<T> {
    pub fn iter(&self) -> GridIterator<'_, T> {
        GridIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

impl<'a, T: Copy> Iterator for GridIterator<'a, T> {
    type Item = (Point, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y as i64 >= self.grid.height {
            return None;
        }

        let point = Point {
            x: self.x as i64,
            y: self.y as i64,
        };
        let value = self.grid.cells[self.y][self.x];

        self.x += 1;
        if self.x as i64 >= self.grid.width {
            self.x = 0;
            self.y += 1;
        }

        Some((point, value))
    }
}

impl<T: TaskInput + Copy> TaskInput for Grid<T> {
    fn from_str(s: &str) -> Self {
        let rows: Vec<Vec<T>> = s
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|cell_str| T::from_str(&cell_str.to_string()))
                    .collect()
            })
            .collect();

        Grid::new(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let grid = Grid::new(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
            vec![10, 11, 12],
        ]);

        assert_eq!(grid.iter().count(), 12);
        assert_eq!(grid.iter().nth(3).unwrap(), (Point { x: 0, y: 1 }, 4));
        assert_eq!(grid.iter().nth(7).unwrap(), (Point { x: 1, y: 2 }, 8));
    }
}
