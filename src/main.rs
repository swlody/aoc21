mod day01;
mod day02;

aoc_main::main! {
    year 2021;
    day01 : input_generator => solve_part1, solve_part2;
    day02 : input_generator => solve_part1, solve_part2;
}
