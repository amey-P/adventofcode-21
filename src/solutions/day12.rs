use std::fs;
use std::io::Error;
use std::collections::HashMap;


fn travel(path: &mut Vec<String>, 
          connections: &HashMap<String, Vec<String>>, 
          criteria: fn(&String, &Vec<String>) -> bool) -> Vec<Vec<String>> {
    let last_node = path.last().unwrap();
    if last_node == "end" { return vec![path.clone()]; }

    let mut all_paths = vec![];
    if let Some(connected_to) = connections.get(last_node) {
        let revisitable = connected_to.iter().filter(|x| criteria(x, &path));
        for next_node in revisitable {
            let mut this_path = path.clone();
            this_path.push(next_node.to_string());
            all_paths.extend(travel(&mut this_path, connections, criteria));
        }
    }
    return all_paths;
}


pub fn solver_part1() -> Result<usize, Error> {
    let file_name = "./inputs/day12.txt";
    let content = fs::read_to_string(file_name)?;

    let mut nodes: HashMap<String, Vec<String>> = HashMap::new();
    fn insert(nodes: &mut HashMap<String, Vec<String>>, a: String, b: String) {
        if let Some(list) = nodes.get_mut(&a) {
            list.push(b);
        }
        else {
            nodes.insert(a, vec![b]);
        }
    }
    content.split("\n")
           .filter(|x| *x != "")
           .for_each(|x| {
               let splitted: Vec<&str> = x.split("-").collect();
               let a = splitted[0].to_string();
               let b = splitted[1].to_string();
               insert(&mut nodes, a.clone(), b.clone());
               insert(&mut nodes, b, a);
           });

    fn allow_once(node: &String, path: &Vec<String>) -> bool {
        if *node == "start".to_string() { return false; }
        if node.chars().nth(0).unwrap().is_lowercase() && path.contains(node) {
            return false;
        }
        return true;
    }

    let paths: Vec<Vec<String>> = travel(&mut vec![String::from("start")], &nodes, allow_once);
    
    Ok(paths.len())
}

pub fn solver_part2() -> Result<usize, Error> {
    let file_name = "./inputs/day12.txt";
    let content = fs::read_to_string(file_name)?;

    let mut nodes: HashMap<String, Vec<String>> = HashMap::new();
    fn insert(nodes: &mut HashMap<String, Vec<String>>, a: String, b: String) {
        if let Some(list) = nodes.get_mut(&a) {
            list.push(b);
        }
        else {
            nodes.insert(a, vec![b]);
        }
    }
    content.split("\n")
           .filter(|x| *x != "")
           .for_each(|x| {
               let splitted: Vec<&str> = x.split("-").collect();
               let a = splitted[0].to_string();
               let b = splitted[1].to_string();
               insert(&mut nodes, a.clone(), b.clone());
               insert(&mut nodes, b, a);
           });

    fn allow_twice(node: &String, path: &Vec<String>) -> bool {
        if *node == "start".to_string() { return false; }
        if node.chars().nth(0).unwrap().is_lowercase() {
            let count = path.iter().fold(0, |x, y| x + ((y == node) as u8));
            if count == 0 { 
                return true; 
            }
            else if count == 1 { 
                for letter in path.iter() {
                    let count = path.iter()
                                    .filter(|x| x.chars().nth(0).unwrap().is_lowercase())
                                    .fold(0, |x, y| x + ((y == letter) as u8));
                    if count == 2 { return false; }
                }
                return true; 
            }
            else if count > 1 { return false; }
        }
        return true;
    }

    let paths: Vec<Vec<String>> = travel(&mut vec![String::from("start")], &nodes, allow_twice);
    Ok(paths.len())
}
