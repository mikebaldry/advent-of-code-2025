fn part_1(s: &str) -> u64 {
    let mut lines = s.lines().rev();
    let operators: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
    let row_iters = lines.map(|l| l.split_whitespace().map(|o| o.parse::<u64>().unwrap()));
    let mut results = vec![0; operators.len()];

    for row_iter in row_iters {
        for (col, value) in row_iter.enumerate() {
            match operators[col] {
                "+" => {
                    results[col] += value;
                }
                "*" => {
                    if results[col] == 0 {
                        results[col] = 1;
                    }
                    results[col] *= value;
                }
                _ => panic!("wut is {}", operators[col]),
            }
        }
    }

    results.iter().sum()
}

fn part_2(s: &str) -> u64 {
    // Now uses an iterator per line to step each column, and performing operation when it changes
    // far fewer allocations and simpler to read.

    let mut line_iters: Vec<_> = s.lines().map(|l| l.chars()).collect();
    let mut sum = 0;

    let mut current_op: Option<char> = None;
    let mut current_nums: Vec<u64> = Vec::new();

    loop {
        let col = line_iters
            .iter_mut()
            .map(|l| l.next())
            .collect::<Vec<Option<char>>>();

        if col.iter().all(Option::is_none) {
            sum += calculate(current_op.unwrap(), &current_nums);
            current_nums.clear();
            break;
        }

        if let Some(op) = col.last().unwrap()
            && !op.is_ascii_whitespace()
        {
            if current_op.is_some() {
                sum += calculate(current_op.unwrap(), &current_nums);
                current_op = Some(*op);
                current_nums.clear();
            } else {
                current_op = Some(*op);
            }
        }

        if let Some(num) = digits_to_num(&col[0..col.len() - 1]) {
            current_nums.push(num);
        }
    }

    sum
}

fn calculate(op: char, values: &[u64]) -> u64 {
    match op {
        '+' => values.iter().sum(),
        '*' => values.iter().product(),
        op => panic!("wut is {op}"),
    }
}

fn digits_to_num(digits: &[Option<char>]) -> Option<u64> {
    let result = digits
        .iter()
        .filter_map(|&c| {
            if let Some(c) = c
                && c.is_ascii_digit()
            {
                Some(c as u8 - b'0')
            } else {
                None
            }
        })
        .fold(0, |acc, digit| acc * 10 + digit as u64);

    if result > 0 { Some(result) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bench;

    #[test]
    fn test_day_6_part_1_sample() {
        assert_eq!(part_1(include_str!("assets/day_6_sample.txt")), 4277556);
    }

    #[test]
    fn test_day_6_part_1_real() {
        assert_eq!(part_1(include_str!("assets/day_6.txt")), 4412382293768);
    }

    #[test]
    fn test_day_6_part_2_sample() {
        assert_eq!(part_2(include_str!("assets/day_6_sample.txt")), 3263827);
    }

    #[test]
    fn test_day_6_part_2_real() {
        bench(1000, || {
            assert_eq!(part_2(include_str!("assets/day_6.txt")), 7858808482092);
        });
    }
}
