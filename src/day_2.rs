use std::ops::Range;

struct DayTwo {
    buf: Vec<u8>,
    ranges: Vec<Range<u64>>,
}

impl DayTwo {
    fn new(data: &str) -> Self {
        Self {
            buf: vec![0u8; 20],
            ranges: parse_ranges(data),
        }
    }

    fn result(&mut self, predicate: fn(&[u8]) -> bool) -> u64 {
        let mut result = Vec::new();

        for range in self.ranges.iter() {
            for n in range.clone() {
                let digits = get_digits(n, &mut self.buf);

                if predicate(digits) {
                    result.push(n);
                }
            }
        }

        result.iter().sum()
    }

    fn part_1(digits: &[u8]) -> bool {
        if digits.len().is_multiple_of(2) {
            let first_half = &digits[0..digits.len() / 2];
            let second_half = &digits[digits.len() / 2..];

            first_half == second_half
        } else {
            false
        }
    }

    fn part_2(digits: &[u8]) -> bool {
        let mut chunk_size = 1;

        while chunk_size < digits.len() {
            if digits.len().is_multiple_of(chunk_size)
                && digits.chunks(chunk_size).min() == digits.chunks(chunk_size).max()
            {
                return true;
            }

            chunk_size += 1;
        }

        false
    }
}

// reuse a buffer here, as we don't have to allocate for each number in each range
fn get_digits(n: u64, v: &mut [u8]) -> &[u8] {
    let mut r = n;
    let mut d = 19;

    while r > 0 {
        v[d] = (r % 10) as u8;
        r /= 10;
        d -= 1;
    }

    &v[d + 1..]
}

fn parse_ranges(data: &str) -> Vec<Range<u64>> {
    data.split(",")
        .map(str::trim)
        .map(|str| {
            let (start, end) = str.split_once("-").unwrap();
            let start = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();

            start..end + 1
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2_part_1_sample() {
        let mut day_two = DayTwo::new(include_str!("assets/day_2_sample.txt"));
        assert_eq!(day_two.result(DayTwo::part_1), 1227775554);
    }

    #[test]
    fn test_day_2_part_1_real() {
        let mut day_two = DayTwo::new(include_str!("assets/day_2.txt"));
        assert_eq!(day_two.result(DayTwo::part_1), 40398804950);
    }

    #[test]
    fn test_day_2_part_2_sample() {
        let mut day_two = DayTwo::new(include_str!("assets/day_2_sample.txt"));
        assert_eq!(day_two.result(DayTwo::part_2), 4174379265);
    }

    #[test]
    fn test_day_2_part_2_real() {
        let mut day_two = DayTwo::new(include_str!("assets/day_2.txt"));
        assert_eq!(day_two.result(DayTwo::part_2), 65794984339);
    }
}
