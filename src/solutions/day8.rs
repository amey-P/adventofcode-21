use std::fs;
use std::io::Error;
use std::collections::HashMap;


pub fn solver_part1() -> Result<u64, Error> {
    let file_name = "./inputs/day8.txt";
    let content = fs::read_to_string(file_name)?;

    let values: Vec<Vec<Vec<String>>> = content.split("\n")
                                          .map(|l| l.split("|")
                                                    .map(|x| x
                                                             .trim()
                                                             .split(" ")
                                                             .filter(|y| *y != "")
                                                             .map(|y| y
                                                                      .trim()
                                                                      .to_string()).collect()).collect()).collect();
    let mut count = 0;
    for entry in values.iter() {
        match entry.get(1) {
            None => break,
            Some(output) => {
                for signal in output.iter() {
                    match signal.len() {
                        2| 3| 4| 7 => count += 1,
                        _ => (),
                    }
                }
            }
        }

    }

    return Ok(count);
}

fn intersection(string_1: &String, string_2: &String) -> u8 {
    let mut intersect = 0;
    for ch in string_1.chars() {
        if string_2.contains(ch) { intersect += 1; }
    }
    return intersect;
}

fn eq(string_1: &String, string_2: &String) -> bool {
    // Lazy implementation, assumes no repetition
    if string_1.len() != string_2.len() { return false; }
    for ch in string_1.chars() {
        if !string_2.contains(ch) { return false; }
    }
    return true;
}


fn get_mapping(lights: Vec<String>) -> HashMap<u8, String> {
    let mut mapping = HashMap::new();
    
    // Definite Digits: 1, 4, 7, 8
    for leds in lights.iter() {
        if let Some(number) = match leds.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
        {
            mapping.insert(number, leds.clone());
        }
    }

    // Deducing Digits with 6 leds (0, 6 and 9)
    for leds in lights.iter() {
        if leds.len() != 6 {
            continue;
        }

        match intersection(leds, mapping.get(&1).unwrap()) {
            1 => mapping.insert(6, leds.clone()),
            2 => match intersection(leds, mapping.get(&4).unwrap()) {
                3 => mapping.insert(0, leds.clone()),
                4 => mapping.insert(9, leds.clone()),
                x => panic!("6 LEDS but intersection with 4 is {}", x.to_string()),
            },
            x => panic!("6 LEDS but intersection with 1 is {}", x.to_string()),
        };
    }

    // Deducing Digits with 5 LEDS (2, 3 and 5)
    for leds in lights.iter() {
        if leds.len() != 5 {
            continue;
        }
        match intersection(leds, mapping.get(&1).unwrap()) {
            2 => mapping.insert(3, leds.clone()),
            _ => match intersection(leds, mapping.get(&6).unwrap()) {
                5 => mapping.insert(5, leds.clone()),
                4 => mapping.insert(2, leds.clone()),
                x => panic!("5 LEDS but intersection with 6 is {}", x.to_string()),
            },
        };
    }
    
    return mapping;
}


pub fn solver_part2() -> Result<u64, Error> {
    let file_name = "./inputs/day8.txt";
    let content = fs::read_to_string(file_name)?;

    let values: Vec<Vec<Vec<String>>> = content.split("\n").filter(|x| *x != "")
                                          .map(|l| l.split("|")
                                                    .map(|x| x
                                                             .trim()
                                                             .split(" ")
                                                             .filter(|y| *y != "")
                                                             .map(|y| y
                                                                      .trim()
                                                                      .to_string()).collect()).collect()).collect();
    let mut answer = 0;
    for line in values.iter() {
        let mapping = get_mapping(line[0].clone());
        let mut number: u64 = 0;
        for digit in line[1].iter() {
            let mapped = mapping.clone().into_keys().filter(|x| eq(digit, mapping.get(x).unwrap())).collect::<Vec<u8>>()[0];
            number += mapped as u64;
            number *= 10;
        }
        answer += number / 10;
    }
    return Ok(answer);
}
