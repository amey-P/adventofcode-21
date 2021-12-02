use std::fs;
use std::io::Error;


pub fn solver_part1() -> Result<i128, Error> {
    let filename = "./inputs/day2.txt";
    let content: String = fs::read_to_string(filename)?;

    let motions: Vec<Vec<&str>> = content.split("\n").filter(|x| *x != "").map(|x| x.split(" ").collect()).collect();

    let mut horizontal: i128 = 0;
    let mut vertical: i128 = 0;
    for step in motions.iter() {
        let direction = step[0];
        let magnitude = step[1].parse::<i128>().unwrap();

        if direction == "forward" {
            horizontal += magnitude;
        }
        else if direction == "up" {
            vertical -= magnitude;
        }
        else if direction == "down" {
            vertical += magnitude;
        }
    }

    return Ok(horizontal * vertical);
}


pub fn solver_part2() -> Result<i128, Error> {
    let filename = "./inputs/day2.txt";
    let content: String = fs::read_to_string(filename)?;

    let motions: Vec<Vec<&str>> = content.split("\n").filter(|x| *x != "").map(|x| x.split(" ").collect()).collect();

    let mut horizontal: i128 = 0;
    let mut vertical: i128 = 0;
    let mut aim: i128 = 0;
    for step in motions.iter() {
        let direction = step[0];
        let magnitude = step[1].parse::<i128>().unwrap();

        if direction == "forward" {
            horizontal += magnitude;
            vertical += aim * magnitude;
        }
        else if direction == "up" {
            aim -= magnitude;
        }
        else if direction == "down" {
            aim += magnitude;
        }
    }

    return Ok(horizontal * vertical);
}
