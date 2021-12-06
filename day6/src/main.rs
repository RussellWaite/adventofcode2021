use std::fs::{File};
use std::io::{Read};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day6_1_result("./input", 80));
    let duration = Instant::now() - start;
    println!("how quick, this quick: {} μs", duration.as_micros());
    let start = Instant::now();
    println!("The second answer is {}", day6_2_result("./input", 256));
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

fn parse_input_get_lanterns(path: &str) -> LanternsState {
    let line = read_file(path);
    // let lanterns= line.split(",");
    // println!("{:?}", lanterns);
    let lanterns: Vec<u8> = line.split(",")
        .into_iter()
        .map(|s| s.parse::<u8>().unwrap()).collect();

    LanternsState::new_with_schoool(lanterns.into_iter().map(|i| Lantern::new(i)).collect())
}

fn day6_1_result(path: &str, days_to_age: i32) -> u32 {
    let mut lanterns = parse_input_get_lanterns(path);
    lanterns.age_by_days(days_to_age);
    lanterns.count_lanterns()
}
// modeling like this as massively inefficient, as we see for 6.2 as my 32Gb laptop ran out of memory.
// Think it's do do with how the instances are referred to. it's not just 8 bits for the u8, there
// is memory taken up for pointers to it
struct LanternsState {
    school: Vec<Lantern>,
}

struct Lantern {
    age: u8,
}

impl std::fmt::Debug for Lantern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({})", self.age)
    }
    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     f.debug_struct("Point")
    //         .field("x", &self.x)
    //         .field("y", &self.y)
    //         .finish()
    // }
}

impl Lantern {
    pub fn new(age: u8) -> Lantern {
        Lantern {
            age,
        }
    }
    pub fn age(&mut self) -> bool {
        if self.age == 0 {
            self.age = 6;
            return true;
        }
        self.age -= 1;
        false
    }
}

impl LanternsState {
    pub fn new() -> LanternsState {
        LanternsState {
            school: vec![],
        }
    }
    pub fn new_with_schoool(lanterns: Vec<Lantern>) -> LanternsState {
        LanternsState {
            school: lanterns,
        }
    }
    pub fn age_by_days(&mut self, days: i32) {
        for day in 0..days {
            self.age_lanterns();
            println!("Day {} done, {} in school", day, self.school.len());
        }
    }
    pub fn add_lantern(&mut self, age: u8) {
        self.school.push(Lantern::new(age));
    }

    pub fn age_lanterns(&mut self) {
        let mut new_starters = 0;
        for l in self.school.iter_mut() {
            if l.age() {
                new_starters += 1;
            }
        }
        for _ in 0..new_starters {
            self.add_lantern(8);
        }
        //self.display_lanterns()
    }
    pub fn count_lanterns(&self) -> u32 {
        self.school.len() as u32
    }
    #[allow(dead_code)]
    fn display_lanterns(&mut self) {
        println!("{:?}", self.school);
    }
}

#[test]
fn day6_1_test() {
    assert_eq!(day6_1_result("./test_input", 18), 26);//
    assert_eq!(day6_1_result("./test_input", 80), 5934);
    assert_eq!(day6_1_result("./input", 80), 380612);
}

#[test]
fn lanterns_test() {
    let mut lanterns = LanternsState::new();
    lanterns.add_lantern(3);
    lanterns.add_lantern(4);
    lanterns.add_lantern(3);
    lanterns.add_lantern(1);
    lanterns.add_lantern(2);
    lanterns.age_by_days(18);
    assert_eq!(lanterns.count_lanterns(), 26);
}

fn day6_2_result(path: &str, days_to_go: u32) -> u64 {
    let results = group_by_age(path);
    how_many_fish(&results, days_to_go)
}

fn group_by_age(path: &str) -> Vec<u64> {
    let data = parse_input_get_byte_array(path);
    let mut days: Vec<u64> = vec![0; 9];
    data.into_iter().for_each(|i| {
        days[i as usize] += 1;
    });
    days
}

fn parse_input_get_byte_array(path: &str) -> Vec<u8> {
    let line = read_file(path);

    line.split(",")
        .into_iter()
        .map(|s| s.parse::<u8>().unwrap()).collect()
}

fn how_many_fish(start: &Vec<u64>, days_to_count: u32) -> u64 {
    let mut days = start.clone();
    for day in 0..days_to_count {
        let spawning: u64 = days[0];
        days[0] = days[1];
        days[1] = days[2];
        days[2] = days[3];
        days[3] = days[4];
        days[4] = days[5];
        days[5] = days[6] ;
        days[6] = days[7] + spawning; // parent
        days[7] = days[8];
        days[8] = spawning; // baby
        println!("Day {} done, {} lantern fishies", day, days.clone().into_iter().fold(0, |acc, x| acc + x));
    }
    days.into_iter().fold(0 as u64, |acc: u64, x| acc + x)
}

#[test]
fn group_by_age_test() {
    let results = group_by_age("./test_input");

    assert_eq!(results[0], 0);
    assert_eq!(results[1], 1);
    assert_eq!(results[2], 1);
    assert_eq!(results[3], 2);
    assert_eq!(results[4], 1);
    assert_eq!(results[5], 0);
    assert_eq!(results[6], 0);
    assert_eq!(results[7], 0);
    assert_eq!(results[8], 0);

    assert_eq!(how_many_fish(&results, 18), 26);
    assert_eq!(how_many_fish(&results, 80), 5934);
}