use std::collections::HashMap;
use std::collections::HashSet;

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
        let solver = Part2Solver::new(&input);
        solver.solve().to_string()
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

struct Part2Solver {
    tiles: Vec<Point>,
    // map x -> list of ys. ys are sorted
    horizontal_borders: HashMap<i64, Vec<Border>>,
    // map y -> list of xs. xs are sorted
    vertical_borders: HashMap<i64, Vec<Border>>,
}

#[derive(Debug, Clone, Copy)]
struct Border {
    coordinate: i64,
    border_type: BorderType,
}

#[derive(Debug, Clone, Copy)]
enum BorderType {
    // vertices and edges are treated differently because
    // going through the vertice doesn't change insideness
    // you start sliding across the edge
    // insideness changes every TWO vertices or ONE edge
    Red,   //Red borders are vertices of the figure
    Green, //Green borders are intersections with edges of the figure
}

impl Part2Solver {
    fn new(tiles: &Tiles) -> Self {
        let mut horizontal_borders = HashMap::new();
        let mut vertical_borders = HashMap::new();

        for i in 0..tiles.tiles.len() {
            let start = tiles.tiles[i];
            let end = tiles.tiles[(i + 1) % tiles.tiles.len()];
            if start.x == end.x {
                for p in start.line(&end) {
                    let border_type = if p == start || p == end {
                        BorderType::Red
                    } else {
                        BorderType::Green
                    };
                    vertical_borders
                        .entry(p.y)
                        .or_insert_with(Vec::new)
                        .push(Border {
                            coordinate: p.x,
                            border_type,
                        });
                }
            } else if start.y == end.y {
                for p in start.line(&end) {
                    let border_type = if p == start || p == end {
                        BorderType::Red
                    } else {
                        BorderType::Green
                    };
                    horizontal_borders
                        .entry(p.x)
                        .or_insert_with(Vec::new)
                        .push(Border {
                            coordinate: p.y,
                            border_type,
                        });
                }
            } else {
                unreachable!("only horizontal and vertical lines are supported");
            }
        }

        horizontal_borders
            .values_mut()
            .for_each(|ys| ys.sort_by_key(|b| b.coordinate));

        vertical_borders
            .values_mut()
            .for_each(|xs| xs.sort_by_key(|b| b.coordinate));

        Part2Solver {
            tiles: tiles.tiles.clone(),
            horizontal_borders,
            vertical_borders,
        }
    }

    pub fn solve(&self) -> i64 {
        let mut max_area = 0;

        let mut counter = 0;
        let total = self.tiles.len() * (self.tiles.len() - 1) / 2;

        for (i, p1) in self.tiles.iter().enumerate() {
            for p2 in self.tiles.iter().skip(i + 1) {
                counter += 1;
                if counter % 1000 == 0 {
                    println!("Checking rectangle {}/{}", counter, total);
                }

                let (a, b, c, d) = (
                    p1.clone(),
                    Point::new(p1.x, p2.y),
                    p2.clone(),
                    Point::new(p2.x, p1.y),
                );

                let mut rectangle_edge = a
                    .line(&b)
                    .chain(b.line(&c))
                    .chain(c.line(&d))
                    .chain(d.line(&a));

                // it's enought to ensure that line edges are not colourless
                // No need to check insides

                if rectangle_edge.any(|p| !self.is_point_inside(&p)) {
                    continue;
                }

                let area = Tiles::area((a, c));
                max_area = max_area.max(area);
            }
        }

        max_area
    }

    fn is_point_inside(&self, point: &Point) -> bool {
        match self.horizontal_borders.get(&point.x) {
            Some(borders) => {
                // dbg!("horizontal", &borders, point);
                if !Self::check_border(&borders, point.y) {
                    return false;
                }
            }
            None => {
                return false;
            }
        };

        match self.vertical_borders.get(&point.y) {
            Some(borders) => {
                // dbg!("vertical", &borders, point);
                if !Self::check_border(&borders, point.x) {
                    return false;
                }
            }
            None => {
                return false;
            }
        };

        return true;
    }

    fn check_border(borders: &Vec<Border>, coordinate: i64) -> bool {
        {
            // if point is located between two borders
            // in such a way that tile before has an even index
            // that means we're inside
            if coordinate < borders[0].coordinate
                || coordinate > borders[borders.len() - 1].coordinate
            {
                return false;
            }

            let mut state = ScanningState::Outside;
            for border in borders {
                if coordinate < border.coordinate {
                    break;
                }
                if coordinate == border.coordinate {
                    return true;
                }
                match (&state, &border.border_type) {
                    (ScanningState::Outside, BorderType::Red) => {
                        state = ScanningState::AlongTheEdge
                    }
                    (ScanningState::Outside, BorderType::Green) => state = ScanningState::Inside,
                    (ScanningState::AlongTheEdge, BorderType::Red) => state = ScanningState::Inside,
                    (ScanningState::AlongTheEdge, BorderType::Green) => {
                        unreachable!(
                            "Edge should end with red tile. Got green: line: {:?}, point: {:?}",
                            borders, coordinate
                        )
                    }
                    (ScanningState::Inside, BorderType::Red) => state = ScanningState::AlongTheEdge,
                    (ScanningState::Inside, BorderType::Green) => state = ScanningState::Outside,
                }
            }

            match state {
                ScanningState::Inside | ScanningState::AlongTheEdge => true,
                ScanningState::Outside => false,
            }
        }
    }
}

