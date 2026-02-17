use day5::{solve_part1, solve_part2};

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let ans1 = solve_part1(&input);
    println!("Part 1. Lowest location number is {ans1}");
    let ans2 = solve_part2(&input);
    println!("Part 2. Lowest location number is {ans2}");
}
