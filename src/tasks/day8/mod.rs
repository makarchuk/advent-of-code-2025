use std::collections::HashSet;

pub struct Task {}

impl super::task::Task for Task {
    type TaskInput = Points;

    fn part1(&self, input: Self::TaskInput) -> String {
        let shortest_distances = Task::get_n_shortest_distances(input, 1000);

        let mut grouper = PointGroupper::new();

        for &((p1, p2), _dist) in shortest_distances.iter() {
            grouper.add_point_pair(p1, p2);
        }

        grouper
            .biggest_groups(3)
            .iter()
            .map(|s| s.len() as u64)
            .product::<u64>()
            .to_string()
    }

    fn part2(&self, input: Self::TaskInput) -> String {
        let distances = Task::get_all_distances(&input);
        let mut grouper = PointGroupper::new();

        for &((p1, p2), _dist) in distances.iter() {
            grouper.add_point_pair(p1, p2);

            if grouper.groups.len() == 1 && grouper.groups[0].len() == input.points.len() {
                // p1 to p2 is the last match
                return (input.points[p1].x * input.points[p2].x).to_string();
            }
        }

        unreachable!("never managed to connect all the points")
    }
}

impl Task {
    fn get_all_distances(input: &Points) -> Vec<((usize, usize), u64)> {
        let mut distances: Vec<((usize, usize), u64)> = Vec::new();

        for i in 0..input.points.len() {
            for j in (i + 1)..input.points.len() {
                let dist = input.points[i].distance_squared(&input.points[j]);
                distances.push(((i, j), dist));
            }
        }

        distances.sort_by_key(|&(_, dist)| dist);
        distances
    }

    fn get_n_shortest_distances(input: Points, n: usize) -> Vec<((usize, usize), u64)> {
        let mut distances = Task::get_all_distances(&input);
        distances.truncate(n);
        distances
    }
}

struct PointGroupper {
    groups: Vec<HashSet<usize>>,
}

impl PointGroupper {
    fn new() -> Self {
        PointGroupper { groups: vec![] }
    }

    fn add_point_pair(&mut self, p1: usize, p2: usize) {
        let mut group_indices = vec![];
        for (idx, group) in self.groups.iter().enumerate() {
            if group.contains(&p1) || group.contains(&p2) {
                group_indices.push(idx);
            }
        }

        if group_indices.is_empty() {
            let mut new_group = HashSet::new();
            new_group.insert(p1);
            new_group.insert(p2);
            self.groups.push(new_group);
        } else {
            let first_index = group_indices[0];
            self.groups[first_index].insert(p1);
            self.groups[first_index].insert(p2);

            for &idx in group_indices.iter().skip(1).rev() {
                let to_merge = self.groups.remove(idx);
                for point in to_merge {
                    self.groups[first_index].insert(point);
                }
            }
        }
    }

    fn biggest_groups(&self, n: usize) -> Vec<&HashSet<usize>> {
        let mut groups: Vec<&HashSet<usize>> = self.groups.iter().collect();
        groups.sort_by(|a, b| b.len().cmp(&a.len()));
        groups.truncate(n);
        groups
    }
}

fn group_points(points: Vec<(usize, usize)>) -> Vec<HashSet<usize>> {
    let mut groups: Vec<HashSet<usize>> = vec![];

    for (p1, p2) in points {
        let mut group_indices = vec![];
        for (idx, group) in groups.iter().enumerate() {
            if group.contains(&p1) || group.contains(&p2) {
                group_indices.push(idx);
            }
        }

        if group_indices.is_empty() {
            let mut new_group = HashSet::new();
            new_group.insert(p1);
            new_group.insert(p2);
            groups.push(new_group);
        } else {
            let first_index = group_indices[0];
            groups[first_index].insert(p1);
            groups[first_index].insert(p2);

            for &idx in group_indices.iter().skip(1).rev() {
                let to_merge = groups.remove(idx);
                for point in to_merge {
                    groups[first_index].insert(point);
                }
            }
        }
    }

    groups
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    fn new(x: u64, y: u64, z: u64) -> Self {
        Point { x, y, z }
    }
}

pub struct Points {
    points: Vec<Point>,
}

impl super::task::TaskInput for Points {
    fn from_str(input: &str) -> Self {
        let points = input
            .lines()
            .map(|line| {
                let coords: Vec<u64> = line.split(',').map(|num| num.parse().unwrap()).collect();
                Point::new(coords[0], coords[1], coords[2])
            })
            .collect();
        Points { points }
    }
}

impl Point {
    fn distance_squared(&self, other: &Point) -> u64 {
        let dx = self.x as i64 - other.x as i64;
        let dy = self.y as i64 - other.y as i64;
        let dz = self.z as i64 - other.z as i64;
        (dx * dx + dy * dy + dz * dz) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distances() {
        let points = Points {
            points: vec![
                Point::new(162, 817, 812), // 0
                Point::new(57, 618, 57),   // 1
                Point::new(906, 360, 560), // 2
                Point::new(592, 479, 940), // 3
                Point::new(352, 342, 300), // 4
                Point::new(466, 668, 158), // 5
                Point::new(542, 29, 236),  // 6
                Point::new(431, 825, 988), // 7
                Point::new(739, 650, 466), // 8
                Point::new(52, 470, 668),  // 9
                Point::new(216, 146, 977), // 10
                Point::new(819, 987, 18),  // 11
                Point::new(117, 168, 530), // 12
                Point::new(805, 96, 715),  // 13
                Point::new(346, 949, 466), // 14
                Point::new(970, 615, 88),  // 15
                Point::new(941, 993, 340), // 16
                Point::new(862, 61, 35),   // 17
                Point::new(984, 92, 344),  // 18
                Point::new(425, 690, 689), // 19
            ],
        };
        let top_distances = Task::get_n_shortest_distances(points, 10);

        assert_eq!(top_distances[0].0, (0, 19));
        assert_eq!(top_distances[1].0, (0, 7));
        assert_eq!(top_distances[2].0, (2, 13));

        let groups = group_points(
            top_distances
                .iter()
                .map(|((p1, p2), _dist)| (*p1, *p2))
                .collect(),
        );

        assert_eq!(groups.len(), 4);
        assert_eq!(groups.iter().map(|s| s.len()).max().unwrap(), 5);
    }
}
