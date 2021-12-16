use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Map;
use std::vec::IntoIter;

fn main() {  
    let start = Instant::now();
    println!("The first answer is {}", day10_1_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());
    let start = Instant::now();
    println!("The second answer is {}", day10_2_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());
}
 
fn read_file(path: &str) -> Vec<Vec<char>> { 
    let lines = io::BufReader::new(File::open(path).unwrap()).lines();
        lines.map(|x| x.unwrap().chars().collect::<Vec<char>>()).collect()
}


fn day10_1_result(path: &str) -> u32 {
    common_code(path)
        .map(|(_, result)| result)
        .sum()
}

fn common_code(path: &str) -> Map<IntoIter<Vec<char>>, fn(Vec<char>) -> (Vec<char>, u32)> {
    let data = read_file(path);
    
    data.into_iter().map(|x| {
        x.into_iter().fold((vec![], 0), |(mut openers, error_val), next| {
            if error_val == 0 {
                let result = match next {
                    open @ '(' => { openers.push(open); 0 },
                    open @ '[' => { openers.push(open); 0 },
                    open @ '{' => { openers.push(open); 0 },
                    open @ '<' => { openers.push(open); 0 },

                    ')' => if openers.pop() != Some('(') { 3 } else { 0 },
                    ']' => if openers.pop() != Some('[') { 57 } else { 0 },
                    '}' => if openers.pop() != Some('{') { 1197 } else { 0 },
                    '>' => if openers.pop() != Some('<') { 25137 } else { 0 },

                    _ => panic!("something really bad happened here")
                };
        
                return (openers, result); 
            }
            (openers, error_val)
        })
    })
}

fn day10_2_result(path: &str) -> u64 {
    let mut scores: Vec<u64> = common_code(path)
    .filter(|(_, error_val)| *error_val==0) 
    .map(|(mut openers, _)| {
        let mut score:u64 = 0;
        while let Some(opener) = openers.pop() {
            score = (score * 5) + match opener {
               '(' => 1,
               '[' => 2,
               '{' => 3,
               '<' => 4,
               _ => panic!("something really bad happened here")
            };
        }
        score
    }).collect();

    scores.sort();
    scores[scores.len()/2]
}

#[test]
fn day10_1_result_test() {
    assert_eq!(day10_1_result("test_input"), 26397);
}

#[test]
fn day10_2_result_test() {
    assert_eq!(day10_2_result("test_input"), 288957);
}
