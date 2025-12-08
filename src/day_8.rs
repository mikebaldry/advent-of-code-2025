use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
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
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        dx * dx + dy * dy + dz * dz
    }
}

struct Space {
    points: Vec<Point>,
    closest_points: Vec<(i64, Point, Point)>,
}

impl Space {
    fn from_str(s: &str) -> Self {
        let points: Vec<_> = s.lines().map(Point::from_str).collect();
        let points_ref = &points;

        let mut closest_points: Vec<_> = (0..points_ref.len())
            .flat_map(|a| {
                (a + 1..points_ref.len()).map(move |b| {
                    let p1 = points_ref[a];
                    let p2 = points_ref[b];

                    (p1.distance(&p2), p1, p2)
                })
            })
            .collect();
        closest_points.sort_unstable_by_key(|(d, _, _)| *d);

        Self {
            points,
            closest_points,
        }
    }

    fn part_1(&self, iters: usize) -> u32 {
        let mut circuit_lookup: HashMap<Point, usize> = self
            .points
            .iter()
            .enumerate()
            .map(|(i, p)| (*p, i))
            .collect();

        for (_, p1, p2) in self.closest_points.iter().take(iters) {
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
        let mut circuit_lookup: HashMap<Point, usize> = self
            .points
            .iter()
            .enumerate()
            .map(|(i, p)| (*p, i))
            .collect();
        let mut total_circuits = self.points.len();

        for (_, p1, p2) in self.closest_points.iter() {
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
        bench(100, || {
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
        bench(100, || {
            let space = Space::from_str(include_str!("assets/day_8.txt"));

            assert_eq!(space.part_2(), 3767453340);
        });
    }
}
