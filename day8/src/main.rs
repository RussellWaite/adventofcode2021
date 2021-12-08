use std::{collections::HashMap, time::Instant};

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day8_1_result("./input"));
    let duration = Instant::now() - start;
    println!("how quick, this quick: {} μs", duration.as_micros());
    let start = Instant::now();
    println!("The second (PERF) answer is {}", day8_2_result("./input"));
    let duration = Instant::now() - start;
    println!("how quick, this quick: {} μs", duration.as_micros());
}

fn read_file(path: &str) -> Vec<String> {
    std::fs::read_to_string(path)
        .unwrap()
        .split(&['\n', '|'][..])
        .skip(1)
        .step_by(2)
        .map(|s| s.to_string())
        .collect()
}

fn day8_1_result(path: &str) -> usize {
    let result = read_file(path);
    // the iterator way beat me here, BAD CODE WARNING...
    let mut total: usize = 0;
    for line in result {
        // counts of 2,4,3,7
        total += line
            .split_whitespace()
            .map(|digit| digit.len())
            .filter(|x| [2, 3, 4, 7].contains(x))
            .count();
    }
    total
}

fn read_file_with_diagnostic_data(path: &str) -> (Vec<String>, Vec<String>) {
    let (diagnostics, output, _) = std::fs::read_to_string(path)
        .unwrap()
        .split(&['\n', '|'][..])
        .map(|s| s.to_string())
        .filter(|s| s.len() > 0)
        .fold(
            (vec![], vec![], false),
            |(mut diagnostics, mut output, is_output), x| {
                match is_output {
                    true => output.push(x),
                    false => diagnostics.push(x),
                };
                (diagnostics, output, !is_output)
            },
        );

    (diagnostics, output)
}

