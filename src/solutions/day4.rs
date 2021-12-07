use std::fs;
use std::io::Error;


#[derive(Clone)]
struct Board {
    cells: Vec<Vec<i64>>,
}

impl Board {
    fn delete(&mut self, number: i64) {
        for row in self.cells.iter_mut() {
            for cell in row {
                if *cell == number { *cell = -1; }
            }
        }
    }

    fn solved(&self) -> bool {
        let num_rows = self.cells.len();
        let num_cols = self.cells[0].len();
        let mut cols = vec![true; num_cols];
        let mut rows = vec![true; num_rows];
        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell != -1 {
                    cols[j] = false;
                    rows[i] = false;
                }
            }
        }
        rows.extend(cols);
        return rows.into_iter().fold(0, |x, y| x + y as i64) > 0;
    }

    fn sum(&self) -> i64 {
        self.cells.iter().map(|x| {
            x.iter().filter(|x| **x != -1).fold(0, |x, y| x + y)
        }).sum()
    }
}


pub fn solver_part1() -> Result<i64, Error> {
    let file_path = "./inputs/day4.txt";
    let content = fs::read_to_string(file_path)?;
    let mut lines = content.trim().split("\n");

    let numbers: Vec<i64> = lines.next().unwrap().split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    let mut board: Vec<Vec<Vec<i64>>> = vec![vec![]];
    for line in lines {
        if line == "" {
            board.push(vec![]);
        }
        else {
            let row: Vec<i64> = line.trim().split(" ").filter(|x| *x != "").map(|x| x.parse::<i64>().unwrap()).collect();
            let last = board.len() - 1;
            board[last].push(row);
        }
    }
    board = board.into_iter().filter(|x| x.len() != 0).collect();
    let mut boards: Vec<Board> = board.into_iter().map(|x| Board{cells: x}).collect();

    let mut solved: Vec<Board> = vec![];
    let mut final_number = 0;
    for number in numbers.iter() {
        boards.iter_mut().for_each(|x| x.delete(*number));
        solved = boards.clone().into_iter().filter(|x| x.solved()).collect();
        if solved.len() > 0 { 
            final_number = *number;
            break;
        }
    }

    let first_board = solved.get(0).unwrap();
    return Ok(first_board.sum() * final_number);
}

pub fn solver_part2() -> Result<i64, Error> {
    let file_path = "./inputs/day4.txt";
    let content = fs::read_to_string(file_path)?;
    let mut lines = content.trim().split("\n");

    let numbers: Vec<i64> = lines.next().unwrap().split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    let mut board: Vec<Vec<Vec<i64>>> = vec![vec![]];
    for line in lines {
        if line == "" {
            board.push(vec![]);
        }
        else {
            let row: Vec<i64> = line.trim().split(" ").filter(|x| *x != "").map(|x| x.parse::<i64>().unwrap()).collect();
            let last = board.len() - 1;
            board[last].push(row);
        }
    }
    board = board.into_iter().filter(|x| x.len() != 0).collect();
    let mut boards: Vec<Board> = board.into_iter().map(|x| Board{cells: x}).collect();

    let mut final_number = 0;
    let mut last_board_sum = 0;
    for number in numbers.iter() {
        boards.iter_mut().for_each(|x| x.delete(*number));
        boards = boards.into_iter().filter(|x| !x.solved()).collect();
        if boards.len() == 1 {
            last_board_sum = boards[0].sum();
        }
        else if boards.len() == 0 { 
            final_number = *number;
            break;
        }
    }

    last_board_sum -= final_number;
    return Ok(last_board_sum * final_number);
}
