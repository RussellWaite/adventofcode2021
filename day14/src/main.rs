use std::{collections::HashMap, time::Instant, char};

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day14_1_result("./input", 10));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!("The second answer is {}", here_goes_nothing("./input", 40));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());
}

fn read_file(path: &str) -> (String, HashMap<String, String>, HashMap<String, char>) {
    let contents = &std::fs::read_to_string(path).unwrap();
    let mut polymer_template: String = "".to_string();

    let (pair_insertions, pair_extra_char):(HashMap<String, String>, HashMap<String, char>) =
        contents.lines().fold((HashMap::new(), HashMap::new()), |(mut result, mut extra_char), line| {

            if let Ok((pair, insertion, extra)) = line
                .split_once(" -> ")
                .map(|(pair, insertion)| {
                    // oh lord this is UGLY, this can't be how rust deals with strings and
                    // characters, I must have messed this up...
                    
                    let mut temp = String::new();
                    let mut chars = pair.chars();

                    temp.push(chars.next().unwrap());
                    temp += insertion;
                    temp.push(chars.next().unwrap());

                    (pair.parse::<String>().unwrap(), temp , insertion.chars().next().unwrap())
                })
                .ok_or_else(|| {
                    if !line.is_empty() {
                        polymer_template = line.to_string();
                    }
                })
            {
                result.insert(pair.to_owned(), insertion);
                extra_char.insert(pair, extra);
            }
            (result, extra_char)
        });

    (polymer_template, pair_insertions, pair_extra_char)
}

fn day14_1_result(path: &str, iteration: u32) -> u64 {
    let (template, pair_insertions, _) = read_file(path);
    let mut template_of_doom = template;

    (0..iteration).for_each(|_x| {
        let (charlistio, _) =
            template_of_doom
                .chars()
                .fold((String::new(), ' '), |(mut polymer, last_element), element| {
                    if last_element != ' ' {
                        let index = String::from_iter(vec![last_element, element]);
                        polymer.pop(); 
                        polymer += &pair_insertions[&index];
                    } else {
                        polymer.push(element);
                    }
                    (polymer, element)
                });
        template_of_doom = charlistio;
    });

    let grouped = template_of_doom.chars().fold(HashMap::new(), |mut map, c| {
        let counter = map.entry(c).or_insert(0);
        *counter += 1;
        map
    });

    let least_val = grouped.values().min().unwrap();
    let most_val = grouped.values().max().unwrap();

    println!("Hashmap for grouped = {:?}", grouped);
    most_val - least_val
}

fn here_goes_nothing(path: &str, goes_around: u8) -> u64 {
    let (template, _, pair_to_char) = read_file(path);
    let mut inserts: HashMap<char, u64> = HashMap::new();

    let next_pairs : HashMap<String, (String, String)> = build_next_pairs(&pair_to_char);
    let mut workload: HashMap<String, u64> = gimme_pairs_from_input(&template)
        .iter()
        .fold(HashMap::new(), |mut map, pair| {
            let count = map.entry(pair.to_string()).or_insert(0);
            *count +=1;
            map
        });

    // seed start into inserts
    template.chars().for_each(|c|{
        let count = inserts.entry(c).or_insert(0);
        *count += 1;
    });

    for i in 0..goes_around {
        let mut next_workload: HashMap<String, u64> = HashMap::new();
        workload.iter().for_each(|(pair, repetitions)|{
            let count = inserts.entry(pair_to_char[pair]).or_insert(0);
            *count += repetitions;

            // now double the work...
            let (first_pair, second_pair) = next_pairs.get(pair).unwrap();
            let count = next_workload.entry(first_pair.to_string()).or_insert(0);
            *count += repetitions;
            let count = next_workload.entry(second_pair.to_string()).or_insert(0);
            *count += repetitions;
            // can probably DRY this up.
        });
        workload = next_workload
    }
    inserts.values().max().unwrap() - inserts.values().min().unwrap()
}

fn gimme_pairs_from_input(input: &str) -> Vec<String> {                                                                                                                         
    let (pairs, _) = input.chars().fold(
        (vec![], ' '),
        |(mut template_pairs, last_char), c| {
            if last_char != ' ' {
                template_pairs.push(vec![last_char, c].iter().collect::<String>());
            }                                                     
            (template_pairs, c)
    });
    pairs
}                                        

fn build_next_pairs(pair_to_char: &HashMap<String, char>) -> HashMap<String, (String, String)> {
    let result: HashMap<String, (String, String)> =
        pair_to_char.iter().fold(HashMap::new(), |mut map, (key, &val)|{
            let mut key_clone = key.clone();
            let right = vec![val,  key_clone.pop().unwrap()].iter().collect::<String>();
            let left = vec![key_clone.pop().unwrap(), val].iter().collect::<String>();
            map.insert(key.to_string(), (left, right));
            map
        });
    result
}