fn process_diag_output_pair(diagnostics: &str, output: &str) -> u32{
    /*
     * 1 is only 2 char
     * 7 is 3 char
     * 4 is 4 char
     * 8 is 7 char
     *
     * 2,3,5 are 5 char
     * 6,9,0 are 6 char
     *
     * 6 contians none of 1's chars
     * 0 contains 1 miss on 4's chars, wheras 6 and 9 contain all
     * 9 should be whats left after above
     *
     * 3 contains all of 1's chars
     * 2 has only 1 miss from 4's chars
     * 5 has two misses from 4's chars
     *
     */
    let digit_rubbish: Vec<&str> = diagnostics.split_whitespace().map(|s| s).collect();
    //TODO: is there a better way to create a populated vec of String? Should I even be using String or should I move to &str?
    let mut digits = vec!["".to_string();10];
    let mut six09 = vec![];
    let mut two35 = vec![];

    for i in 0..digit_rubbish.len() {
        let x = sorted_string(&digit_rubbish[i]);
        match digit_rubbish[i].len() {
            2 => digits[1] = x,
            3 => digits[7] = x,
            4 => digits[4] = x,
            7 => digits[8] = x,
            5 => two35.push(x),
            6 => six09.push(x),
            _ => {}
        }
    }
    let d1 = digits[1].clone();
    let d4 = digits[4].clone();
    let split_one: Vec<&str> = d1.split("").collect::<Vec<&str>>().to_owned();
    let split_four: Vec<&str> = d4.split("").collect::<Vec<&str>>().to_owned();

    for i in 0..six09.len() {
        if !split_one.iter().all(|d| six09[i].contains(d)) {
            digits[6] = six09[i].to_owned();
            continue;
        }
        if split_four.iter().all(|d| six09[i].contains(d)) {
            digits[9] = six09[i].to_owned();
        } else {
            digits[0] = six09[i].to_owned();
        }
    }

    for i in 0..two35.len() {
        if split_one.iter().all(|d| two35[i].contains(d)) {
            digits[3] = two35[i].to_owned();
            continue;
        }
        if two35[i].split("").all(|d| digits[6].contains(d)) {
            digits[5] = two35[i].to_owned();
        } else {
            digits[2] = two35[i].to_owned();
        }
    }

    println!("DIGITS IS AS FOLLOWS: {:?}", digits);
    // all digits mapped, let's look up...

    let output_digits: Vec<&str> = output.split_whitespace().collect();
    let mut converted_output: Vec<u32> = vec![];
    for od in output_digits {
        println!("OUTPUT DIGIT = {}", od);
        match od.len() {
            2 => {println!("read {} as a 1",od); converted_output.push(1)},
            3 => {println!("read {} as a 7",od); converted_output.push(7)},
            4 => {println!("read {} as a 4",od); converted_output.push(4)},
            7 => {println!("read {} as a 8",od); converted_output.push(8)},
            5 => {
                let temp = sorted_string(od);
                if temp == digits[2] {
                    println!("read {} as a 2",temp);  
                    converted_output.push(2);
                }
                if temp == digits[3] {
                    println!("read {} as a 3",temp);  
                    converted_output.push(3);
                }
                if temp == digits[5] {
                    println!("read {} as a 5",temp);  
                    converted_output.push(5);
                }
                // TODO: why didn't this work? rust analyser went nuts at me...
                //match sorted_string(od) {
                //    digits[2] => converted_output.push(2),
                //    digits[3] => converted_output.push(3),
                //    digits[5] => converted_output.push(5)
            }
            6 => {
                let temp = sorted_string(od);

                if temp == digits[6] {
                    println!("read {} as a 6",temp);
                    converted_output.push(6);
                }
                if temp == digits[0] {
                    println!("read {} as a 0",temp); 
                    converted_output.push(0);
                }
                if temp == digits[9] {
                    println!("read {} as a 9",temp);
                    converted_output.push(9);
                }
            }
            _ => {}
        }
    }
    
    converted_output[0] * 1000 + converted_output[1] * 100 + converted_output[2] * 10 + converted_output[3]
    // TODO:attempt x-1, how can you make this work? 
    //let (a,b,c,d,six09, two35) =
    //    diagnostics.split_whitespace()
    //    .map(|s|s.to_string())
    //    .fold(
    //        ("","","","", vec![], vec![]),
    //        | (mut one, mut seven, mut four, mut eight, mut six09, mut two35), x |
    //        {
    //            match x {
    //                if x.len() == 2 => one = x,
    //                if x.len() == 3 => seven = x,
    //                if x.len() == 4 => four= x,
    //                if x.len() == 7 => eight= x,
    //                five_chars if x.len() == 5 => six09.push(five_chars.clone()),
    //                six_chars if x.len() == 6 => two35.push(six_chars.clone()),
    //                _ => {}
    //            };
    //            (one, seven, four, eight, six09, two35)
    //        });
}

fn sorted_string(input: &str) -> String {
    let mut i = input.as_bytes().to_owned();
    i.sort();
    let my_s = String::from_utf8(i.to_vec()).unwrap();
    my_s
}
fn day8_2_result(path: &str) -> u32 {
    let (diagnostics, output) = read_file_with_diagnostic_data(path);

    println!("Diagnostics: {:?}", diagnostics);

    println!("Output: {:?}", output);
    
    let mut total = 0;

    for i in 0..diagnostics.len() {
        total += process_diag_output_pair(&diagnostics[i], &output[i]);
    }

    total
}

#[test]
fn day8_1_result_test() {
    let result = day8_1_result("test_input");
    assert_eq!(result, 26);
}

#[test]
fn day8_2_result_test() {
    assert_eq!(day8_2_result("test_input"), 61229);
}
#[test]
fn process_diag_output_pair_test() {
    let diag = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab";
    let output = "cdfeb fcadb cdfeb cdbaf";
    assert_eq!(process_diag_output_pair(diag, output), 5353);
}
#[test]
fn read_file_test() {
    let result = read_file("test_input");
    println!("{:?}", result);
    assert_eq!(result.len(), 10);
    assert_eq!("fdgacbe cefdb cefbgd gcbe", result[0].trim());
}
