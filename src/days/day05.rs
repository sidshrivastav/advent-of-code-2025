fn merge_range(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return Vec::new();
    }

    ranges.sort_by_key(|x| x.0);

    let mut merged_ranges = vec![ranges[0]];

    for i in 1..ranges.len() {
        let last = merged_ranges.last_mut().unwrap();

        if last.1 >= ranges[i].0 {
            last.1 = last.1.max(ranges[i].1);
        } else {
            merged_ranges.push(ranges[i]);
        }
    }

    merged_ranges
}

pub fn part_a(input: &str) -> u64 {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut numbers: Vec<u64> = Vec::new();

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if let Some((start, end)) = line.split_once('-') {
            ranges.push((start.parse().unwrap(), end.parse().unwrap()));
        } else {
            numbers.push(line.parse().unwrap());
        }
    }

    ranges = merge_range(ranges);

    let mut result = 0;

    for &num in &numbers {
        if ranges
            .binary_search_by(|&(start, end)| {
                if num < start {
                    std::cmp::Ordering::Greater
                } else if num > end {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            })
            .is_ok()
        {
            result += 1;
        }
    }

    result
}

pub fn part_b(input: &str) -> u64 {
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if let Some((start, end)) = line.split_once('-') {
            ranges.push((start.parse().unwrap(), end.parse().unwrap()));
        }
    }

    let mut result = 0;
    ranges = merge_range(ranges);
    for &range in &ranges {
        result += range.1 - range.0 + 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{part_a, part_b};

    const SAMPLEA: &str = r#"
        3-5
        10-14
        16-20
        12-18

        1
        5
        8
        11
        17
        32
    "#;

    #[test]
    fn solves_part_a_sample() {
        assert_eq!(part_a(SAMPLEA), 3);
    }

    const SAMPLEB: &str = r#"
        3-5
        10-14
        16-20
        12-18
    "#;

    #[test]
    fn solves_part_b_sample() {
        assert_eq!(part_b(SAMPLEB), 14);
    }
}
