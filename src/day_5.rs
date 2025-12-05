use std::cmp::Ordering::{self, Equal, Greater};

#[derive(Eq, PartialEq)]
struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn from_str(line: &str) -> Self {
        let (min, max) = line.split_once("-").unwrap();
        Self {
            min: min.parse().unwrap(),
            max: max.parse().unwrap(),
        }
    }

    /// Used in binary search, if the value we're searching for is possibly within
    /// this range, it matches, otherwise, the next search should look to a greater
    /// min - as the ranges are ordered by min, it can't be in a lower range
    fn binary_ordering(&self, value: u64) -> Ordering {
        if value >= self.min { Equal } else { Greater }
    }

    /// The number of ingredients in the range, inclusive
    fn count(&self) -> u64 {
        self.max - self.min + 1
    }
}

impl Ord for Range {
    /// Sorting of ranges is by the min only
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.min.cmp(&other.min)
    }
}

impl PartialOrd for Range {
    /// Range has total order, so use that
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Ranges {
    ranges: Vec<Range>,
}

impl Ranges {
    fn new(mut ranges: Vec<Range>) -> Self {
        ranges.sort_unstable();

        let mut merged_ranges: Vec<Range> = Vec::new();

        for r in ranges {
            match merged_ranges.last_mut() {
                Some(Range { max: last_max, .. }) => {
                    if r.min > *last_max {
                        merged_ranges.push(r);
                    } else if r.max > *last_max {
                        *last_max = r.max;
                    }
                }
                None => {
                    merged_ranges.push(r);
                }
            }
        }

        Self {
            ranges: merged_ranges,
        }
    }

    /// Binary search to efficiently see if any of the ranges contain value
    fn contains(&self, value: u64) -> bool {
        if let Ok(index) = self
            .ranges
            .binary_search_by(|range| range.binary_ordering(value))
        {
            value <= self.ranges[index].max
        } else {
            false
        }
    }

    /// How many of values are contained within any range
    fn count_containing(&self, values: &[u64]) -> usize {
        values.iter().filter(|&&val| self.contains(val)).count()
    }

    /// returns the total count of all ranges
    fn count(&self) -> u64 {
        self.ranges.iter().map(|range| range.count()).sum()
    }
}

#[cfg(test)]
mod tests {
    use std::str::Lines;

    use crate::bench;

    use super::*;

    fn parse_ranges(s: &str) -> (Ranges, Lines<'_>) {
        let mut lines = s.lines();
        let range_lines: Vec<Range> = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(Range::from_str)
            .collect();

        (Ranges::new(range_lines), lines)
    }

    fn parse_input(s: &str) -> (Ranges, Vec<u64>) {
        let (ranges, lines) = parse_ranges(s);
        let ingredient_lines = lines.map(|l| l.parse().unwrap()).collect();

        (ranges, ingredient_lines)
    }

    #[test]
    fn test_day_5_part_1_sample() {
        let (ranges, ingredients) = parse_input(include_str!("assets/day_5_sample.txt"));

        assert_eq!(ranges.count_containing(&ingredients), 3);
    }

    #[test]
    fn test_day_5_part_1_real() {
        bench(1000, || {
            let (ranges, ingredients) = parse_input(include_str!("assets/day_5.txt"));

            assert_eq!(ranges.count_containing(&ingredients), 638);
        });
    }

    #[test]
    fn test_day_5_part_2_sample() {
        let (ranges, _) = parse_ranges(include_str!("assets/day_5_sample.txt"));

        assert_eq!(ranges.count(), 14);
    }

    #[test]
    fn test_day_5_part_2_real() {
        bench(1000, || {
            let (ranges, _) = parse_ranges(include_str!("assets/day_5.txt"));

            assert_eq!(ranges.count(), 352946349407338);
        });
    }
}
