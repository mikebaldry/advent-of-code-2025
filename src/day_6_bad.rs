#[cfg(test)]
mod tests {
    use crate::bench;

    fn part_1(s: &str) -> u64 {
        let mut lines = s.lines().rev();
        let operators: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
        let row_iters = lines.map(|l| {
            l.split_whitespace()
                .map(|o| o.parse::<u64>().unwrap())
                .into_iter()
        });
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
        let mut lines: Vec<&str> = s.lines().collect();
        let operators: Vec<&str> = lines.pop().unwrap().split_whitespace().collect();

        // this is pretty terrible but works. Is there a better way?
        // 1. work out column widths as whitespace is very significant
        // 2. pull out strings for each column of each row with whitespace
        // 3. for each column, for each n in width, pull out the chars from each row, parse to a number, do */+ in to an array of results
        // 4. sum results

        let mut column_widths = Vec::with_capacity(operators.len());
        let intermediate_sizes: Vec<Vec<usize>> = lines
            .iter()
            .map(|l| l.split_whitespace().map(|v| v.len()).collect())
            .collect();
        for col in 0..operators.len() {
            column_widths.push(intermediate_sizes.iter().map(|v| v[col]).max().unwrap());
        }

        let mut column_strs: Vec<Vec<String>> = Vec::with_capacity(operators.len());
        for col in 0..operators.len() {
            let mut column = Vec::new();
            let column_width = column_widths[col];
            lines.iter_mut().for_each(|l| {
                let to_take = column_width.min(l.len());
                column.push(l[0..to_take].to_owned());
                if l.len() > to_take {
                    *l = &l[to_take + 1..];
                }
            });
            column_strs.push(column);
        }

        let mut results = vec![0; operators.len()];

        for (col, op) in operators.iter().enumerate() {
            let width = column_widths[col];

            for n in 0..width {
                let value: u64 = column_strs[col]
                    .iter()
                    .map(|s| s.chars().nth(n))
                    .filter(|c| c.is_some() && c.unwrap() != ' ')
                    .filter_map(|x| x)
                    .collect::<String>()
                    .parse()
                    .unwrap();

                match *op {
                    "+" => {
                        results[col] += value;
                    }
                    "*" => {
                        if results[col] == 0 {
                            results[col] = 1;
                        }
                        results[col] *= value;
                    }
                    _ => panic!("unknown operator: {}", op),
                }
            }
        }

        results.iter().sum()
    }

    #[test]
    fn test_day_6_bad_part_1_sample() {
        assert_eq!(part_1(include_str!("assets/day_6_sample.txt")), 4277556);
    }

    #[test]
    fn test_day_6_bad_part_1_real() {
        assert_eq!(part_1(include_str!("assets/day_6.txt")), 4412382293768);
    }

    #[test]
    fn test_day_6_bad_part_2_sample() {
        assert_eq!(part_2(include_str!("assets/day_6_sample.txt")), 3263827);
    }

    #[test]
    fn test_day_6_bad_part_2_real() {
        bench(1000, || {
            assert_eq!(part_2(include_str!("assets/day_6.txt")), 7858808482092);
        });
    }
}
