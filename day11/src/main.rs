use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day11_1_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!("The second answer is {}", day11_2_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!(
        "The second answer from structs method is {}",
        day11_2_2_result("./input")
    );
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());
}

fn read_file(path: &str) -> Vec<Vec<u8>> {
    let lines = io::BufReader::new(File::open(path).unwrap()).lines();
    lines
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect()
}

fn day11_1_result(path: &str) -> u64 {
    let mut octy = read_file(path);
    let mut flashing: u64 = 0;
    let mut flash_map: Vec<Vec<u8>> = vec![vec![0; 10]; 10];

    //for row in 0..octy.len() {
    //    for col in 0..octy[row].len() {
    //        print!("{}", octy[row][col]);
    //    }
    //    println!();
    //}
    //println!("iteration none, flashing none");
    for i in 0..100 {
        // increment
        for row in 0..octy.len() {
            for col in 0..octy[row].len() {
                octy[row][col] += 1;
            }
        }

        let mut still_flashing = true;
        //flash
        while still_flashing {
            still_flashing = false;

            for row in 0..octy.len() {
                for col in 0..octy[row].len() {
                    if octy[row][col] > 9 && flash_map[row][col] == 0 {
                        flash_map[row][col] = 1;
                        still_flashing = true;
                        // above
                        if row != 0 {
                            octy[row - 1][col] += 1;
                            if col != 0 {
                                octy[row - 1][col - 1] += 1;
                            }
                            if col != octy[row].len() - 1 {
                                octy[row - 1][col + 1] += 1;
                            }
                        }

                        // sides
                        if col != 0 {
                            octy[row][col - 1] += 1;
                        }
                        if col != octy[row].len() - 1 {
                            octy[row][col + 1] += 1;
                        }

                        // below
                        if row != octy.len() - 1 {
                            octy[row + 1][col] += 1;
                            if col != 0 {
                                octy[row + 1][col - 1] += 1;
                            }
                            if col != octy[row].len() - 1 {
                                octy[row + 1][col + 1] += 1;
                            }
                        }
                    }
                }
            } // 2 iter = 35, above says 24 - WRONG....
        }
        // debug
        // for row in 0..octy.len() {
        //     for col in 0..octy[row].len() {
        //         let x = octy[row][col];
        //         if x > 9 {
        //             print!("x");
        //         }
        //         else {
        //             print!("{}",x);
        //         }
        //     }
        //    println!();
        // }

        // reset flashers
        flashing += octy
            .iter()
            .map(|row| row.iter().filter(|&&o| o > 9).map(|x| *x as u64).count())
            .sum::<usize>() as u64;

        //   println!("iteration {}, flashing {}", i, flashing);

        octy.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|o| {
                if *o > 9 {
                    *o = 0
                }
            })
        });
        flash_map = vec![vec![0; 10]; 10];
    }
    flashing
}

