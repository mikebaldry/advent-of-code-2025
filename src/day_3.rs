fn digits_to_num(digits: &[u32]) -> u64 {
    let mut result: u64 = 0;
    let mut pow: u64 = 1;
    for &digit in digits.iter().rev() {
        result += digit as u64 * pow;
        pow *= 10;
    }

    result
}

fn part_1(bank: &[u32]) -> u64 {
    let (start_index, &first_digit) = bank[0..bank.len() - 1]
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(&a.0)))
        .unwrap();
    let second_digit = bank[start_index + 1..].iter().max().unwrap();

    digits_to_num(&[first_digit, *second_digit])
}

fn part_2(bank: &[u32]) -> u64 {
    let mut digits = Vec::<u32>::new();
    let mut starting_from_index = 0;

    let indexed_bank = bank.iter().enumerate().collect::<Vec<(usize, &u32)>>();

    while digits.len() < 12 {
        let remaining = &indexed_bank[starting_from_index..];
        let further_digits_required = 12 - digits.len() - 1;
        let window: usize = remaining.len() - further_digits_required;
        let possibilities = &remaining[0..window];
        let (index, digit) = possibilities
            .iter()
            .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(&a.0)))
            .unwrap();

        digits.push(**digit);
        starting_from_index = index + 1;
    }

    digits_to_num(&digits)
}

fn total_output_joltage(data: &str, part: fn(bank: &[u32]) -> u64) -> u64 {
    data.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.chars()
                .map(|digit| digit.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|bank| part(&bank))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::time;

    use super::*;

    #[test]
    fn test_day_3_part_1_sample() {
        assert_eq!(
            total_output_joltage(include_str!("assets/day_3_sample.txt"), part_1),
            357
        );
    }

    #[test]
    fn test_day_3_part_1_real() {
        assert_eq!(
            total_output_joltage(include_str!("assets/day_3.txt"), part_1),
            17155
        );
    }

    #[test]
    fn test_day_3_part_2_sample() {
        assert_eq!(
            total_output_joltage(include_str!("assets/day_3_sample.txt"), part_2),
            3121910778619
        );
    }

    #[test]
    fn test_day_3_part_2_real() {
        let t = time::Instant::now();

        assert_eq!(
            total_output_joltage(include_str!("assets/day_3.txt"), part_2),
            169685670469164
        );

        println!("time taken: {:?}", t.elapsed());
    }
}
