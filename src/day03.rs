pub fn solve_part1(report: &str) -> usize {
    // Count of number of ones for each binary position
    let mut ones_count: [usize; 64] = [0; 64];
    // The total number of lines in the file
    let mut lines = 0;
    // The length of the binary numbers (assume the same for all)
    let mut length = 0;
    // For each binary number in the input...
    for line in report.lines() {
        // Store and check the length of the number
        if length == 0 {
            length = line.len();
        } else {
            assert!(length == line.len());
        }

        // For each position, if it is a 1, increment the ones count
        for (i, c) in line.chars().enumerate() {
            match c {
                '1' => ones_count[i] += 1,
                '0' => {}
                c => panic!("Invalid number in input: {}", c),
            }
        }
        lines += 1;
    }

    // The number of ones which would imply more ones than zeroes were seen in a given position
    let threshold = lines / 2;
    let (mut gamma, mut epsilon) = (0, 0);
    // For each position up to the length of the binary numbers...
    for &count in ones_count.iter().take(length) {
        // Left shift each number by 1 position
        gamma <<= 1;
        epsilon <<= 1;
        // Add a one to the end depending on the gamma/epsilon rules
        if count > threshold {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    // Finally, multiply the two numbers together
    gamma * epsilon
}

pub fn solve_part2(input: &str) -> u64 {
    let mut common_candidates = input
        .lines()
        .map(|line| line.to_owned())
        .collect::<Vec<String>>();
    let mut uncommon_candidates = common_candidates.clone();

    let length = common_candidates[0].len();

    let mut co2_rating = 0;
    let mut o2_rating = 0;

    for i in 0..length {
        let mut common_ones_count = 0;
        let mut common_zeroes_count = 0;
        let mut uncommon_ones_count = 0;
        let mut uncommon_zeroes_count = 0;

        let mut ones_common_candidates = Vec::new();
        let mut zeroes_common_candidates = Vec::new();
        let mut ones_uncommon_candidates = Vec::new();
        let mut zeroes_uncommon_candidates = Vec::new();

        // This loop will keep running even if we've already calculated the O2/CO2 rating, whoops
        for num in common_candidates {
            if num.as_bytes()[i] == b'1' {
                common_ones_count += 1;
                ones_common_candidates.push(num);
            } else {
                common_zeroes_count += 1;
                zeroes_common_candidates.push(num);
            }
        }

        for num in uncommon_candidates {
            if num.as_bytes()[i] == b'1' {
                uncommon_ones_count += 1;
                ones_uncommon_candidates.push(num);
            } else {
                uncommon_zeroes_count += 1;
                zeroes_uncommon_candidates.push(num);
            }
        }

        if common_ones_count >= common_zeroes_count {
            common_candidates = ones_common_candidates;
        } else {
            common_candidates = zeroes_common_candidates;
        }

        if uncommon_ones_count < uncommon_zeroes_count {
            uncommon_candidates = ones_uncommon_candidates;
        } else {
            uncommon_candidates = zeroes_uncommon_candidates;
        }

        if common_candidates.len() == 1 {
            o2_rating = u64::from_str_radix(&common_candidates[0], 2).unwrap();
        }
        if uncommon_candidates.len() == 1 {
            co2_rating = u64::from_str_radix(&uncommon_candidates[0], 2).unwrap();
        }
    }

    o2_rating * co2_rating
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part1() {
        assert_eq!(198, solve_part1(&INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(230, solve_part2(&INPUT));
    }
}
