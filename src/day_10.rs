struct Machine {
    target: u16,
    buttons: Vec<u16>,
    joltages: Vec<u16>,
}

impl Machine {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split_whitespace().peekable();

        let target = Machine::parse_target(parts.next().unwrap());

        let buttons: Vec<_> = parts
            .clone()
            .take_while(|s| s.starts_with("("))
            .map(Machine::parse_button)
            .collect();

        let joltages = parts.skip(buttons.len()).next().unwrap();
        let joltages = joltages[1..joltages.len() - 1]
            .split(",")
            .map(|num| num.parse::<u16>().unwrap())
            .collect();

        Self {
            target,
            buttons,
            joltages,
        }
    }

    fn parse_target(s: &str) -> u16 {
        let mut result = 0;

        s.chars()
            .filter_map(|ch| {
                if ch == '.' {
                    Some(false)
                } else if ch == '#' {
                    Some(true)
                } else {
                    None
                }
            })
            .enumerate()
            .for_each(|(i, toggle)| {
                if toggle {
                    result |= 1 << i;
                }
            });

        result
    }

    fn parse_button(s: &str) -> u16 {
        let mut button = 0;

        s[1..s.len() - 1]
            .split(",")
            .map(|num| num.parse::<u8>().unwrap())
            .for_each(|bit| {
                button |= 1 << bit;
            });

        button
    }

    fn part_1(&self) -> u32 {
        // Essentially XOR logic problem.
        // pressing a button twice is the same as not pressing it at all
        // pressing 3 buttons in any order gives the same result
        // so knowing that order is not important!
        // we loop over each combination of buttons to find the min.
        // if we represent buttons as bits:
        // 0b0000 = none were pressed
        // 0b0011 = first 2 are pressed
        // 0b1111 = all are pressed
        // we can loop from 0b0000 to 0b1111 and apply the button mask for each 1
        // to work out the max value to loop to - there are 2 states for 6 buttons, so 2 ^^ 6

        // or more simply, we can employ Gosper's Hack to iterate bit combinations
        // from all 1 bit combinations for a width, then all 2 bit, then all 3 bit...
        // This will save us from work we don't need to do
        // More info here: https://rosettacode.org/wiki/Gosper%27s_hack

        // I'm sure there is some linear algebra that'll solve this :shrug:

        for (buttons_pressed_mask, buttons_pressed) in GosperIterator::new(self.buttons.len() as u8)
        {
            if self.buttons_pressed_result(buttons_pressed_mask) == self.target {
                return buttons_pressed as u32;
            }
        }

        panic!("no solution");
    }

    fn buttons_pressed_result(&self, buttons_pressed_mask: u32) -> u16 {
        self.buttons
            .iter()
            .enumerate()
            .filter_map(|(i, mask)| {
                if buttons_pressed_mask & (1 << i) != 0 {
                    Some(mask)
                } else {
                    None
                }
            })
            .fold(0, |acc, mask| acc ^ mask)
    }
}

#[cfg(test)]
mod tests {
    use crate::bench;

    use super::*;

    fn parse_machines(s: &str) -> Vec<Machine> {
        s.lines().map(Machine::from_str).collect()
    }

    #[test]
    fn test_day_10_part_1_sample() {
        let machines = parse_machines(include_str!("assets/day_10_sample.txt"));

        assert_eq!(machines.iter().map(Machine::part_1).sum::<u32>(), 7);
    }

    #[test]
    fn test_day_10_part_1_real() {
        bench(1000, || {
            let machines = parse_machines(include_str!("assets/day_10.txt"));
            assert_eq!(machines.iter().map(Machine::part_1).sum::<u32>(), 438);
        });
    }

    #[test]
    fn test_day_10_machine_from_str() {
        let machine = Machine::from_str("[.##.] (1,2,3) (2) (2,4) {3,5,4,7}");
        assert_eq!(machine.target, 0b0110);
        assert_eq!(machine.buttons, vec![0b1110, 0b0100, 0b10100]);
        assert_eq!(machine.joltages, vec![3, 5, 4, 7]);
    }

    #[test]
    fn test_day_10_parse_target_state() {
        assert_eq!(Machine::parse_target("[#...]"), 0b0001);
        assert_eq!(Machine::parse_target("[##..]"), 0b0011);
        assert_eq!(Machine::parse_target("[#.#.]"), 0b0101);
        assert_eq!(Machine::parse_target("[#..#]"), 0b1001);
        assert_eq!(
            Machine::parse_target("[.......#.......#]"),
            0b1000000010000000
        );
    }

    #[test]
    fn test_day_10_parse_button() {
        assert_eq!(Machine::parse_button("(0,1,3)"), 0b1011);
        assert_eq!(Machine::parse_button("(5)"), 0b100000);
        assert_eq!(Machine::parse_button("(4,10)"), 0b10000010000);
    }

    #[test]
    fn test_day_10_gospers_iter() {
        let mut gospers_iter = GosperIterator::new(4);

        assert_eq!(gospers_iter.next(), Some((0b0001, 1)));
        assert_eq!(gospers_iter.next(), Some((0b0010, 1)));
        assert_eq!(gospers_iter.next(), Some((0b0100, 1)));
        assert_eq!(gospers_iter.next(), Some((0b1000, 1)));

        assert_eq!(gospers_iter.next(), Some((0b0011, 2)));
        assert_eq!(gospers_iter.next(), Some((0b0101, 2)));
        assert_eq!(gospers_iter.next(), Some((0b0110, 2)));
        assert_eq!(gospers_iter.next(), Some((0b1001, 2)));
        assert_eq!(gospers_iter.next(), Some((0b1010, 2)));
        assert_eq!(gospers_iter.next(), Some((0b1100, 2)));

        assert_eq!(gospers_iter.next(), Some((0b0111, 3)));
        assert_eq!(gospers_iter.next(), Some((0b1011, 3)));
        assert_eq!(gospers_iter.next(), Some((0b1101, 3)));
        assert_eq!(gospers_iter.next(), Some((0b1110, 3)));

        assert_eq!(gospers_iter.next(), Some((0b1111, 4)));

        assert_eq!(gospers_iter.next(), None);
    }
}

// I didn't come up with this magic: https://rosettacode.org/wiki/Gosper%27s_hack
struct GosperIterator {
    width: u8,
    k: u8,
    x: u32,
}

impl GosperIterator {
    fn new(width: u8) -> Self {
        Self { width, k: 1, x: 1 }
    }
}

impl Iterator for GosperIterator {
    type Item = (u32, u8);

    fn next(&mut self) -> Option<Self::Item> {
        while self.k <= self.width {
            if self.x < (1 << self.width) {
                let result = self.x;

                let c = self.x & (!self.x + 1);
                let r = self.x + c;
                self.x = (((r ^ self.x) >> 2) / c) | r;

                return Some((result, self.k));
            }

            self.k += 1;
            self.x = (1 << self.k) - 1;
        }

        None
    }
}
