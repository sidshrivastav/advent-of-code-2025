use std::cmp::Reverse;

fn max_digits(input: &str, digit_count: usize) -> u64 {
    input
        .trim()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let numbers: Vec<u32> = line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect();

            let mut result = 0u64;
            let mut start = 0;

            for remaining in (1..=digit_count).rev() {
                let end = numbers.len() - remaining;

                let (offset, &digit) = numbers[start..=end]
                    .iter()
                    .enumerate()
                    .max_by_key(|(offset, digit)| (**digit, Reverse(*offset)))
                    .unwrap();

                result = result * 10 + digit as u64;
                start += offset + 1;
            }
            result
        })
        .sum()
}

pub fn part_a(input: &str) -> u64 {
    max_digits(input, 2)
}

pub fn part_b(input: &str) -> u64 {
    max_digits(input, 12)
}


#[cfg(test)]
mod tests {
    use super::{part_a, part_b};

    const SAMPLE: &str = r#"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "#;

    #[test]
    fn solves_part_a_sample() {
        assert_eq!(part_a(SAMPLE), 357);
    }

    #[test]
    fn solves_part_b_sample() {
        assert_eq!(part_b(SAMPLE), 3_121_910_778_619);
    }
}