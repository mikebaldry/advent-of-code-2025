enum Rotation {
    Left(u32),
    Right(u32),
}

impl Rotation {
    fn change(&self) -> i32 {
        match self {
            Rotation::Left(n) => -(*n as i32),
            Rotation::Right(n) => *n as i32,
        }
    }
}

impl From<&str> for Rotation {
    fn from(value: &str) -> Self {
        let direction = &value[0..1];
        let number = value[1..].parse().expect("valid number");

        match direction {
            "L" => Rotation::Left(number),
            "R" => Rotation::Right(number),
            _ => panic!("unknown direction: {0}", direction),
        }
    }
}

struct RotaryLock {
    position: i32,
    max: i32,
    rotations: u32,
}

impl RotaryLock {
    fn new() -> Self {
        Self {
            position: 50,
            max: 100,
            rotations: 0,
        }
    }

    fn rotate_1(&mut self, r: &Rotation) {
        self.position = (self.position + r.change()).rem_euclid(self.max);

        if self.position == 0 {
            self.rotations += 1;
        }
    }

    fn rotate_2(&mut self, r: &Rotation) {
        let new_position = (self.position + r.change()).rem_euclid(self.max);
        let mut new_rotations = (self.position + r.change())
            .div_euclid(self.max)
            .unsigned_abs();

        if let Rotation::Left(_) = r {
            if new_position == 0 {
                new_rotations += 1;
            }

            if self.position == 0 {
                new_rotations -= 1;
            }
        }

        self.position = new_position;
        self.rotations += new_rotations;
    }

    fn password(&self) -> u32 {
        self.rotations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_rotations(data: &str) -> Vec<Rotation> {
        data.lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .map(Rotation::from)
            .collect()
    }

    fn get_password(data: &str, rotate: fn(&mut RotaryLock, &Rotation)) -> u32 {
        let rotations = parse_rotations(data);
        let mut rotary_lock = RotaryLock::new();

        for r in rotations {
            rotate(&mut rotary_lock, &r)
        }

        rotary_lock.password()
    }

    #[test]
    fn test_day_1_part_1_sample() {
        let password = get_password(
            include_str!("assets/day_1_sample.txt"),
            RotaryLock::rotate_1,
        );

        assert_eq!(password, 3);
    }

    #[test]
    fn test_day_1_part_1_real() {
        let password = get_password(include_str!("assets/day_1.txt"), RotaryLock::rotate_1);

        assert_eq!(password, 1052);
    }

    #[test]
    fn test_day_1_part_2_sample() {
        let password = get_password(
            include_str!("assets/day_1_sample.txt"),
            RotaryLock::rotate_2,
        );

        assert_eq!(password, 6);
    }

    #[test]
    fn test_day_1_part_2_real() {
        let password = get_password(include_str!("assets/day_1.txt"), RotaryLock::rotate_2);

        assert_eq!(password, 6295);
    }
}
