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
}