#[test]
fn here_goes_nothing_test() {
    let once = here_goes_nothing("test_input", 1);
    let twice= here_goes_nothing("test_input", 2);
    let three = here_goes_nothing("test_input", 3);
    let four = here_goes_nothing("test_input", 4);
    println!("TEST 5 DONE, onto 10...");
    let _ten = here_goes_nothing("test_input", 10);
    println!("TEST 10 DONE, onto 15...");
    let _fifteen = here_goes_nothing("test_input", 15);
    println!("TEST 15 DONE, onto 20...");
    let _twenty = here_goes_nothing("test_input", 20);

    assert_eq!(once, 1);
    assert_eq!(twice, 5);
    assert_eq!(three, 7);
    assert_eq!(four, 18);
    assert_eq!(_ten, 1588)
}

#[test]
fn read_file_test() {
    let (tempalte, rules, _) = read_file("test_input");
    println!("Template: {}", tempalte);
    println!("Rules: {:?}", rules);
}

#[test]
fn day14_1_result_test() {
    assert_eq!(day14_1_result("test_input", 1), 1); // this was right answer according to site.
    assert_eq!(day14_1_result("test_input", 2), 5); // this was right answer according to site.
    assert_eq!(day14_1_result("test_input", 3), 7); // this was right answer according to site.
    assert_eq!(day14_1_result("test_input", 4), 18); // this was right answer according to site.
}

//#[test]
//fn day14_2_result_test() {
//    //assert_eq!(day14_2_result("test_input",40), 2188189693529);
//    assert_eq!(day14_2_result("test_input", 1), 1); // this was right answer according to site.
//    assert_eq!(day14_2_result("test_input", 2), 5); // this was right answer according to site.
//    assert_eq!(day14_2_result("test_input", 3), 7); // this was right answer according to site.
//    assert_eq!(day14_2_result("test_input", 4), 18); // this was right answer according to site.
//    assert_eq!(day14_2_result("test_input", 40), 182188189693529); // this was right answer according to site.
//}
// #[test]
// fn split_input_for_cache_search_test() {
//     assert_eq!(split_input_for_cache_search("Rust"), ("Rus".to_string(), "st".to_string()));
// }

// fn day14_2_result(path: &str, iteration: u8) -> u64 {
//
//
//     let (mut template, primitives, _) = read_file(path);
//
//     let mut bucket: HashMap<String, (String, u64)> = HashMap::new();
//     primitives.iter().for_each(|(key, value)| {
//         bucket.insert(key.to_string(), (value.to_string(), 0));
//     });
//
//     for i in 0..iteration{
//         println!("------------------- ATTEMPT {}, HashMap contains {} keys", i, bucket.len());
//         template = match template.len() {
//             0 | 1 | 2 => panic!("what are you doing?!?"),
//             // I can't be bothered to fix this, it's irrelevant anyway
//             //2 => { let mut value = &bucket[&template]; *value.1 +=1; value.0},
//             _ => {
//                 plunder_cache(&mut bucket, template, i == iteration-1)
//             }
//         };
//     }
//     let grouped = bucket.iter().fold(HashMap::new(), |mut map, (_k,(v, c))| {
//         v.chars().for_each(|ch|{
//             let counter = map.entry(ch).or_insert(0);
//             *counter += c;
//         });
//         map
//     });
//
//     grouped.values().max().unwrap() - grouped.values().min().unwrap()
// }



//
// fn plunder_cache(bucket: &mut HashMap<String, (String, u64)>, key: String, record_usage: bool) -> String {
//     return match bucket.get_mut(&key) {
//         Some((polymer, times_used)) => {
//             *times_used += u64::from(record_usage);
//             polymer.to_string()
//         },
//         None => {
//             let (left, right) = split_input_for_cache_search(&key);
//             let mut left_match = plunder_cache(bucket, left, false);
//             let right_match = plunder_cache(bucket, right, false);
//             left_match.pop(); // strip repeated value (last of left is first of right)
//             let mut value: String = left_match;
//             value += &right_match;
//             bucket.entry(key).or_insert((value.to_string(), u64::from(record_usage)));
//
//             value
//         }
//     };
// }
//
//
// fn split_input_for_cache_search(input: &str) -> (String, String) {
//     match input.len() {
//         0 | 1 | 2  => panic!("should never be using this for such short keys."),
//         3 => {
//             let left: String = (input[0..2]).to_owned();
//             let right:String = (input[1..=2]).to_owned();
//             (left, right)
//         },
//         _ => {
//             let half_input = input.len()/2;
//             let left: String = (input[0..half_input+1]).to_owned();
//             let right:String = (input[half_input..input.len()]).to_owned();
//             (left, right)
//         }
//     }
// }