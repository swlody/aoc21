mod day01;
mod day02;
mod day03;
mod day05;
mod day10;

aoc_main::main! {
    year 2021;
    day01 : input_generator => solve_part1, solve_part2;
    day02 : input_generator => solve_part1, solve_part2;
    day03 => solve_part1, solve_part2;
    day05 : input_generator => solve_part1;
    day10 => solve_part1, solve_part2;
}
