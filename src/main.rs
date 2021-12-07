mod solutions;

fn main() {
    match solutions::day1::solver_part1() {
        Err(e) => println!("Day1-Pt1Error: {:?}", e),
        Ok(solution) => println!("Day 1 - Part1 answer: {}", solution),
    };
    match solutions::day1::solver_part2() {
        Err(e) => println!("Day1 -Pt2 Error: {:?}", e),
        Ok(solution) => println!("Day 1 - Part2 answer: {}", solution),
    };
    match solutions::day2::solver_part1() {
        Err(e) => println!("Day2-Pt1 Error: {:?}", e),
        Ok(solution) => println!("Day 2 - Part1 answer: {}", solution),
    };
    match solutions::day2::solver_part2() {
        Err(e) => println!("Day2-Pt2 Error: {:?}", e),
        Ok(solution) => println!("Day 2 - Part2 answer: {}", solution),
    };
    match solutions::day3::solver_part1() {
        Err(e) => println!("Day3-Pt1 Error: {:?}", e),
        Ok(solution) => println!("Day 3 - Part1 answer: {}", solution),
    };
    match solutions::day3::solver_part2() {
        Err(e) => println!("Day3-Pt2 Error: {:?}", e),
        Ok(solution) => println!("Day 3 - Part2 answer: {}", solution),
    };
    match solutions::day4::solver_part1() {
        Err(e) => println!("Day4-Pt1 Error: {:?}", e),
        Ok(solution) => println!("Day 4 - Part1 answer: {}", solution),
    };
    match solutions::day4::solver_part2() {
        Err(e) => println!("Day4-Pt2 Error: {:?}", e),
        Ok(solution) => println!("Day 4 - Part2 answer: {}", solution),
    };
    match solutions::day5::solver_part1() {
        Err(e) => println!("Day5-Pt1 Error: {:?}", e),
        Ok(solution) => println!("Day 5 - Part1 answer: {}", solution),
    };
    match solutions::day5::solver_part2() {
        Err(e) => println!("Day5-Pt2 Error: {:?}", e),
        Ok(solution) => println!("Day 5 - Part2 answer: {}", solution),
    };
    match solutions::day6::solver_part1() {
        Err(e) => println!("Day6-Pt1 Error: {:?}", e),
        Ok(solution) => println!("Day 6 - Part1 answer: {}", solution),
    };
    match solutions::day6::solver_part2() {
        Err(e) => println!("Day6-Pt2 Error: {:?}", e),
        Ok(solution) => println!("Day 6 - Part2 answer: {}", solution),
    };
    match solutions::day7::solver_part1() {
        Err(e) => println!("Day7-Pt1 Error: {:?}", e),
        Ok(solution) => println!("Day 7 - Part1 answer: {}", solution),
    };
    match solutions::day7::solver_part2() {
        Err(e) => println!("Day7-Pt2 Error: {:?}", e),
        Ok(solution) => println!("Day 7 - Part2 answer: {}", solution),
    };
    // match solutions::day8::solver_part1() {
    //     Err(e) => println!("Day8-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 8 - Part1 answer: {}", solution),
    // };
    // match solutions::day8::solver_part2() {
    //     Err(e) => println!("Day8-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 8 - Part2 answer: {}", solution),
    // };
    // match solutions::day9::solver_part1() {
    //     Err(e) => println!("Day9-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 9 - Part1 answer: {}", solution),
    // };
    // match solutions::day9::solver_part2() {
    //     Err(e) => println!("Day9-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 9 - Part2 answer: {}", solution),
    // };
    // match solutions::day10::solver_part1() {
    //     Err(e) => println!("Day10-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 10 - Part1 answer: {}", solution),
    // };
    // match solutions::day10::solver_part2() {
    //     Err(e) => println!("Day10-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 10 - Part2 answer: {}", solution),
    // };
    // match solutions::day11::solver_part1() {
    //     Err(e) => println!("Day11-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 11 - Part1 answer: {}", solution),
    // };
    // match solutions::day11::solver_part2() {
    //     Err(e) => println!("Day11-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 11 - Part2 answer: {}", solution),
    // };
    // match solutions::day12::solver_part1() {
    //     Err(e) => println!("Day12-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 12 - Part1 answer: {}", solution),
    // };
    // match solutions::day12::solver_part2() {
    //     Err(e) => println!("Day12-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 12 - Part2 answer: {}", solution),
    // };
    // match solutions::day13::solver_part1() {
    //     Err(e) => println!("Day13-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 13 - Part1 answer: {}", solution),
    // };
    // match solutions::day13::solver_part2() {
    //     Err(e) => println!("Day13-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 13 - Part2 answer: {}", solution),
    // };
    // match solutions::day14::solver_part1() {
    //     Err(e) => println!("Day14-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 14 - Part1 answer: {}", solution),
    // };
    // match solutions::day14::solver_part2() {
    //     Err(e) => println!("Day14-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 14 - Part2 answer: {}", solution),
    // };
    // match solutions::day15::solver_part1() {
    //     Err(e) => println!("Day15-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 15 - Part1 answer: {}", solution),
    // };
    // match solutions::day15::solver_part2() {
    //     Err(e) => println!("Day15-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 15 - Part2 answer: {}", solution),
    // };
    // match solutions::day16::solver_part1() {
    //     Err(e) => println!("Day16-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 16 - Part1 answer: {}", solution),
    // };
    // match solutions::day16::solver_part2() {
    //     Err(e) => println!("Day16-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 16 - Part2 answer: {}", solution),
    // };
    // match solutions::day17::solver_part1() {
    //     Err(e) => println!("Day17-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 17 - Part1 answer: {}", solution),
    // };
    // match solutions::day17::solver_part2() {
    //     Err(e) => println!("Day17-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 17 - Part2 answer: {}", solution),
    // };
    // match solutions::day18::solver_part1() {
    //     Err(e) => println!("Day18-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 18 - Part1 answer: {}", solution),
    // };
    // match solutions::day18::solver_part2() {
    //     Err(e) => println!("Day18-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 18 - Part2 answer: {}", solution),
    // };
    // match solutions::day19::solver_part1() {
    //     Err(e) => println!("Day19-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 19 - Part1 answer: {}", solution),
    // };
    // match solutions::day19::solver_part2() {
    //     Err(e) => println!("Day19-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 19 - Part2 answer: {}", solution),
    // };
    // match solutions::day20::solver_part1() {
    //     Err(e) => println!("Day20-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 20 - Part1 answer: {}", solution),
    // };
    // match solutions::day20::solver_part2() {
    //     Err(e) => println!("Day20-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 20 - Part2 answer: {}", solution),
    // };
    // match solutions::day21::solver_part1() {
    //     Err(e) => println!("Day21-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 21 - Part1 answer: {}", solution),
    // };
    // match solutions::day21::solver_part2() {
    //     Err(e) => println!("Day21-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 21 - Part2 answer: {}", solution),
    // };
    // match solutions::day22::solver_part1() {
    //     Err(e) => println!("Day22-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 22 - Part1 answer: {}", solution),
    // };
    // match solutions::day22::solver_part2() {
    //     Err(e) => println!("Day22-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 22 - Part2 answer: {}", solution),
    // };
    // match solutions::day23::solver_part1() {
    //     Err(e) => println!("Day23-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 23 - Part1 answer: {}", solution),
    // };
    // match solutions::day23::solver_part2() {
    //     Err(e) => println!("Day23-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 23 - Part2 answer: {}", solution),
    // };
    // match solutions::day24::solver_part1() {
    //     Err(e) => println!("Day24-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 24 - Part1 answer: {}", solution),
    // };
    // match solutions::day24::solver_part2() {
    //     Err(e) => println!("Day24-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 24 - Part2 answer: {}", solution),
    // };
    // match solutions::day25::solver_part1() {
    //     Err(e) => println!("Day25-Pt1 Error: {:?}", e),
    //     Ok(solution) => println!("Day 25 - Part1 answer: {}", solution),
    // };
    // match solutions::day25::solver_part2() {
    //     Err(e) => println!("Day25-Pt2 Error: {:?}", e),
    //     Ok(solution) => println!("Day 25 - Part2 answer: {}", solution),
    // };
}
