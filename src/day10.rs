fn get_closing_brace(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Invalid character"),
    }
}

pub fn solve_part1(input: &str) -> u64 {
    let mut score = 0;

    for line in input.lines() {
        let mut stack = String::new();
        for c in line.chars() {
            match c {
                '[' | '(' | '{' | '<' => {
                    stack.push(c);
                }
                closing => {
                    if closing != get_closing_brace(stack.pop().unwrap()) {
                        score += match closing {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!("Invalid character"),
                        };
                    }
                }
            }
        }
    }

    score
}

pub fn solve_part2(input: &str) -> u64 {
    let mut scores = input
        .lines()
        .flat_map(|line| {
            let mut stack = String::new();
            for c in line.chars() {
                match c {
                    '[' | '(' | '{' | '<' => {
                        stack.push(c);
                    }
                    closing => {
                        if closing != get_closing_brace(stack.pop().unwrap()) {
                            return None;
                        }
                    }
                }
            }

            Some(stack.chars().rev().fold(0, |score, c| {
                score * 5
                    + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("Invalid character"),
                    }
            }))
        })
        .collect::<Vec<u64>>();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part1() {
        assert_eq!(26397, solve_part1(&INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(288957, solve_part2(&INPUT));
    }
}
