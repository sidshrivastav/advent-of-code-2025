pub fn solve(input: &str) -> i32 {
    let mut position = 50;
    let mut zeroes = 0;
    for line in input.lines() {
        let (direction, distance) = line.split_at(1);
        let distance: i32 = distance.parse().expect("distance should be a number");
        position = match direction {
            "L" => (position - distance).rem_euclid(100),
            "R" => (position + distance).rem_euclid(100),
            _ => panic!("direction should be L or R"),
        };
        if position == 0 {
            zeroes += 1;
        }
    }
    zeroes
}
#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn counts_zeroes() {
        assert_eq!(solve("R50\nL1\nR1\n"), 2);
    }
}
