use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
        
fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day12_1_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!("The second answer is {}", day12_2_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());
}

fn read_file(path: &str) -> Vec<(String, String)> {
    let lines = io::BufReader::new(File::open(path).unwrap()).lines();
     lines
        .map(|x| {
            let line = x.unwrap();
            let temp:Vec<&str> = line.split('-').collect();
            ((temp[0]).to_owned(), (temp[1]).to_owned())
        })
        .collect()
}

fn create_map(path: &str) -> HashMap<String, Vec<String>> {
    let pairs = read_file(path);

    let mut hashmap:HashMap<String, Vec<String>> = HashMap::with_capacity(pairs.len());
    
    for (key, value) in pairs {
        if let Some(vector) = hashmap.get_mut(key.as_str()) {
            let temp = value.to_string();
            vector.push(temp);
        } else {
            hashmap.insert(key.to_string(), vec![value.to_string()]);
        }
        // if A can go to b, then b can go to A
        if let Some(vector) = hashmap.get_mut(value.as_str()) {
            let temp = key.to_string();
            vector.push(temp);
        } else {
            hashmap.insert(value.to_string(), vec![key.to_string()]);
        }
    }
    hashmap
}

fn day12_1_result(path: &str) -> usize {
    let hashmap = create_map(path);
    let paths = travel(&hashmap, "start", "end", vec![]);
    paths.len()
}

fn travel(map: &HashMap<String, Vec<String>>, start: &str, end: &str, visited: Vec<String>) -> Vec<Vec<String>> {
    let mut local_visited: Vec<String>  = visited.clone(); 
    local_visited.push(start.to_string());

    let caves = match map.get(start) {
        Some(caves) => caves,
        None => return vec![]
    };
    let mut journeys: Vec<Vec<String>> = vec![];

    for cave in caves {
        if local_visited.contains(cave) && cave.to_lowercase() == *cave {
            continue; // not allowed to visit small cave twice
        }
        if cave == end { //short circtuit another trip, we know this is the end...
            let mut copy = local_visited.clone();
            copy.push(cave.to_string());
            journeys.push(copy);
            continue;
        }
        journeys.append(&mut travel(map, cave, end, local_visited.clone()));
    }

    journeys
}


fn day12_2_result(path: &str) -> usize {
    let hashmap = create_map(path);
    let mut paths:Vec<Vec<String>> = vec![];
    let banned_specials = ["start".to_string(), "end".to_string()];
    //let special_cases: Vec<String> = 
    hashmap
        .keys()
        .filter(|x| !banned_specials.contains(x))
        .filter(|&s| s.to_lowercase() == *s)
        .for_each(|special| {
            println!("Today's special is {}", &special);
            paths.append(&mut travel2(&hashmap, "start", "end", vec![], (special, 0)));
        });
    let mut distinct:Vec<Vec<String>> = vec![];

    for ele in paths {
        if !distinct.contains(&ele) {
            distinct.push(ele);
        }
    }
    distinct.len()
}

fn travel2(map: &HashMap<String, Vec<String>>, start: &str, end: &str, visited: Vec<String>, special: (&str, u8)) -> Vec<Vec<String>> {
    let mut local_visited: Vec<String>  = visited.clone(); 
    local_visited.push(start.to_string());
    let mut local_special = special.clone();
    let mut journeys: Vec<Vec<String>> = vec![];

    let caves = match map.get(start) {
        Some(caves) => caves,
        None => return vec![]
    };

    for cave in caves {
        if cave == local_special.0 &&
            local_visited.iter().filter(|cave| cave.as_str() == local_special.0).count() > 1 {
                continue;
            // I think this is experiencing threading issues, debuigging was skipping all over like
            // it does in VS/Rider when on multi-threaded code.
            //if local_special.1 == 2 {
            //    continue;
            //}
            //local_special.1 += 1;
        }
        if cave != local_special.0 && local_visited.contains(cave) && cave.to_lowercase() == *cave {
            continue; // not allowed to visit small cave twice
        }
        if cave == end { //short circtuit another trip, we know this is the end...
            let mut copy = local_visited.clone();
            copy.push(cave.to_string());
            journeys.push(copy);
            continue;
        }
        journeys.append(&mut travel2(map, cave, end, local_visited.clone(), local_special.clone()));
    }

    journeys
}

#[test]
fn read_file_test() {
    let data = read_file("test_input");

    assert_eq!(data[0].0, "start");
    assert_eq!(data[0].1, "A");
}

#[test]
fn day12_1_result_1_test() {
assert_eq!(day12_1_result("test_input"), 10);
}

#[test]
fn day12_1_result_2_test() {
assert_eq!(day12_1_result("test_input2"), 19);
}

#[test]
fn day12_1_result_3_test() {
assert_eq!(day12_1_result("test_input3"), 226);
}


#[test]
fn day12_2_result_1_test() {
    assert_eq!(day12_2_result("test_input"), 36);
}
#[test]
fn day12_2_result_2_test() { 
    assert_eq!(day12_2_result("test_input2"), 103);
}
#[test]
fn day12_2_result_3_test() {
    assert_eq!(day12_2_result("test_input3"), 3509);
}
