use std::collections::HashSet;

fn repeated_ids(
    low: u64,
    high: u64,
    min_repeats: usize,
    max_repeats: usize,
) -> impl Iterator<Item = u64> {
    let mut ids = HashSet::new();

    for chunk_len in 1usize..=9 {
        let min_chunk = if chunk_len == 1 {
            1
        } else {
            10_u64.pow((chunk_len - 1) as u32)
        };
        let max_chunk = 10_u64.pow(chunk_len as u32) - 1;

        for repeats in min_repeats..=max_repeats.min(18usize / chunk_len) {
            let factor =
                (0..repeats).fold(0_u64, |factor, _| factor * 10_u64.pow(chunk_len as u32) + 1);
            let first = ((low + factor - 1) / factor).max(min_chunk);
            let last = (high / factor).min(max_chunk);

            for chunk in first..=last {
                ids.insert(chunk * factor);
            }
        }
    }

    ids.into_iter()
}

fn sum_repeated_ids(input: &str, min_repeats: usize, max_repeats: usize) -> u64 {
    input
        .trim()
        .split(",")
        .filter(|range| !range.trim().is_empty())
        .map(|range| {
            let (low, high) = range.trim().split_once("-").expect("invalid range");
            let low: u64 = low.parse().expect("invalid lower bound");
            let high: u64 = high.parse().expect("invalid upper bound");
            repeated_ids(low, high, min_repeats, max_repeats).sum::<u64>()
        })
        .sum()
}

pub fn part_a(input: &str) -> u64 {
    sum_repeated_ids(input, 2, 2)
}

pub fn part_b(input: &str) -> u64 {
    sum_repeated_ids(input, 2, 18)
}

#[cfg(test)]
mod tests {
    use super::{part_a, part_b};

    const SAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn solves_part_a_sample() {
        assert_eq!(part_a(SAMPLE), 1_227_775_554);
    }

    #[test]
    fn solves_part_b_sample() {
        assert_eq!(part_b(SAMPLE), 4_174_379_265);
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(part_a("\n"), 0);
        assert_eq!(part_b("\n"), 0);
    }
}
