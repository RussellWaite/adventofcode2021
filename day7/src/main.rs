use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn main() {
    println!("Hello, world!");
    let start = Instant::now();
    println!("The first answer is {}", day7_1_result("./input"));
    let duration = Instant::now() - start;
    println!("how quick, this quick: {} μs", duration.as_micros());
    let start = Instant::now();
    println!("The second answer is {}", day7_2_result("./input"));
    let duration = Instant::now() - start;
    println!("how quick, this quick: {} μs", duration.as_micros());
    let start = Instant::now();
    println!("The second (PERF) answer is {}", day7_2p_result("./input", calc_fuel_used));
    let duration = Instant::now() - start;
    println!("how quick, this quick: {} μs", duration.as_micros());
}

fn read_file(path: &str) -> Vec<i32> {
    std::fs::read_to_string(path)
        .unwrap()
        .trim_end()
        .split(',')
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}


fn day7_1_result(path: &str) -> i32 {
    let data = read_file(path);

    let (min, max, sum, count) = data
        .iter()
        .fold((0, 0, 0, 0), |(min, max, sum, count), &x| {
            (
                if x < min { x } else { min },
                if x > max { x } else { max },
                sum + x,
                count + 1,
            )
        });

    //let mid_range:i32 = ((max - min) /2) as i32;
    //let mean:i32 = (sum/count) as i32;
    //let direction:i32 = match mean - mid_range {
    //    x if x<0 => -1,
    //    _ => 1
    //};
    //let direction: i32  = 0;
    let mut fuel_used = i32::MAX;
    //let mut target_position:i32 = mean;

    let direction = 1;
    let mut target_position = 0;
    let mut answer = 0;

    loop {
        let total_distance_from_position = data
            .iter()
            .fold(0, |sum, &x| sum + (x - target_position).abs());
        if total_distance_from_position < fuel_used {
            fuel_used = total_distance_from_position;
        }
        answer = fuel_used;

        target_position += 1;
        if target_position >= 1000 {
            break;
        }
    }
    answer // too high: 248413364 :-(  nor is 105462913
}

fn day7_2_result(path: &str) -> i32 {
    let data = read_file(path); //parse_input_get_positions(path);

    let (min, max, sum, count) = data
        .iter()
        .fold((0, 0, 0, 0), |(min, max, sum, count), &x| {
            (
                if x < min { x } else { min },
                if x > max { x } else { max },
                sum + x,
                count + 1,
            )
        });

    //let mid_range:i32 = ((max - min) /2) as i32;
    //let mean:i32 = (sum/count) as i32;
    //let direction:i32 = match mean - mid_range {
    //    x if x<0 => -1,
    //    _ => 1
    //};
    //let direction: i32  = 0;
    let mut fuel_used = i32::MAX;
    //let mut target_position:i32 = mean;

    let direction = 1;
    let mut target_position = 0;
    let mut answer = 0;

    loop {
        let total_distance_from_position = data
            .iter()
            .fold(0, |sum, &x| sum + calc_fuel_used(x, target_position));
        if total_distance_from_position < fuel_used {
            fuel_used = total_distance_from_position;
        }
        answer = fuel_used;
        //println!("iteration {},  dist from pos {}, fuel_used {}, answer {}", target_position, total_distance_from_position, fuel_used, answer);

        //target_position+=direction;
        target_position += 1;
        if target_position >= 1000 {
            break;
        }
    }
    answer
}

fn day7_2p_result(path: &str, calc_fuel: fn (i32, i32) -> i32) -> i32 {
    let data = read_file(path);

    let (min, max, sum, count) = data
        .iter()
        .fold((0, 0, 0, 0), |(min, max, sum, count), &x| {
            (
                if x < min { x } else { min },
                if x > max { x } else { max },
                sum + x,
                count + 1,
            )
        });

    let mid_range: i32 = ((max - min) / 2) as i32;
    let mean: i32 = (sum / count) as i32;
    let direction: i32 = match mean - mid_range {
        x if x < 0 => -1,
        _ => 1,
    };
    let mut fuel_used = i32::MAX;
    let mut target_position: i32 = mean;
    let mut answer = 0;

    loop {
        let total_distance_from_position = data
            .iter()
            .fold(0, |sum, &x| sum + calc_fuel(x, target_position)); 

        if total_distance_from_position < fuel_used {
            fuel_used = total_distance_from_position;
            answer = fuel_used;
            break;
        }

        target_position += direction;
        if target_position >= 1000 || target_position < 0 {
            break;
        }
    }
    answer
}

fn simple_calc_fuel_used (from: i32, to:i32) -> i32 {
    (from - to).abs()
}

fn calc_fuel_used(from: i32, to: i32) -> i32 {
    incremental_cost((from - to).abs())
}

fn incremental_cost(x: i32) -> i32 {
    match x {
        0 => 0,
        x => x + incremental_cost(x - 1),
    }
}

#[test]
fn incremental_cost_test() {
    assert_eq!(incremental_cost(4), 10);
    assert_eq!(incremental_cost(10), 10 + 9 + 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1);
}

//#[test]
fn day7_1_test() {
    assert_eq!(day7_1_result("test_input"), 3);
    // guessed 462, too low :-(
    //
    // 0 = 49. 1 = 41, 2 = 37, 3 = 39, 4 = 43
}

#[test]
fn read_file_test() {
    let results = read_file("test_input");
    assert_eq!(results[0], 16);
    assert_eq!(results[5], 2);
}
