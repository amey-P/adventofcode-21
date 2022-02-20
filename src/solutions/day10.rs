use std::fs;
use std::io::Error;


fn match_char(first: char, second: char) -> bool {
    let matches = vec![('<', '>'), 
                       ('{', '}'),
                       ('(', ')'),
                       ('[', ']')];
    return matches.into_iter().fold(false, |x, y| x || ((first == y.0) && (second == y.1)) ||
                                                       ((second == y.0) && (first == y.1)))
}

fn is_open(ch: char) -> bool {
    match ch {
        '<' | '{' | '(' | '[' => return true,
        '>' | '}' | ')' | ']' => return false,
        _ => panic!("Provide valid brackets."),
    };
}

fn eliminate<'a, 'b>(trailing: &'a mut Vec<char>, stack: &'a mut Vec<char>, residue: &'b mut Vec<char>) -> &'b mut Vec<char> {
    if let Some(next_char) = trailing.get(0) {
        if is_open(*next_char) {
            stack.push(*next_char);
            trailing.remove(0);
            return eliminate(trailing, stack, residue);
        }
        else {
            if let Some(final_char) = stack.pop() {
                if match_char(final_char, *next_char) {
                    trailing.remove(0);
                    return eliminate(trailing, stack, residue);
                }
                else {
                    residue.push(final_char);
                    residue.push(*next_char);
                    trailing.remove(0);
                    return eliminate(trailing, stack, residue);
                }
            }
        }
    }
    else { 
        return residue;
    }
    return residue;
}

pub fn solver_part1() -> Result<u64, Error> {
    let file_name = "./inputs/day10.txt";
    let content = fs::read_to_string(file_name)?;
    let mut lines: Vec<Vec<char>> = content.split("\n")
                                           .filter(|&x| x != "")
                                           .map(|x| x.trim().chars().collect())
                                           .collect();

    let mut residue = vec![];
    for line in lines.iter_mut() {
        let mut res = vec![];
        eliminate(line, &mut vec![], &mut res);
        residue.push(res);
    }

    let mut points = 0;
    for res in residue.iter_mut() {
        for ch in res.iter() {
            points += match ch {
                ')' => 3, 
                ']' => 57, 
                '}' => 1197, 
                '>' => 25137, 
                _ => 0,
            }
        }
    }
    return Ok(points);
}


pub fn solver_part2() -> Result<u64, Error> {
    let file_name = "./inputs/day10.txt";
    let content = fs::read_to_string(file_name)?;
    let mut lines: Vec<Vec<char>> = content.split("\n")
                                           .filter(|&x| x != "")
                                           .map(|x| x.trim().chars().collect())
                                           .collect();

    let mut unmatched = vec![];
    for line in lines.iter_mut() {
        let mut res = vec![];
        let mut stack = vec![];
        eliminate(line, &mut stack, &mut res);
        if res.len() == 0 {
            unmatched.push(stack);
        }
    }

    let mut points = vec![];
    for open_bracs in unmatched.iter_mut() {
        // println!("{:?}", open_bracs);
        let mut point = 0;
        for ch in open_bracs.iter().rev() {
            // println!("{}", point);
            point *= 5;
            point += match ch {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                x => panic!("Invalid Bracket: {}", x),
            }
        }
        points.push(point);
    }

    points.sort();
    return Ok(points[points.len()/2]);
}
