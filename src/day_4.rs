// --- Day 4: Printing Department ---
// You ride the escalator down to the printing department. They're clearly getting ready for Christmas; they have lots of large rolls of paper everywhere, and there's even a massive printer in the corner (to handle the really big print jobs).

// Decorating here will be easy: they can make their own decorations. What you really need is a way to get further into the North Pole base while the elevators are offline.

// "Actually, maybe we can help with that," one of the Elves replies when you ask for help. "We're pretty sure there's a cafeteria on the other side of the back wall. If we could break through the wall, you'd be able to keep moving. It's too bad all of our forklifts are so busy moving those big rolls of paper around."

// If you can optimize the work the forklifts are doing, maybe they would have time to spare to break through the wall.

// The rolls of paper (@) are arranged on a large grid; the Elves even have a helpful diagram (your puzzle input) indicating where everything is located.

// For example:

// ..@@.@@@@.
// @@@.@.@.@@
// @@@@@.@.@@
// @.@@@@..@.
// @@.@@@@.@@
// .@@@@@@@.@
// .@.@.@.@@@
// @.@@@.@@@@
// .@@@@@@@@.
// @.@.@@@.@.
// The forklifts can only access a roll of paper if there are fewer than four rolls of paper in the eight adjacent positions. If you can figure out which rolls of paper the forklifts can access, they'll spend less time looking and more time breaking down the wall to the cafeteria.

// In this example, there are 13 rolls of paper that can be accessed by a forklift (marked with x):

// ..xx.xx@x.
// x@@.@.@.@@
// @@@@@.x.@@
// @.@@@@..@.
// x@.@@@@.@x
// .@@@@@@@.@
// .@.@.@.@@@
// x.@@@.@@@@
// .@@@@@@@@.
// x.x.@@@.x.
// Consider your complete diagram of the paper roll locations. How many rolls of paper can be accessed by a forklift?

// Now, the Elves just need help accessing as much of the paper as they can.

// Once a roll of paper can be accessed by a forklift, it can be removed. Once a roll of paper is removed, the forklifts might be able to access more rolls of paper, which they might also be able to remove. How many total rolls of paper could the Elves remove if they keep repeating this process?

// Stop once no more rolls of paper are accessible by a forklift. In this example, a total of 43 rolls of paper can be removed.

// Start with your original diagram. How many rolls of paper in total can be removed by the Elves and their forklifts?

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
