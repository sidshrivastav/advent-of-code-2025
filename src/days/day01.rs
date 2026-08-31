fn rotations(input: &str) -> impl Iterator<Item = (char, i32)> + '_ {
    input.lines().map(|line| {
        let (direction, distance) = line.split_at(1);
        (
            direction.chars().next().unwrap(),
            distance.parse().expect("distance should be a number"),
        )
    })
}

pub fn part_a(input: &str) -> i32 {
    let mut position = 50;
    rotations(input)
        .filter_map(|(direction, distance)| {
            position = match direction {
                'L' => (position - distance).rem_euclid(100),
                'R' => (position + distance).rem_euclid(100),
                _ => panic!("direction should be L or R"),
            };
            (position == 0).then_some(1)
        })
        .sum()
}

pub fn part_b(input: &str) -> i32 {
    let mut position = 50;
    rotations(input)
        .map(|(direction, distance)| {
            let step: i32 = if direction == 'L' {
                -1
            } else if direction == 'R' {
                1
            } else {
                panic!("direction should be L or R")
            };
            (0..distance)
                .filter(|_| {
                    position = (position + step).rem_euclid(100);
                    position == 0
                })
                .count() as i32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part_a, part_b};
    #[test]
    fn counts_ending_zeroes() {
        assert_eq!(
            part_a(
                "R50
L1
R1
"
            ),
            2
        );
    }
    #[test]
    fn counts_each_zero_crossing() {
        assert_eq!(
            part_b(
                "L50
R100
"
            ),
            2
        );
    }
}
