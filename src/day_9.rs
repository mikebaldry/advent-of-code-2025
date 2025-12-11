use std::cmp::Reverse;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}

impl Point {
    fn from_str(s: &str) -> Self {
        let mut split = s.split(",").map(|n| n.parse::<i64>().unwrap());

        Self {
            x: split.next().unwrap(),
            y: split.next().unwrap(),
        }
    }

    fn area(&self, other: &Point) -> u64 {
        let width = self.x.abs_diff(other.x) + 1;
        let height = self.y.abs_diff(other.y) + 1;

        width * height
    }
}

#[derive(Debug)]
struct Space {
    points: Vec<Point>,
    furthest_points: Vec<(u64, Point, Point)>,
}

impl Space {
    fn from_str(s: &str) -> Self {
        let points: Vec<_> = s.lines().map(Point::from_str).collect();
        let points_ref = &points;

        let mut furthest_points: Vec<_> = (0..points_ref.len())
            .flat_map(|a| {
                (a + 1..points_ref.len()).map(move |b| {
                    let p1 = points_ref[a];
                    let p2 = points_ref[b];

                    (p1.area(&p2), p1, p2)
                })
            })
            .collect();
        furthest_points.sort_unstable_by_key(|(a, _, _)| Reverse(*a));

        Self {
            points,
            furthest_points,
        }
    }

    fn part_1(&self) -> u64 {
        let (area, _, _) = self.furthest_points.first().unwrap();

        *area
    }
}

#[cfg(test)]
mod tests {
    use crate::bench;

    use super::*;

    #[test]
    fn test_day_9_part_1_sample() {
        let space = Space::from_str(include_str!("assets/day_9_sample.txt"));

        assert_eq!(space.part_1(), 50);
    }

    #[test]
    fn test_day_9_part_1_real() {
        bench(100, || {
            let space = Space::from_str(include_str!("assets/day_9.txt"));

            assert_eq!(space.part_1(), 4777409595);
        });
    }
}
