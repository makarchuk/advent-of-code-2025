use crate::point::Point;

pub struct Task {}

impl super::task::Task for Task {
    type TaskInput = Tiles;

    fn part1(&self, input: Self::TaskInput) -> String {
        input
            .rectagles()
            .map(|rect| Tiles::area(rect))
            .max()
            .unwrap()
            .to_string()
    }

    fn part2(&self, input: Self::TaskInput) -> String {
        unimplemented!()
    }
}

pub struct Tiles {
    tiles: Vec<Point>,
}

impl super::task::TaskInput for Tiles {
    fn from_str(s: &str) -> Self {
        let tiles = s
            .lines()
            .map(|l| {
                let coords = l
                    .split(",")
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i64>>();
                Point {
                    x: coords[0],
                    y: coords[1],
                }
            })
            .collect();
        Tiles { tiles }
    }
}

impl Tiles {
    fn rectagles(&self) -> impl Iterator<Item = (Point, Point)> {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(i, p1)| self.tiles.iter().skip(i + 1).map(move |p2| (*p1, *p2)))
    }

    fn area(rect: (Point, Point)) -> i64 {
        let width = (rect.0.x - rect.1.x).abs() + 1;
        let height = (rect.0.y - rect.1.y).abs() + 1;
        width * height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area() {
        let p1 = Point { x: 2, y: 5 };
        let p2 = Point { x: 11, y: 1 };

        assert_eq!(Tiles::area((p1, p2)), 50);
    }
}
