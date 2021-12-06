use std::fs::{File};
use std::io::{Read};
use std::time::Instant;
use std::collections::HashMap;

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day5_1_result("./input"));
    let duration = Instant::now() - start;
    println!("how quick, this quick: {} μs", duration.as_micros());
    let start = Instant::now();
    println!("The second answer is {}", day5_2_result("./input"));
    let duration = Instant::now() - start;
    println!("how quick, this quick: {} μs", duration.as_micros());
}

fn read_file(path: &str) -> String {
    let mut data = String::new();
    let did_read = File::open(path).unwrap().read_to_string(&mut data);
    if did_read.is_err() {
        panic!("Could not read file");
    }
    data
}

fn pull_coords_from_data(path: &str) -> Vec<(i32, i32, i32, i32)> {
    let data = read_file(path);
    let lines = data.split("\n");
    lines.map(|s| parse_line_get_quad_tuple(s)).collect()
}

fn parse_line_get_quad_tuple(line: &str) -> (i32, i32, i32, i32) {
    let raw_doubles: Vec<&str> = line.split("->").collect();
    let raw_quads = raw_doubles.into_iter()
        .map(|s| s.trim().split(",")).into_iter()
        .fold(Vec::new(), |mut acc, b| {
            acc.append(&mut b.collect());
            acc
        });
    let values: Vec<i32> = raw_quads.into_iter().map(|s| s.parse::<i32>().unwrap()).collect();
    (values[0], values[1], values[2], values[3])
}

fn day5_1_result(path: &str) -> usize {
    let data = pull_coords_from_data(path);
    let seabed: HashMap<i32, i32> = build_seabed_from_coords(data);
    seabed.into_values().filter(|&x| x >= 2).count()
}

fn build_seabed_from_coords(data: Vec<(i32, i32, i32, i32)>) -> HashMap<i32, i32> {
    let mut seabed = HashMap::new();

    for (x1, y1, x2, y2) in data {
        if x1 == x2 {
            let low = if y1 < y2 { y1 } else { y2 };
            let high = if y1 > y2 { y1 } else { y2 };
            // vertical line
            for i in low..=high {
                let vents = seabed.entry(x1 * 1000 + i).or_insert(0);
                *vents += 1;
            }
        }
        if y1 == y2 {
            let low = if x1 < x2 { x1 } else { x2 };
            let high = if x1 > x2 { x1 } else { x2 };
            // horizontal line
            for i in low..=high {
                let vents = seabed.entry(i * 1000 + y1).or_insert(0);
                *vents += 1;
            }
        }
    }
    seabed
}

fn day5_2_result(path: &str) -> usize {
    let data = pull_coords_from_data(path);
    let seabed: HashMap<i32, i32> = build_seabed_from_coords_inc_diagonal(data);
    seabed.into_values().filter(|&x| x >= 2).count()
}

fn build_seabed_from_coords_inc_diagonal(data: Vec<(i32, i32, i32, i32)>) -> HashMap<i32, i32> {
    let mut seabed = HashMap::new();

    for (x1, y1, x2, y2) in data {
        match (x1, y1, x2, y2) {
            (x1, y1, x2, y2) if x1 == x2 => {
                let low = if y1 < y2 { y1 } else { y2 };
                let high = if y1 > y2 { y1 } else { y2 };
                // vertical line
                for i in low..=high {
                    let vents = seabed.entry(x1 * 1000 + i).or_insert(0);
                    *vents += 1;
                }
            },
            (x1, y1, x2, y2) if y1 == y2 => {
                let low = if x1 < x2 { x1 } else { x2 };
                let high = if x1 > x2 { x1 } else { x2 };
                // horizontal line
                for i in low..=high {
                    let vents = seabed.entry(i * 1000 + y1).or_insert(0);
                    *vents += 1;
                }
            },
            (x1, y1, x2, y2) if x1 - x2 > 0 && y1 - y2 > 0 => {
                //back and up
                let mut y = y2;
                for i in x2..=x1 {
                    let vents = seabed.entry(i * 1000 + y).or_insert(0);
                    *vents += 1;
                    y += 1
                }
            },
            (x1, y1, x2, y2) if x1 - x2 > 0 && y1 - y2 < 0 => {
                //back and down
                let mut y = y2;
                for i in x2..=x1 {
                    let vents = seabed.entry(i * 1000 + y).or_insert(0);
                    *vents += 1;
                    y -= 1
                }
            },
            (x1, y1, x2, y2) if x1 - x2 < 0 && y1 - y2 > 0 => {
                // forwards and up
                let mut y = y1;
                for i in x1..=x2 {
                    let vents = seabed.entry(i * 1000 + y).or_insert(0);
                    *vents += 1;
                    y -= 1
                }
            },
            (x1, y1, x2, y2) if x1 - x2 < 0 && y1 - y2 < 0 => {
                // forwards and down
                let mut y = y1;
                for i in x1..=x2 {
                    let vents = seabed.entry(i * 1000 + y).or_insert(0);
                    *vents += 1;
                    y += 1
                }
            },
            _ => {}
        }
    }
    seabed
}

#[test]
fn day5_2_test() {
    assert_eq!(day5_2_result("test_input"), 12);
    // 26285 - TOO HIGH - had to use match and
    // map y assignment and increment carefully on diagonal
    // 22088 was right
    assert_eq!(day5_2_result("input"), 22088);
}

#[test]
fn day5_1_test() {
    let result = pull_coords_from_data("input");
    assert_eq!(result.into_iter().count(), 500);
    assert_eq!(day5_1_result("test_input"), 5); // 26285 - TOO HIGH
    // 8111 was right answer
    assert_eq!(day5_1_result("input"), 8111); // 26285 - TOO HIGH
}

#[test]
fn parse_line_get_quad_tuple_test() {
    let input = "0,9 -> 5,9";
    let (x1, y1, x2, y2) = parse_line_get_quad_tuple(input);
    assert_eq!(x1, 0);
    assert_eq!(y1, 9);
    assert_eq!(x2, 5);
    assert_eq!(y2, 9);
}