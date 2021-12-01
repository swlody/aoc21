use itertools::Itertools;

pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn count(depths: impl IntoIterator<Item = u32>) -> usize {
    depths
        .into_iter()
        .tuple_windows()
        .filter(|(last_depth, current_depth)| current_depth > last_depth)
        .count()
}

pub fn solve_part1(depths: &[u32]) -> usize {
    count(depths.iter().cloned())
}

pub fn solve_part2(depths: &[u32]) -> usize {
    count(depths.windows(3).map(|window| window.iter().sum()))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

    #[test]
    fn test_part1() {
        let depths = input_generator(INPUT);
        assert_eq!(7, solve_part1(&depths));
    }

    #[test]
    fn test_part2() {
        let depths = input_generator(INPUT);
        assert_eq!(5, solve_part2(&depths));
    }
}
