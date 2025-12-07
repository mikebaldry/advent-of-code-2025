use std::collections::{HashMap, HashSet};
struct Grid {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_str(s: &str) -> Self {
        let lines = s.lines().collect::<Vec<&str>>();
        let width = lines.first().unwrap().chars().count();
        let height = lines.len();
        let data = s.replace('\n', "").chars().collect();

        Self {
            data,
            width,
            height,
        }
    }

    fn tachyon_beam_split_count(&self) -> usize {
        let mut emitters = HashSet::new();

        self.track(self.start_x(), 0, &mut emitters);

        emitters.len()
    }

    fn track(&self, x: usize, y: usize, emitters: &mut HashSet<(usize, usize)>) {
        for y in y..self.height - 1 {
            if self.is_splitter(x, y) {
                if emitters.insert((x, y)) {
                    if x > 0 {
                        self.track(x - 1, y, emitters);
                    }
                    if x < self.width - 1 {
                        self.track(x + 1, y, emitters);
                    }
                }
                break;
            }
        }
    }

    fn tachyon_path_count(&self) -> usize {
        let mut count_cache = HashMap::new();

        self.track_path(self.start_x(), 0, &mut count_cache)
    }

    fn track_path(
        &self,
        x: usize,
        y: usize,
        count_cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if y >= self.height {
            return 1;
        }

        if let Some(&count) = count_cache.get(&(x, y)) {
            return count;
        }

        let count = if self.is_splitter(x, y) {
            let mut total = 0;

            if x > 0 {
                total += self.track_path(x - 1, y, count_cache);
            }
            if x < self.width - 1 {
                total += self.track_path(x + 1, y, count_cache);
            }

            total
        } else {
            self.track_path(x, y + 1, count_cache)
        };

        count_cache.insert((x, y), count);
        count
    }

    fn is_splitter(&self, x: usize, y: usize) -> bool {
        self.data[y * self.width + x] == '^'
    }

    fn start_x(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .find(|(_, ch)| *ch == &'S')
            .unwrap()
            .0
    }
}

#[cfg(test)]
mod tests {
    use crate::bench;

    use super::*;

    #[test]
    fn test_day_7_part_1_sample() {
        let grid = Grid::from_str(include_str!("assets/day_7_sample.txt"));

        assert_eq!(grid.tachyon_beam_split_count(), 21);
    }

    #[test]
    fn test_day_7_part_1_real() {
        let grid = Grid::from_str(include_str!("assets/day_7.txt"));

        assert_eq!(grid.tachyon_beam_split_count(), 1507);
    }

    #[test]
    fn test_day_7_part_2_sample() {
        let grid = Grid::from_str(include_str!("assets/day_7_sample.txt"));

        assert_eq!(grid.tachyon_path_count(), 40);
    }

    #[test]
    fn test_day_7_part_2_real() {
        bench(1000, || {
            let grid = Grid::from_str(include_str!("assets/day_7.txt"));

            assert_eq!(grid.tachyon_path_count(), 1537373473728);
        });
    }
}
