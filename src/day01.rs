pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn solve_part1(depths: &[u32]) -> u32 {
    let mut num_increases = 0;
    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            num_increases += 1;
        }
    }
    num_increases
}

pub fn solve_part2(depths: &[u32]) -> u32 {
    let mut num_increases = 0;
    for i in 3..depths.len() {
        if depths[i - 2] + depths[i - 1] + depths[i] > depths[i - 3] + depths[i - 2] + depths[i - 1] {
            num_increases += 1;
        }
    }
    num_increases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
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

        let depths = input_generator(INPUT);
        assert_eq!(7, solve_part1(&depths));
    }

    #[test]
    fn test_part2() {
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

        let depths = input_generator(INPUT);
        assert_eq!(5, solve_part2(&depths));
    }
}
