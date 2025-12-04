use std::ops::Add;

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
}
