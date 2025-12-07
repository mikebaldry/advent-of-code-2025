use std::collections::VecDeque;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

struct Grid {
    data: Vec<bool>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_str(s: &str) -> Self {
        let lines = s.lines().collect::<Vec<&str>>();
        let width = lines.first().unwrap().chars().count();
        let height = lines.len();
        let data = s.replace('\n', "").chars().map(|c| c == '@').collect();

        Self {
            data,
            width,
            height,
        }
    }

    fn calculate_accessible_rolls(&self) -> usize {
        let mut result = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(around) = self.roll_block_at(x, y)
                    && around.iter().filter(|&&x| x).count() < 4
                {
                    result += 1;
                }
            }
        }

        result
    }

    fn remove_as_many_rolls_as_you_can(&mut self) -> usize {
        let mut result = 0;
        let mut changed: VecDeque<(usize, usize)> = VecDeque::new();

        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(around) = self.roll_block_at(x, y)
                    && around.iter().filter(|&&x| x).count() < 4
                {
                    self.set_is_roll(x, y, false);
                    changed.push_front((x, y));
                    result += 1;
                }
            }
        }

        while let Some((x, y)) = changed.pop_front() {
            let surrounding_positions = self.set_surrounding_positions(x, y);

            for pos in surrounding_positions {
                if let Some((x, y)) = pos
                    && let Some(around) = self.roll_block_at(x, y)
                    && around.iter().filter(|&&x| x).count() < 4
                {
                    self.set_is_roll(x, y, false);
                    changed.push_front((x, y));
                    result += 1;
                }
            }
        }

        result
    }

    fn is_roll(&self, x: usize, y: usize) -> bool {
        self.data[y * self.width + x]
    }

    fn set_is_roll(&mut self, x: usize, y: usize, b: bool) {
        self.data[y * self.width + x] = b;
    }

    fn roll_block_at(&self, x: usize, y: usize) -> Option<[bool; 8]> {
        if !self.is_roll(x, y) {
            return None;
        }

        let mut result = [false; DIRECTIONS.len()];
        for (i, &(x_delta, y_delta)) in DIRECTIONS.iter().enumerate() {
            result[i] = self.is_roll_relative(x, y, x_delta, y_delta);
        }

        Some(result)
    }

    fn set_surrounding_positions(&self, x: usize, y: usize) -> [Option<(usize, usize)>; 8] {
        DIRECTIONS.map(|(dx, dy)| {
            self.relative_position(x, y, dx, dy)
                .filter(|&(nx, ny)| self.is_roll(nx, ny))
        })
    }

    fn is_roll_relative(&self, x: usize, y: usize, x_delta: isize, y_delta: isize) -> bool {
        let x: isize = x as isize + x_delta;
        let y: isize = y as isize + y_delta;

        if x < 0 || x >= self.width as isize {
            return false;
        }

        if y < 0 || y >= self.height as isize {
            return false;
        }

        self.data[y as usize * self.width + x as usize]
    }

    fn relative_position(
        &self,
        x: usize,
        y: usize,
        x_delta: isize,
        y_delta: isize,
    ) -> Option<(usize, usize)> {
        let x: isize = x as isize + x_delta;
        let y: isize = y as isize + y_delta;

        if x < 0 || x >= self.width as isize {
            return None;
        }

        if y < 0 || y >= self.height as isize {
            return None;
        }

        Some((x as usize, y as usize))
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn test_day_4_part_1_sample() {
        let grid = Grid::from_str(include_str!("assets/day_4_sample.txt"));

        assert_eq!(grid.calculate_accessible_rolls(), 13);
    }

    #[test]
    fn test_day_4_part_1_real() {
        let grid = Grid::from_str(include_str!("assets/day_4.txt"));

        assert_eq!(grid.calculate_accessible_rolls(), 1491);
    }

    #[test]
    fn test_day_4_part_2_sample() {
        let mut grid = Grid::from_str(include_str!("assets/day_4_sample.txt"));

        assert_eq!(grid.remove_as_many_rolls_as_you_can(), 43);
    }

    #[test]
    fn test_day_4_part_2_real() {
        let t = Instant::now();

        let mut grid = Grid::from_str(include_str!("assets/day_4.txt"));
        assert_eq!(grid.remove_as_many_rolls_as_you_can(), 8722);

        println!("time taken: {:?}", t.elapsed());
    }
}
