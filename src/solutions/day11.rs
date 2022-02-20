use std::fs;
use std::io::Error;


fn increment_neighbours(octopuses: &mut Vec<Vec<u8>>, i: usize, j: usize) {
    let shifts = vec![(-1, 0), (-1, -1), (-1, 1), (0, -1), (0, 1), (1, 0), (1, -1), (1, 1)];
    for (x, y) in shifts.iter() {
        if let Some(row) = octopuses.get_mut((x+(i as i8)) as usize) {
            if let Some(octo) = row.get_mut((y+(j as i8)) as usize) {
                *octo += 1;
                if (*octo-1) == 9 { increment_neighbours(octopuses, ((i as i8)+x) as usize, ((j as i8)+y) as usize) }
            }
        }
    }
}

fn explode(octopuses: &mut Vec<Vec<u8>>) -> u64 {
    let mut explosions = 0;
    let mut exceeds = vec![];
    for (i, row) in octopuses.iter().enumerate() {
        for (j, octo) in row.iter().enumerate() {
            if *octo > 9 { exceeds.push((i, j)); }
        }
    }
    for (i, j) in exceeds.into_iter() {
        increment_neighbours(octopuses, i, j);
    }
    octopuses.iter_mut().for_each(|x| x.iter_mut().for_each(|y| if *y > 9 { explosions += 1;
                                                                            *y = 0; }));
    return explosions;
}

fn iteration(octopuses: &mut Vec<Vec<u8>>) -> u64 {
    for row in octopuses.iter_mut() {
        for octo in row.iter_mut() {
            *octo += 1;
        }
    }
    return explode(octopuses);
}

pub fn solver_part1() -> Result<u64, Error> {
    let file_name = "./inputs/day11.txt";
    let content = fs::read_to_string(file_name)?;
    let mut octopuses: Vec<Vec<u8>> = content.split("\n")
                                             .filter(|&x| x != "")
                                             .map(|x| x.split("")
                                                       .filter(|&x| x != "")
                                                       .map(|x| x.parse::<u8>().unwrap())
                                                       .collect()).collect();

    let mut explosions = 0;
    for _ in 0..100 {
        explosions += iteration(&mut octopuses);
    }

    Ok(explosions)
}

pub fn solver_part2() -> Result<u64, Error> {
    let file_name = "./inputs/day11.txt";
    let content = fs::read_to_string(file_name)?;
    let mut octopuses: Vec<Vec<u8>> = content.split("\n")
                                             .filter(|&x| x != "")
                                             .map(|x| x.split("")
                                                       .filter(|&x| x != "")
                                                       .map(|x| x.parse::<u8>().unwrap())
                                                       .collect()).collect();

    let mut step = 0;
    while !octopuses.iter().fold(true, |x, y| x && y.iter().fold(true, |r, s| r && *s==0)) as bool {
        iteration(&mut octopuses);
        step += 1;
    }

    Ok(step)
}
