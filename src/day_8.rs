use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }
}

impl Point {
    fn from_str(s: &str) -> Self {
        let mut split = s.split(",").map(|n| n.parse::<i64>().unwrap());

        Self {
            x: split.next().unwrap(),
            y: split.next().unwrap(),
            z: split.next().unwrap(),
        }
    }

    fn distance(&self, other: &Point) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}

#[derive(Debug)]
struct Space {
    points: Vec<Point>,
    closest_points: VecDeque<(Point, Point)>,
}

impl Space {
    fn from_str(s: &str) -> Self {
        let points: Vec<_> = s.lines().map(Point::from_str).collect();
        let mut sorted_closest_points: BTreeMap<i64, Vec<(Point, Point)>> = BTreeMap::new();
        let mut processed: HashSet<(Point, Point)> = HashSet::new();

        for &p1 in points.iter() {
            for &p2 in points.iter() {
                if p1 == p2 || processed.contains(&(p2, p1)) {
                    continue;
                }

                processed.insert((p1, p2));

                sorted_closest_points
                    .entry(p1.distance(&p2))
                    .or_default()
                    .push((p1, p2));
            }
        }

        let mut closest_points = VecDeque::new();

        for (_, pairs) in sorted_closest_points.iter() {
            for &(p1, p2) in pairs.iter() {
                closest_points.push_back((p1, p2));
            }
        }

        Self {
            points,
            closest_points,
        }
    }

    fn part_1(&self, iters: usize) -> u32 {
        let mut circuit_lookup = HashMap::new();

        for (i, p) in self.points.iter().enumerate() {
            circuit_lookup.insert(p, i);
        }

        for (p1, p2) in self.closest_points.iter().take(iters) {
            let p1_circuit = *circuit_lookup.get(p1).unwrap();
            let p2_circuit = *circuit_lookup.get(p2).unwrap();

            if p1_circuit != p2_circuit {
                for (_, circuit) in circuit_lookup.iter_mut() {
                    if *circuit == p2_circuit {
                        *circuit = p1_circuit;
                    }
                }
            }
        }

        let mut circuits: HashMap<usize, u32> = HashMap::new();

        for (_, c) in circuit_lookup.iter() {
            circuits.entry(*c).and_modify(|v| *v += 1).or_insert(1);
        }

        let mut sizes: Vec<u32> = circuits.into_values().collect();
        sizes.sort_by(|a, b| b.cmp(a));
        sizes.iter().take(3).product()
    }

    fn part_2(&self) -> u64 {
        let mut circuit_lookup = HashMap::new();
        let mut total_circuits = self.points.len();

        for (i, p) in self.points.iter().enumerate() {
            circuit_lookup.insert(p, i);
        }

        for (p1, p2) in self.closest_points.iter() {
            let p1_circuit = *circuit_lookup.get(p1).unwrap();
            let p2_circuit = *circuit_lookup.get(p2).unwrap();

            if p1_circuit != p2_circuit {
                for (_, circuit) in circuit_lookup.iter_mut() {
                    if *circuit == p2_circuit {
                        *circuit = p1_circuit;
                    }
                }

                total_circuits -= 1;
                if total_circuits == 1 {
                    return p1.x as u64 * p2.x as u64;
                }
            }
        }

        panic!("the points never formed one circuit!");
    }
}

#[cfg(test)]
mod tests {
    use crate::bench;

    use super::*;

    #[test]
    fn test_day_8_part_1_sample() {
        let space = Space::from_str(include_str!("assets/day_8_sample.txt"));

        assert_eq!(space.part_1(10), 40);
    }

    #[test]
    fn test_day_8_part_1_real() {
        bench(10, || {
            let space = Space::from_str(include_str!("assets/day_8.txt"));

            assert_eq!(space.part_1(1000), 67488);
        });
    }

    #[test]
    fn test_day_8_part_2_sample() {
        let space = Space::from_str(include_str!("assets/day_8_sample.txt"));

        assert_eq!(space.part_2(), 25272);
    }

    #[test]
    fn test_day_8_part_2_real() {
        bench(10, || {
            let space = Space::from_str(include_str!("assets/day_8.txt"));

            assert_eq!(space.part_2(), 3767453340);
        });
    }
}
