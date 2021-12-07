use std::fs;
use std::io::Error;
use std::collections::HashMap;


fn day_vec(school: &mut Vec<u8>) -> &mut Vec<u8> {
    let mut new_fish = vec![];
    for fish in school.iter_mut() {
        if *fish == 0 {
            new_fish.push(8);
            *fish = 6;
        }
        else { *fish -= 1; }
    }
    school.extend(new_fish);

    return school;
}

fn day_hash(school: &mut HashMap<u8, u128>) -> &mut HashMap<u8, u128> {
    let old_school = school.clone();
    for day in 0..8 {
        if let Some(x) = school.get_mut(&(day as u8)) {
            *x = *old_school.get(&(day+1)).unwrap();
        }
    }

    *school.get_mut(&8).unwrap() = *old_school.get(&0).unwrap();
    *school.get_mut(&6).unwrap() += *old_school.get(&0).unwrap();

    return school;
}

pub fn solver_part1() -> Result<usize, Error> {
    let file_path = "./inputs/day6.txt";
    let content = fs::read_to_string(file_path)?;
    let experiment_days = 80;

    let mut school: Vec<u8> = content.split(",").filter(|x| *x != "").map(|x| x.trim().parse::<u8>().unwrap()).collect();
    for _ in 1..=experiment_days {
        school = day_vec(&mut school).clone();
    }
    return Ok(school.len());
}

pub fn solver_part2() -> Result<u128, Error> {
    let file_path = "./inputs/day6.txt";
    let content = fs::read_to_string(file_path)?;
    let experiment_days = 256;

    let school_vec: Vec<u8> = content.split(",").filter(|x| *x != "").map(|x| x.trim().parse::<u8>().unwrap()).collect();
    let mut school: HashMap<u8, u128> = HashMap::new();
    for i in 0..=8 {
        school.insert(i, 0);
    }
    for fish in school_vec {
        if let Some(x) = school.get_mut(&fish) {
            *x += 1;
        }
    }
    
    for _ in 1..=experiment_days {
        school = day_hash(&mut school).clone();
    }
    return Ok(school.values().sum());
}
