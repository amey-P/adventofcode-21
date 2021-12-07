use std::fs;
use std::io::Error;


fn linear_fuel(heights: &Vec<u64>, position: i64) -> u64 {
    return heights.iter().map(|x| (*x as i128 - position as i128).abs() as u64).sum();
}

fn stepped_fuel(heights: &Vec<u64>, position: i64) -> u64 {
    return heights.iter().map(|x| {
        let n = (*x as i128 - position as i128).abs() as u64;
        (n * (n + 1)) / 2
    }).sum();
}


pub fn solver_part1() -> Result<u64, Error> {
    let file_path = "./inputs/day7.txt";
    let content = fs::read_to_string(file_path)?;

    let heights: Vec<u64> = content.trim().split(",").filter(|x| *x != "").map(|x| x.parse::<u64>().unwrap()).collect();
    let min = *heights.iter().min().unwrap() as i64;
    let max = *heights.iter().max().unwrap() as i64;

    let mut last_cost = linear_fuel(&heights, min);
    for i in min..=max {
        let current_cost = linear_fuel(&heights, i);
        if last_cost < current_cost {
            return Ok(last_cost);
        }
        last_cost = current_cost;
    }

    return Ok(0);
}

pub fn solver_part2() -> Result<u64, Error> {
    let file_path = "./inputs/day7.txt";
    let content = fs::read_to_string(file_path)?;

    let heights: Vec<u64> = content.trim().split(",").filter(|x| *x != "").map(|x| x.parse::<u64>().unwrap()).collect();
    let min = *heights.iter().min().unwrap() as i64;
    let max = *heights.iter().max().unwrap() as i64;

    let mut last_cost = stepped_fuel(&heights, min);
    for i in min..=max {
        let current_cost = stepped_fuel(&heights, i);
        if last_cost < current_cost {
            return Ok(last_cost);
        }
        last_cost = current_cost;
    }

    return Ok(0);
}