enum ScanningState {
    Outside,
    AlongTheEdge,
    Inside,
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

    #[test]
    fn test_line() {
        let start = Point { x: 49047, y: 1795 };
        let end = Point { x: 49047, y: 1793 };

        let line_points: Vec<Point> = start.line(&end).collect();
        assert!(line_points.contains(&Point { x: 49047, y: 1794 }));
    }

    #[test]
    fn test_solver() {
        // ..............
        // .......#XXX#..
        // .......XXXXX..
        // ..#XXXX#XXXX..
        // ..XXXXXXXXXX..
        // ..#XXXXXX#XX..
        // .........XXX..
        // .........#X#..
        // ..............

        let tiles = Tiles {
            tiles: vec![
                Point::new(7, 1),
                Point::new(11, 1),
                Point::new(11, 7),
                Point::new(9, 7),
                Point::new(9, 5),
                Point::new(2, 5),
                Point::new(2, 3),
                Point::new(7, 3),
            ],
        };

        let solver = Part2Solver::new(&tiles);

        assert_eq!(solver.is_point_inside(&Point::new(7, 4)), true);
        assert_eq!(solver.is_point_inside(&Point::new(3, 5)), true);

        {
            let (p1, p2) = (Point::new(2, 3), Point::new(9, 5));
            let (a, b, c, d) = (
                p1.clone(),
                Point::new(p1.x, p2.y),
                p2.clone(),
                Point::new(p2.x, p1.y),
            );

            let rectangle_edge = a
                .line(&b)
                .chain(b.line(&c))
                .chain(c.line(&d))
                .chain(d.line(&a));

            for p in rectangle_edge {
                assert_eq!(solver.is_point_inside(&p), true, "Failed at point {:?}", p);
            }
        }

        let area = solver.solve();
        assert_eq!(area, 24);

        assert_eq!(solver.horizontal_borders.len(), 10);

        assert_eq!(solver.is_point_inside(&Point::new(0, 0)), false);
        assert_eq!(solver.is_point_inside(&Point::new(11, 8)), false);

        for p in tiles.tiles.iter() {
            assert_eq!(solver.is_point_inside(p), true);
        }

        for x in 2..=5 {
            assert_eq!(solver.is_point_inside(&Point::new(x, 0)), false);
            assert_eq!(solver.is_point_inside(&Point::new(x, 1)), false);
            assert_eq!(solver.is_point_inside(&Point::new(x, 2)), false);
            assert_eq!(solver.is_point_inside(&Point::new(x, 3)), true);
            assert_eq!(solver.is_point_inside(&Point::new(x, 4)), true);
            assert_eq!(solver.is_point_inside(&Point::new(x, 5)), true);
            assert_eq!(solver.is_point_inside(&Point::new(x, 6)), false);
            assert_eq!(solver.is_point_inside(&Point::new(x, 7)), false);
            assert_eq!(solver.is_point_inside(&Point::new(x, 8)), false);
            assert_eq!(solver.is_point_inside(&Point::new(x, 9)), false);
            assert_eq!(solver.is_point_inside(&Point::new(x, 10)), false);
        }

        assert_eq!(solver.is_point_inside(&Point::new(7, 0)), false);
        assert_eq!(solver.is_point_inside(&Point::new(7, 1)), true);
        assert_eq!(solver.is_point_inside(&Point::new(7, 2)), true);
        assert_eq!(solver.is_point_inside(&Point::new(7, 3)), true);
        assert_eq!(solver.is_point_inside(&Point::new(7, 4)), true);
        assert_eq!(solver.is_point_inside(&Point::new(7, 5)), true);
        assert_eq!(solver.is_point_inside(&Point::new(7, 6)), false);
        assert_eq!(solver.is_point_inside(&Point::new(7, 7)), false);

        assert_eq!(solver.is_point_inside(&Point::new(0, 4)), false);
        assert_eq!(solver.is_point_inside(&Point::new(1, 4)), false);
        assert_eq!(solver.is_point_inside(&Point::new(2, 4)), true);
        assert_eq!(solver.is_point_inside(&Point::new(3, 4)), true);
        assert_eq!(solver.is_point_inside(&Point::new(4, 4)), true);
        assert_eq!(solver.is_point_inside(&Point::new(5, 4)), true);
        assert_eq!(solver.is_point_inside(&Point::new(6, 4)), true);
        assert_eq!(solver.is_point_inside(&Point::new(7, 4)), true);
        assert_eq!(solver.is_point_inside(&Point::new(8, 4)), true);
        assert_eq!(solver.is_point_inside(&Point::new(9, 4)), true);
        assert_eq!(solver.is_point_inside(&Point::new(10, 4)), true);
        assert_eq!(solver.is_point_inside(&Point::new(11, 4)), true);
        assert_eq!(solver.is_point_inside(&Point::new(12, 4)), false);
        assert_eq!(solver.is_point_inside(&Point::new(13, 4)), false);
    }
}
