fn number_of_forklift(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input
            .lines()                         
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();  
    
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let row = grid.len();
    let col = grid[0].len();
    let directions: [[i32; 2]; 8] = [
        [1, 0],
        [0, 1],
        [-1, 0],
        [0, -1],
        [1, 1],
        [-1, -1],
        [1, -1],
        [-1, 1]
    ];

    let mut result = 0;
    for r in 0..row {
        for c in 0..col {
            if grid[r][c] == '@' {
                let mut count: usize = 0;
                for [dr, dc] in directions {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    
                    if nr >= 0 && nr < row as i32 && nc >= 0 && nc < col as i32 {
                        if grid[nr as usize][nc as usize] == '@' {
                            count += 1;
                        }
                    }
                }
                if count < 4 {
                    result += 1;
                }
            }
        }
    }
    result
}

pub fn part_a(input: &str) -> u64 {
    number_of_forklift(input)
}

// pub fn part_b(input: &str) -> u64 {
//     max_digits(input, 12)
// }


#[cfg(test)]
mod tests {
    use super::{part_a};

    const SAMPLE: &str = r#"
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
    "#;

    #[test]
    fn solves_part_a_sample() {
        assert_eq!(part_a(SAMPLE), 13);
    }

    // #[test]
    // fn solves_part_b_sample() {
    //     assert_eq!(part_b(SAMPLE), 3_121_910_778_619);
    // }
}