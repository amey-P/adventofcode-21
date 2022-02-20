use std::fs;
use std::io::Error;


fn transform(coord: &Vec<i32>, fold: (char, i32)) -> Option<Vec<i32>> {
    let axis;
    if fold.0 == 'x' { axis = 0; }
    else { axis = 1; }

    if coord[axis] == fold.1 { return None; }

    let mut transformed = coord.clone();
    transformed[axis] -= ((fold.1 < coord[axis]) as i32) * 
                         2 * (coord[axis] - fold.1);
    Some(transformed)
}

pub fn solver_part1() -> Result<usize, Error> {
    let file_name = "./inputs/day13.txt";
    let content = fs::read_to_string(file_name)?;
    let lines: Vec<String> = content.split("\n")
                                    .filter(|x| *x != "")
                                    .map(|x| x.to_string())
                                    .collect();
    let coords: Vec<Vec<i32>> = lines.iter()
                                     .filter(|x| x.contains(","))
                                     .map(|x| x.split(",")
                                               .map(|y| y.parse::<i32>().unwrap())
                                               .collect())
                                     .collect();
    let folds: Vec<(char, i32)> = lines.iter()
                                       .filter(|x| x.contains("="))
                                       .map(|x| {
                                                    let fold: Vec<&str> = x.split("=").collect();
                                                    (fold[0].chars().last().unwrap(), fold[1].parse::<i32>().unwrap())
                                       }).collect();

    let mut transformed: Vec<Vec<i32>> = coords.iter()
                                               .map(|x| transform(x, folds[0]))
                                               .filter(|x| match x { 
                                                                Some(_) => true,
                                                                None => false,
                                                                   })
                                               .map(|x| x.unwrap())
                                               .collect();
    transformed.sort();
    transformed.dedup();

    Ok(transformed.len())
}

pub fn solver_part2() -> Result<String, Error> {
    let file_name = "./inputs/day13.txt";
    let content = fs::read_to_string(file_name)?;
    let lines: Vec<String> = content.split("\n")
                                    .filter(|x| *x != "")
                                    .map(|x| x.to_string())
                                    .collect();
    let coords: Vec<Vec<i32>> = lines.iter()
                                     .filter(|x| x.contains(","))
                                     .map(|x| x.split(",")
                                               .map(|y| y.parse::<i32>().unwrap())
                                               .collect())
                                     .collect();
    let folds: Vec<(char, i32)> = lines.iter()
                                       .filter(|x| x.contains("="))
                                       .map(|x| {
                                                    let fold: Vec<&str> = x.split("=").collect();
                                                    (fold[0].chars().last().unwrap(), fold[1].parse::<i32>().unwrap())
                                       }).collect();


    let mut transformed: Vec<Vec<i32>> = vec![];
    for coord in coords.iter() {
        let mut flipped = coord.clone();
        let mut completed = false;
        for fold in folds.iter() {
            flipped = match transform(&flipped, *fold) {
                Some(x) => x,
                None => break,
            };
            completed = true;
        }
        if completed { transformed.push(flipped); }
    }
    transformed.sort();
    transformed.dedup();

    let max_x = transformed.iter().max_by_key(|x| x[0]).unwrap()[0];
    let max_y = transformed.iter().max_by_key(|y| y[1]).unwrap()[1];
    for j in 0..max_y as usize {
        for i in 0..max_x as usize {
            let mark = match transformed.contains(&vec![i as i32, j as i32]) {
                true => 'X',
                false => '.',
            };
            print!("{}", mark);
        }
        println!("");
    }
    Ok("Above".to_string())
}