fn day11_2_result(path: &str) -> u64 {
    let mut octy = read_file(path);
    let mut flashing: u64 = 0;
    let mut flash_map: Vec<Vec<u8>> = vec![vec![0; 10]; 10];

    //for row in 0..octy.len() {
    //    for col in 0..octy[row].len() {
    //        print!("{}", octy[row][col]);
    //    }
    //    println!();
    // }
    let mut iteration = 0;
    //println!("iteration none, flashing none");
    loop {
        iteration += 1;

        // increment
        for row in 0..octy.len() {
            for col in 0..octy[row].len() {
                octy[row][col] += 1;
            }
        }

        let mut still_flashing = true;
        //flash
        while still_flashing {
            still_flashing = false;

            for row in 0..octy.len() {
                for col in 0..octy[row].len() {
                    if octy[row][col] > 9 && flash_map[row][col] == 0 {
                        flash_map[row][col] = 1;
                        still_flashing = true;
                        // above
                        if row != 0 {
                            octy[row - 1][col] += 1;
                            if col != 0 {
                                octy[row - 1][col - 1] += 1;
                            }
                            if col != octy[row].len() - 1 {
                                octy[row - 1][col + 1] += 1;
                            }
                        }

                        // sides
                        if col != 0 {
                            octy[row][col - 1] += 1;
                        }
                        if col != octy[row].len() - 1 {
                            octy[row][col + 1] += 1;
                        }

                        // below
                        if row != octy.len() - 1 {
                            octy[row + 1][col] += 1;
                            if col != 0 {
                                octy[row + 1][col - 1] += 1;
                            }
                            if col != octy[row].len() - 1 {
                                octy[row + 1][col + 1] += 1;
                            }
                        }
                    }
                }
            } // 2 iter = 35, above says 24 - WRONG....
        }
        // debug
        //for row in 0..octy.len() {
        //    for col in 0..octy[row].len() {
        //        let x = octy[row][col];
        //        if x > 9 {
        //            print!("x");
        //        }
        //        else {
        //            print!("{}",x);
        //        }
        //    }
        //    println!();
        //}

        // reset flashers
        flashing += octy
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|&&o| o > 9)
                    //.map(|x| *x as u64)
                    .count()
            })
            .sum::<usize>() as u64;

        //println!("iteration {}, flashing {}", iteration, flashing);

        octy.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|o| {
                if *o > 9 {
                    *o = 0
                }
            })
        });
        flash_map = vec![vec![0; 10]; 10];

        let a = octy[0][0];
        if octy.iter().all(|row| row.iter().all(|col| col == &a)) {
            break;
        }
    }
    iteration
}

fn day11_2_2_result(path: &str) -> u64 {
    let lines = io::BufReader::new(File::open(path).unwrap()).lines();
    let mut octy: Vec<Vec<Octopus>> = lines
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|c| Octopus::new(c.to_digit(10).unwrap() as u8))
                .collect::<Vec<Octopus>>()
        })
        .collect();

    // need to setup neighrours
    //
    //
    let _temp = octy.iter_mut().enumerate().for_each(|(row, x)| {
        x.iter_mut().enumerate().for_each(|(col, octopus)| {
            if octopus.energy > 9 {
                //&& flash_map[row][col] == 0 {
                //flash_map[row][col] = 1;
                //still_flashing = true;
                // above
                if row != 0 {
                    octopus.neighbours.push(Box::new(octy[row - 1][col]));
                    if col != 0 {
                        octopus.neighbours.push(Box::new(octy[row - 1][col - 1]));
                    }
                    if col != octy[row].len() - 1 {
                        octopus.neighbours.push(Box::new(octy[row - 1][col + 1]));
                    }
                }

                // sides
                if col != 0 {
                    octopus.neighbours.push(Box::new(octy[row][col - 1]));
                }
                if col != octy[row].len() - 1 {
                    octopus.neighbours.push(Box::new(octy[row][col + 1]));
                }

                // below
                if row != octy.len() - 1 {
                    octopus.neighbours.push(Box::new(octy[row + 1][col]));
                    if col != 0 {
                        octopus.neighbours.push(Box::new(octy[row + 1][col - 1]));
                    }
                    if col != octy[row].len() - 1 {
                        octopus.neighbours.push(Box::new(octy[row + 1][col + 1]));
                    }
                }
            } // 2 iter = 35, above says 24 - WRONG....
        })
    });
    0
}

struct Octopus {
    neighbours: Vec<Box<Octopus>>,
    energy: u8,
    flash_count: u32,
    flashing: bool,
}

impl Octopus {
    fn new(starting_energy: u8) -> Self {
        Self {
            neighbours: vec![],
            energy: starting_energy,
            flash_count: 0,
            flashing: false, // can get away without having to check as data won't allow for starting flash.
        }
    }

    fn increment(&mut self) {
        self.energy += 1;
        if self.energy > 9 && !self.flashing {
            self.flash();
        }
    }

    fn flash(&mut self) {
        if !self.flashing {
            self.flashing = true;
            self.neighbours.iter_mut().for_each(|o| o.increment());
        }
    }

    fn reset(&mut self) {
        self.energy = 0;
        self.flashing = false;
    }
}

#[test]
fn day11_1_result_test() {
    assert_eq!(day11_1_result("test_input"), 1656);
}

#[test]
fn day11_2_result_test() {
    assert_eq!(day11_2_result("test_input"), 195);
}
