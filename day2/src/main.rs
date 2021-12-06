use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", distance_travelled("./input"));
    let duration = Instant::now() - start;
    println!("how quick, this quick: {} μs", duration.as_micros());
    let start = Instant::now();
    println!("The second answer is {}", distance_travelled_second_star("./input"));
    let duration = Instant::now() - start;
    println!("how quick, this quick: {} μs", duration.as_micros());
}

fn distance_travelled(path: &str) -> i32 {
    let travelled = io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|x|
            match x.unwrap().split_once(' ') {
                Some((key, value)) =>
                    match key {
                        "forward" => (value.parse::<i32>().unwrap(), 0),
                        "up" => (0, -(value.parse::<i32>().unwrap())),
                        "down" => (0, (value.parse::<i32>().unwrap())),
                        _ => panic!("well you've read something that wasn't meant to be read...")
                    }
                None => panic!("expected a key-value pair")
            })
        .fold((0, 0), |(fwd, height), last_reading| (fwd + last_reading.0, height + last_reading.1));
    travelled.0 * travelled.1
}


fn distance_travelled_second_star(path: &str) -> i32 {
    let traveled = io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|x|
            match x.unwrap().split_once(' ') {
                Some((key, value)) =>
                    match key {
                        "forward" => (value.parse::<i32>().unwrap(), 0),
                        "up" => (0, -(value.parse::<i32>().unwrap())),
                        "down" => (0, (value.parse::<i32>().unwrap())),
                        _ => panic!("well you've read something that wasn't meant to be read...")
                    }
                None => panic!("expected a key-value pair")
            })
        .fold((0, 0, 0),
            |(horizontal, depth, aim), (forward, down)|
                (horizontal + forward, (forward * aim) + depth, aim + down));
    traveled.0 * traveled.1
}

#[test]
fn test_2_1() {
    assert_eq!(distance_travelled("./test1"), 150);

    // found this by running after above worked out, site said correct
    assert_eq!(distance_travelled("./input"), 1451208)
}

#[test]
fn test_2_2() {
    // horizontal position of 15 and a depth of 60. (Multiplying these produces 900.)
    assert_eq!(distance_travelled_second_star("./test1"), 900);
    /*

        forward 5 adds 5 to your horizontal position, a total of 5. Because your aim is 0, your depth does not change.
        down 5 adds 5 to your aim, resulting in a value of 5.
        forward 8 adds 8 to your horizontal position, a total of 13. Because your aim is 5, your depth increases by 8*5=40.
        up 3 decreases your aim by 3, resulting in a value of 2.
        down 8 adds 8 to your aim, resulting in a value of 10.
        forward 2 adds 2 to your horizontal position, a total of 15. Because your aim is 10, your depth increases by 2*10=20 to a total of 60.

    After following these new instructions, you would have a horizontal position of 15 and a depth of 60. (Multiplying these produces 900.)
    */

    // my calc says 1620141160 and the website liked it!
}