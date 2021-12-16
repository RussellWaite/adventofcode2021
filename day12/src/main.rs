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
        hashmap.entry(key.clone()).or_insert_with(Vec::new).push(value.clone());
        hashmap.entry(value.clone()).or_insert_with(Vec::new).push(key.clone()); 
    }
    hashmap
}

fn day12_1_result(path: &str) -> usize {
    let hashmap = create_map(path);
    let paths = travel(&hashmap, "start", "end", vec![]);
    paths.len()
}

fn travel(map: &HashMap<String, Vec<String>>, start: &str, end: &str, mut visited: Vec<String>) -> Vec<Vec<String>> {
    visited.push(start.to_string());

    let caves = match map.get(start) {
        Some(caves) => caves,
        None => return vec![]
    };
    let mut journeys: Vec<Vec<String>> = vec![];

    for cave in caves.iter()  {
        if visited.contains(cave) && cave.to_lowercase() == *cave {
            continue; // not allowed to visit small cave twice
        }
        if cave == end { //short circtuit another trip, we know this is the end...
            let mut copy = visited.clone();
            copy.push(cave.to_string());
            journeys.push(copy);
            continue;
        }
        journeys.append(&mut travel(map, cave, end, visited.clone()));
    }

    journeys
}


fn day12_2_result(path: &str) -> usize {
    let hashmap = create_map(path);
    let mut paths:Vec<Vec<String>> = vec![];
    let banned_specials = ["start".to_string(), "end".to_string()];
    let small_caves: Vec<String> = hashmap.keys()
        .filter(|&key| key.to_lowercase() == *key)
        .map(|key| key.to_owned())
        .collect();
    //let special_cases: Vec<String> = 
    hashmap
        .keys()
        .filter(|x| !banned_specials.contains(x))
        .filter(|&s| s.to_lowercase() == *s)
        .for_each(|special| {
            println!("Today's special is {}", &special);
            paths.append(&mut travel2(&hashmap, "start", "end", vec![], special, &small_caves));
        });
    let mut distinct:Vec<Vec<String>> = vec![];

    for ele in paths {
        if !distinct.contains(&ele) {
            distinct.push(ele);
        }
    }
    distinct.len()
}

fn travel2(map: &HashMap<String, Vec<String>>, start: &str, end: &str, mut visited: Vec<String>, special: &str, small_caves: &Vec<String>) -> Vec<Vec<String>> {
    visited.push(start.to_string());
    let mut journeys: Vec<Vec<String>> = vec![];

    let caves = match map.get(start) {
        Some(caves) => caves,
        None => return vec![]
    };

    for cave in caves {
        if cave == special &&
            visited.iter().filter(|cave| cave.as_str() == special).count() > 1 {
                continue;
            // I think this is experiencing threading issues, debuigging was skipping all over like
            // it does in VS/Rider when on multi-threaded code.
            //if local_special.1 == 2 {
            //    continue;
            //}
            //local_special.1 += 1;
        }
        if cave != special && visited.contains(cave) && small_caves.contains(cave) {
            continue; // not allowed to visit small cave twice
        }
        if cave == end { //short circtuit another trip, we know this is the end...
            let mut copy = visited.clone();
            copy.push(cave.to_string());
            journeys.push(copy);
            continue;
        }
        journeys.append(&mut travel2(map, cave, end, visited.clone(), special, small_caves));
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
