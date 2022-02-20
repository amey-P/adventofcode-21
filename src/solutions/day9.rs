use std::fs;
use std::io::Error;


fn is_lowest(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    let center = grid[i][j];
    let mut lowest = true;
    // LEFT
    if j != 0 {
        if let Some(left) = grid[i].get(j-1) {
            lowest = lowest && *left > center;
        }
    }
    // RIGHT
    if let Some(right) = grid[i].get(j+1) {
        lowest = lowest && *right > center;
    }
    // UPPER
    if i != 0 {
        if let Some(upper) = grid.get(i-1) {
            lowest = lowest && upper[j] > center;
        }
    }
    // LOWER
    if let Some(lower) = grid.get(i+1) {
        lowest = lowest && lower[j] > center;
    }
    
    return lowest;
}

pub fn solver_part1() -> Result<u64, Error> {
    let file_name = "./inputs/day9.txt";
    let contents = fs::read_to_string(file_name)?;
    let grid: Vec<Vec<u8>> = contents.split("\n")
                                     .filter(|x| *x != "")
                                     .map(|x| x.split("")
                                               .filter(|y| *y != "")
                                               .map(|y| y.parse::<u8>().unwrap()).collect()).collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut lowest = 0;
    let mut sum = 0;
    for i in 0..height {
        for j in 0..width {
            if is_lowest(&grid, i, j) {
                lowest += 1 as u64;
                sum += grid[i][j] as u64;
            }
        }
    }

    return Ok(lowest + sum);
}


fn explore<'a, 'b>(grid: &'a Vec<Vec<u8>>, i: usize, j: usize, explored: &'b mut Vec<(usize, usize)>) -> &'b mut Vec<(usize, usize)> {
    if grid[i][j] == 9 { return explored; }

    explored.push((i, j));
    // Left
    if i != 0 && explored.iter()
                         .filter(|&x| *x == (i-1, j))
                         .collect::<Vec<&(usize, usize)>>()
                         .len() == 0 {
        explore(&grid, i-1, j, explored);
    }
    // Right
    if i != grid.len()-1 && explored.iter()
                                  .filter(|&x| *x == (i+1, j))
                                  .collect::<Vec<&(usize, usize)>>()
                                  .len() == 0 {
        explore(&grid, i+1, j, explored);
    }
    // Upper
    if j != 0 && explored.iter()
                         .filter(|&x| *x == (i, j-1))
                         .collect::<Vec<&(usize, usize)>>()
                         .len() == 0 {
        explore(&grid, i, j-1, explored);
    }
    // Lower
    if j != grid[0].len()-1 && explored.iter()
                                  .filter(|&x| *x == (i, j+1))
                                  .collect::<Vec<&(usize, usize)>>()
                                  .len() == 0 {
        explore(&grid, i, j+1, explored);
    }
    return explored;
}

pub fn solver_part2() -> Result<usize, Error> {
    let file_name = "./inputs/day9.txt";
    let contents = fs::read_to_string(file_name)?;
    let grid: Vec<Vec<u8>> = contents.split("\n")
                                     .filter(|x| *x != "")
                                     .map(|x| x.split("")
                                               .filter(|y| *y != "")
                                               .map(|y| y.parse::<u8>().unwrap()).collect()).collect();

    let mut sizes = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if is_lowest(&grid, i, j) {
                sizes.push(explore(&grid, i, j, &mut vec![]).len());
            }
        }
    }

    sizes.sort_by(|a, b| b.cmp(a));
    return Ok(sizes[0..3].iter().fold(1, |x, &y| x * y));
}
