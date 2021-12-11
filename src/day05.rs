use std::{
    cmp::{max, min},
    collections::HashMap,
};

#[derive(Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: u32,
    y: u32,
}

pub struct Line {
    p1: Point,
    p2: Point,
}

pub fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once(" -> ").unwrap();
            let (x1, y1) = p1.split_once(',').unwrap();
            let (x2, y2) = p2.split_once(',').unwrap();

            Line {
                p1: Point {
                    x: x1.parse().unwrap(),
                    y: y1.parse().unwrap(),
                },
                p2: Point {
                    x: x2.parse().unwrap(),
                    y: y2.parse().unwrap(),
                },
            }
        })
        .collect()
}

fn generate_intermediate_points(line: &Line) -> Vec<Point> {
    let mut points = Vec::new();
    if line.p1.x == line.p2.x {
        let min = min(line.p1.y, line.p2.y);
        let max = max(line.p1.y, line.p2.y);
        for y in min..=max {
            points.push(Point { x: line.p1.x, y });
        }
    } else if line.p1.y == line.p2.y {
        let min = min(line.p1.x, line.p2.x);
        let max = max(line.p1.x, line.p2.x);
        for x in min..=max {
            points.push(Point { x, y: line.p1.y });
        }
    }

    points
}

pub fn solve_part1(lines: &[Line]) -> usize {
    let mut counts = HashMap::<Point, u32>::new();

    for line in lines {
        let intermediate_points = generate_intermediate_points(line);
        for point in intermediate_points {
            *counts.entry(point).or_insert(0) += 1;
        }
    }

    counts.values().filter(|&&count| count >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        let input = input_generator(INPUT);
        assert_eq!(5, solve_part1(&input));
    }
}
