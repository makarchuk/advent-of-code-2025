use std::{array::IntoIter, ops::Add};

const NEIGHBOURS: [Point; 8] = [
    Point { x: -1, y: -1 },
    Point { x: 0, y: -1 },
    Point { x: 1, y: -1 },
    Point { x: -1, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: -1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: 1, y: 1 },
];

const ORTHOGONAL_NEIGHBOURS: [Point; 4] = [
    Point { x: 0, y: -1 },
    Point { x: -1, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    pub fn direct_neighbours(&self) -> impl Iterator<Item = Point> {
        ORTHOGONAL_NEIGHBOURS.iter().map(|offset| *self + *offset)
    }

    pub fn neighbours(&self) -> impl Iterator<Item = Point> {
        NEIGHBOURS.iter().map(|offset| *self + *offset)
    }

    // iter_inside accepts a point and iterates over all points from (0,0) to (x-1,y-1)
    pub fn iter_inside(&self) -> impl Iterator<Item = Point> {
        (0..self.y).flat_map(move |y| {
            (0..self.x).map(move |x| Point {
                x: x as i64,
                y: y as i64,
            })
        })
    }

    pub fn line(&self, other: &Point) -> impl Iterator<Item = Point> {
        let dx = other.x - self.x;
        let dy = other.y - self.y;

        assert!(
            dx * dy == 0,
            "Only horizontal or vertical lines are supported"
        );

        // there's probably a better way to do this with iterators and types magic
        // It's too late now
        if dx == 0 {
            (self.y.min(other.y)..=self.y.max(other.y))
                .into_iter()
                .map(|y| Point { x: self.x, y })
                .collect::<Vec<Point>>()
                .into_iter()
        } else {
            (self.x.min(other.x)..=self.x.max(other.x))
                .into_iter()
                .map(|x| Point { x, y: self.y })
                .collect::<Vec<Point>>()
                .into_iter()
        }
    }
}
