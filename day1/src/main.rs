use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("The answer is {}", give_me_depth("./day1_input"));
    let duration = Instant::now() - start;
    println!("how quick, this quick: {} μs", duration.as_micros());
}

fn give_me_depth(path: &str) -> i32 {
    io::BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .fold(
            (0, -9999, -9999, -9999),
            |(deeps, a, b, c), x|
                (if a < 0 { 0 } else { deeps + i32::from(a<x) }, b,c,x)).0
}


#[test]
fn test() {
    assert_eq!(give_me_depth("./day1_test"), 2);
}
