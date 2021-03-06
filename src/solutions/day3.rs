use std::fs;
use std::io::Error;


pub fn solver_part1() -> Result<u64, Error> {
    let file_path = "./inputs/day3.txt";
    let content = fs::read_to_string(file_path)?;

    let binary_vec: Vec<Vec<u64>> = content.split("\n")
        .filter(|x| *x != "")
        .map(|x| x.split("").filter(|x| *x != "")
             .map(|x| x.parse::<u64>().unwrap()).collect())
        .collect();

    let mut vertical_sum: Vec<u64> = vec![0; binary_vec[0].len()];
    for row in binary_vec.iter() {
        for (i, cell) in row.iter().enumerate() {
            vertical_sum[i] += cell;
        }
    }

    let total_length = binary_vec.len();
    let mut epsilon: u64 = 0;
    let mut gamma: u64 = 0;
    for (i, sum) in vertical_sum.iter().rev().enumerate() {
        if *sum <= (total_length as u64 / 2) {
            epsilon += u64::pow(2, i as u32);
        }
        else {
            gamma += u64::pow(2, i as u32);
        }
    }

    return Ok(epsilon * gamma);

}


pub fn solver_part2() -> Result<u64, Error> {
    let file_path = "./inputs/day3.txt";
    let content = fs::read_to_string(file_path)?;

    let binary_vec: Vec<Vec<u64>> = content.split("\n")
        .filter(|x| *x != "")
        .map(|x| x.split("").filter(|x| *x != "")
             .map(|x| x.parse::<u64>().unwrap()).collect())
        .collect();
    let line_size = binary_vec[0].len();

    let mut oxygen: Vec<Vec<u64>> = binary_vec.clone();
    let mut i = 0;
    while (oxygen.len() > 1) && (i < line_size) {
        let ones = oxygen.iter().fold(0, |x, y| x + y[i]);
        let length = oxygen.len() as u64;
        if ones >= (length as f64/2.0).ceil() as u64 {
            oxygen = oxygen.into_iter().filter(|x| x[i] == 0).collect();
        }
        else {
            oxygen = oxygen.into_iter().filter(|x| x[i] == 1).collect();
        }
        i += 1;
    }

    let mut carbon: Vec<Vec<u64>> = binary_vec.clone();
    let mut i = 0;
    while (carbon.len() > 1) && (i < line_size) {
        let ones = carbon.iter().fold(0, |x, y| x + y[i]);
        let length = carbon.len() as u64;
        let zeros = length - ones;
        if zeros <= (length as f64/2.0).ceil() as u64 {
            carbon = carbon.into_iter().filter(|x| x[i] == 1).collect();
        }
        else {
            carbon = carbon.into_iter().filter(|x| x[i] == 0).collect();
        }
        i += 1;
    }

    let oxygen_rating = oxygen[0].iter().rev().enumerate()
        .fold(0, |x, (i, y)| x + y * u64::pow(2, i as u32));
    let carbon_rating = carbon[0].iter().rev().enumerate()
        .fold(0, |x, (i, y)| x + y * u64::pow(2, i as u32));

    return Ok(oxygen_rating * carbon_rating);
}
