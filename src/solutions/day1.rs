use std::fs;
use std::io::Error;

pub fn solver_part1() -> Result<i16, Error> {
    let filename = "./inputs/day1.txt";
    let problem_text: String = fs::read_to_string(filename)?;

    let depths: Vec<i16> = problem_text.split("\n").filter(|x| *x != "").map(|x| x.trim().parse::<i16>().unwrap()).collect();

    let mut answer: i16 = 0;
    for i in 0..(depths.len()-1) {
        if depths.get(i) < depths.get(i+1) {
            answer += 1;
        }
    }
    return Ok(answer);
}

pub fn solver_part2() -> Result<i16, Error> {
    let filename = "./inputs/day1.txt";
    let content: String = fs::read_to_string(filename)?;

    let depths: Vec<i16> = content.split("\n").filter(|x| *x != "").map(|x| x.trim().parse::<i16>().unwrap()).collect();

    let mut answer: i16 = 0;
    for i in 0..(depths.len()-3) {
        let prev: i16 = depths[i..i+3].iter().sum();
        let next: i16 = depths[i+1..i+4].iter().sum();

        answer += (next > prev) as i16;
    }

    return Ok(answer);
}
