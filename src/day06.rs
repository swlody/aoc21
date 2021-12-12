pub fn input_generator(input: &str) -> Vec<u32> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

fn solve(fish: &[u32], days: u32) -> u64 {
    let mut counts: [u64; 9] = [0; 9];
    for &num in fish.iter() {
        counts[num as usize] += 1;
    }

    for _ in 0..days {
        let spawn_count = counts[0];
        for i in 0..=7 {
            counts[i] = counts[i + 1];
        }
        counts[6] += spawn_count;
        counts[8] = spawn_count;
    }

    counts.iter().sum()
}

pub fn solve_part1(fish: &[u32]) -> u64 {
    solve(fish, 80)
}

pub fn solve_part2(fish: &[u32]) -> u64 {
    solve(fish, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        let fish = input_generator(INPUT);
        assert_eq!(5934, solve_part1(&fish));
    }

    #[test]
    fn test_part2() {
        let fish = input_generator(INPUT);
        assert_eq!(26984457539, solve_part2(&fish));
    }
}
