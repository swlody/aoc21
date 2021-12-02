pub enum Direction {
    Forward,
    Up,
    Down,
}

pub fn input_generator(input: &str) -> Vec<(Direction, u32)> {
    input
        .lines()
        .map(|line| {
            let (direction, amount) = line.split_once(' ').unwrap();
            (
                match direction {
                    "forward" => Direction::Forward,
                    "up" => Direction::Up,
                    "down" => Direction::Down,
                    _ => panic!("Unexpected input"),
                },
                amount.parse().unwrap(),
            )
        })
        .collect()
}

pub fn solve_part1(directions: &[(Direction, u32)]) -> u32 {
    let (mut position, mut depth) = (0, 0);
    for (direction, amount) in directions {
        match direction {
            Direction::Forward => position += amount,
            Direction::Up => depth -= amount,
            Direction::Down => depth += amount,
        }
    }
    position * depth
}

pub fn solve_part2(directions: &[(Direction, u32)]) -> u32 {
    let (mut aim, mut position, mut depth) = (0, 0, 0);
    for (direction, amount) in directions {
        match direction {
            Direction::Forward => {
                position += amount;
                depth += aim * amount;
            }
            Direction::Up => aim -= amount,
            Direction::Down => aim += amount,
        }
    }
    position * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_part1() {
        let directions = input_generator(INPUT);
        assert_eq!(150, solve_part1(&directions));
    }

    #[test]
    fn test_part2() {
        let directions = input_generator(INPUT);
        assert_eq!(900, solve_part2(&directions));
    }
}
