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

    result
}

pub fn part_b(input: &str) -> u64 {
    let data: Vec<&str> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    if data.is_empty() {
        return 0;
    }

    let n = data.len();
    let width = data.iter().map(|line| line.len()).max().unwrap();

    let mut result: u64 = 0;
    let mut numbers: Vec<u64> = Vec::new();
    let mut operator = ' ';

    for col in 0..width {
        let mut builder = String::new();

        for row in 0..n - 1 {
            let ch = data[row].as_bytes().get(col).copied().unwrap_or(b' ');

            if ch != b' ' {
                builder.push(ch as char);
            }
        }

        let op = data[n - 1]
            .as_bytes()
            .get(col)
            .copied()
            .unwrap_or(b' ') as char;

        if op == '+' || op == '*' {
            operator = op;
        }

        if builder.is_empty() {
            if !numbers.is_empty() {
                let temp: u64 = if operator == '+' {
                    numbers.iter().sum()
                } else {
                    numbers.iter().product()
                };

                result += temp;
                numbers.clear();
                operator = ' ';
            }
        } else {
            numbers.push(builder.parse::<u64>().unwrap());
        }
    }

    // Process the last problem
    if !numbers.is_empty() {
        let temp: u64 = if operator == '+' {
            numbers.iter().sum()
        } else {
            numbers.iter().product()
        };

        result += temp;
    }

    result
}
#[cfg(test)]
mod tests {
    use super::{part_a, part_b};

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

    #[test]
    fn solves_part_b_sample() {
        assert_eq!(part_b(SAMPLE), 3_263_827);
    }
}
