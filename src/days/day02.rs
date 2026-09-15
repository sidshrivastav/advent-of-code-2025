fn sum_repeated_ids(low: u64, high: u64) -> u64 {
    let mut total = 0;

    for half_len in 1..=9 {
        let power = 10_u64.pow(half_len);
        let factor = power + 1;
        let min_half = if half_len == 1 {
            1
        } else {
            10_u64.pow(half_len - 1)
        };
        let max_half = power - 1;

        let first = ((low + factor - 1) / factor).max(min_half);
        let last = (high / factor).min(max_half);

        if first <= last {
            for half in first..=last {
                total += half * factor;
            }
        }
    }

    total
}

pub fn part_a(input: &str) -> u64 {
    let mut total = 0;

    for range in input
        .trim()
        .split(",")
        .filter(|range| !range.trim().is_empty())
    {
        let (low, high) = range.trim().split_once("-").expect("invalid range");
        let low: u64 = low.parse().expect("invalid lower bound");
        let high: u64 = high.parse().expect("invalid upper bound");
        total += sum_repeated_ids(low, high);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::part_a;

    #[test]
    fn sums_repeated_ids_in_sample() {
        assert_eq!(
            part_a(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            ),
            1_227_775_554
        );
    }

    #[test]
    fn handles_empty_input() {
        assert_eq!(part_a("\n"), 0);
    }
}
