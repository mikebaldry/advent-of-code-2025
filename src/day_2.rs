// --- Day 2: Gift Shop ---
// You get inside and take the elevator to its only other stop: the gift shop. "Thank you for visiting the North Pole!" gleefully exclaims a nearby sign. You aren't sure who is even allowed to visit the North Pole, but you know you can access the lobby through here, and from there you can access the rest of the North Pole base.

// As you make your way through the surprisingly extensive selection, one of the clerks recognizes you and asks for your help.

// As it turns out, one of the younger Elves was playing on a gift shop computer and managed to add a whole bunch of invalid product IDs to their gift shop database! Surely, it would be no trouble for you to identify the invalid product IDs for them, right?

// They've even checked most of the product ID ranges already; they only have a few product ID ranges (your puzzle input) that you'll need to check. For example:

// 11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
// 1698522-1698528,446443-446449,38593856-38593862,565653-565659,
// 824824821-824824827,2121212118-2121212124
// (The ID ranges are wrapped here for legibility; in your input, they appear on a single long line.)

// The ranges are separated by commas (,); each range gives its first ID and last ID separated by a dash (-).

// Since the young Elf was just doing silly patterns, you can find the invalid IDs by looking for any ID which is made only of some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.

// None of the numbers have leading zeroes; 0101 isn't an ID at all. (101 is a valid ID that you would ignore.)

// Your job is to find all of the invalid IDs that appear in the given ranges. In the above example:

// 11-22 has two invalid IDs, 11 and 22.
// 95-115 has one invalid ID, 99.
// 998-1012 has one invalid ID, 1010.
// 1188511880-1188511890 has one invalid ID, 1188511885.
// 222220-222224 has one invalid ID, 222222.
// 1698522-1698528 contains no invalid IDs.
// 446443-446449 has one invalid ID, 446446.
// 38593856-38593862 has one invalid ID, 38593859.
// The rest of the ranges contain no invalid IDs.
// Adding up all the invalid IDs in this example produces 1227775554.

// What do you get if you add up all of the invalid IDs?

// --- Part Two ---
// The clerk quickly discovers that there are still invalid IDs in the ranges in your list. Maybe the young Elf was doing other silly patterns as well?

// Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice. So, 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 five times), and 1111111 (1 seven times) are all invalid IDs.

// From the same example as before:

// 11-22 still has two invalid IDs, 11 and 22.
// 95-115 now has two invalid IDs, 99 and 111.
// 998-1012 now has two invalid IDs, 999 and 1010.
// 1188511880-1188511890 still has one invalid ID, 1188511885.
// 222220-222224 still has one invalid ID, 222222.
// 1698522-1698528 still contains no invalid IDs.
// 446443-446449 still has one invalid ID, 446446.
// 38593856-38593862 still has one invalid ID, 38593859.
// 565653-565659 now has one invalid ID, 565656.
// 824824821-824824827 now has one invalid ID, 824824824.
// 2121212118-2121212124 now has one invalid ID, 2121212121.
// Adding up all the invalid IDs in this example produces 4174379265.

// What do you get if you add up all of the invalid IDs using these new rules?

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
