use std::fs;
use std::io::Error;
use std::ops::Add;
use std::collections::HashMap;


#[derive(PartialEq, Clone, Eq, Hash, Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Path {
    from: Coordinate,
    to: Coordinate,
}

impl Coordinate {
    fn parse_string(input:String) -> Option<Coordinate> {
        match input.find(",") {
            Some(_) => (),
            None => return None,
        }
        let coords: Vec<u32> = input.split(",").filter(|x| *x != "").map(|x| x.trim().parse::<u32>().unwrap()).collect();
        return Some(Coordinate { x: coords[0], y: coords[1] });
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Path {
    fn parse_string(input: String) -> Option<Path> {
        match input.find("->") {
            Some(_) => (),
            None => return None
        };
        let coords: Vec<String> = input.split("->").map(|x| x.trim().to_string()).collect();
        return Some(Path {
            from: match Coordinate::parse_string(coords[0].clone()) {
                Some(o) => o,
                None => return None,
            },
            to: match Coordinate::parse_string(coords[1].clone()) {
                Some(d) => d,
                None => return None,
            },
        });
    }

    fn expand(&self) -> Vec<Coordinate> {
        let x_iter: Vec<u32>;
        let y_iter: Vec<u32>;
        if self.from.x > self.to.x {
            x_iter = (self.to.x..=self.from.x).rev().map(|x| x).collect();
        }
        else {
            x_iter = (self.from.x..=self.to.x).collect();
        }
        if self.from.y > self.to.y {
            y_iter = (self.to.y..=self.from.y).rev().map(|y| y).collect();
        }
        else {
            y_iter = (self.from.y..=self.to.y).collect();
        }


        let mut expanded: Vec<Coordinate> = vec![];
        for (x, y) in (x_iter.iter()).zip(y_iter.clone().iter()) {
            expanded.push(Coordinate { x: *x, y: *y });
        }

        if expanded.len() != 1 { 
            return expanded; 
        }
        else { expanded.pop(); }
        for x in x_iter.iter() {
            for y in y_iter.iter() {
                expanded.push(Coordinate { x: *x, y: *y });
            }
        }
        return expanded;
    }
}


pub fn solver_part1() -> Result<u64, Error> {
    let file_path = "./inputs/day5.txt";
    let content = fs::read_to_string(file_path)?;
    let all_paths: Vec<Path> = content.split("\n").filter(|x| *x != "").map(|x| Path::parse_string(x.to_string()).unwrap()).collect();
    let paths: Vec<Path> = all_paths.into_iter().filter(|x| x.from.x == x.to.x || x.from.y == x.to.y).collect();

    let mut danger: HashMap<Coordinate, u32> = HashMap::new();
    for path in paths.iter() {
        for coord in path.expand().iter() {
            if let Some(x) = danger.get_mut(coord) {
                *x += 1;
            }
            else { danger.insert(coord.clone(), 1); }
        }
    }

    let answer = danger.into_iter().filter(|(_, y)| *y >= 2).collect::<HashMap<Coordinate, u32>>().len();
    return Ok(answer as u64);
}


pub fn solver_part2() -> Result<u64, Error> {
    let file_path = "./inputs/day5.txt";
    let content = fs::read_to_string(file_path)?;
    let paths: Vec<Path> = content.split("\n").filter(|x| *x != "").map(|x| Path::parse_string(x.to_string()).unwrap()).collect();

    let mut danger: HashMap<Coordinate, u32> = HashMap::new();
    for path in paths.iter() {
        for coord in path.expand().iter() {
            if let Some(x) = danger.get_mut(coord) {
                *x += 1;
            }
            else { danger.insert(coord.clone(), 1); }
        }
    }

    let answer = danger.into_iter().filter(|(_, y)| *y >= 2).collect::<HashMap<Coordinate, u32>>().len();
    return Ok(answer as u64);
}
