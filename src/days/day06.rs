pub fn part_a(input: &str) -> u64 {
    let mut data: Vec<Vec<&str>> = Vec::new();

    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        data.push(line.split_whitespace().collect::<Vec<&str>>());
    }

    let rotated: Vec<Vec<&str>> = (0..data[0].len())
        .map(|col| {
            data.iter()
                .map(|row| row[col])
                .collect()
        })
        .collect();

    let mut result: u64 = 0;
    for exp in &rotated {
        let n = exp.len();
        if exp[n-1] == "+" {
            let mut temp: u64 = 0;
            for idx in 0..n-1 {
                temp += exp[idx].parse::<u64>().unwrap();
            }
            result += temp;
        } else {
            let mut temp: u64 = 1;
            for idx in 0..n-1 {
                temp *= exp[idx].parse::<u64>().unwrap();
            }
            result += temp;
        }
    }

    println!("{:?}", result);

    result
}

// pub fn part_b(input: &str) -> u64 {
//     let mut ranges: Vec<(u64, u64)> = Vec::new();

//     for line in input.lines() {
//         let line = line.trim();

//         if line.is_empty() {
//             continue;
//         }

//         if let Some((start, end)) = line.split_once('-') {
//             ranges.push((start.parse().unwrap(), end.parse().unwrap()));
//         }
//     }

//     let mut result = 0;
//     ranges = merge_range(ranges);
//     for &range in &ranges {
//         result += range.1 - range.0 + 1;
//     }

//     result
// }

#[cfg(test)]
mod tests {
    use super::{part_a};

    const SAMPLE: &str = r#"
        123 328  51 64 
        45 64  387 23 
        6 98  215 314
        *   +   *   +  
    "#;

    #[test]
    fn solves_part_a_sample() {
        assert_eq!(part_a(SAMPLE), 4_277_556);
    }

    // const SAMPLEB: &str = r#"
    //     3-5
    //     10-14
    //     16-20
    //     12-18
    // "#;

    // #[test]
    // fn solves_part_b_sample() {
    //     assert_eq!(part_b(SAMPLEB), 14);
    // }
}
