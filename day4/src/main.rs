use std::fs::{File};
use std::io::{Read};
use std::str::Split;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day4_1_result("./input"));
    let duration = Instant::now() - start;
    println!("how quick, this quick: {} μs", duration.as_micros());
    let start = Instant::now();
    println!("The second answer is {}", day4_2_result("./input"));
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

fn day4_2_result(path: &str) -> u32 {
    let data = read_file(path);
    let mut lines = data.split("\n");
    let call_order: Vec<u32> = read_in_game_sequence(&mut lines);
    let mut cards: Vec<BingoCard> = read_in_bingo_cards(&mut lines);
    play_bingo_to_lose(call_order, &mut cards)
}

fn day4_1_result(path: &str) -> u32 {
    let data = read_file(path);
    let mut lines = data.split("\n");
    let call_order: Vec<u32> = read_in_game_sequence(&mut lines);
    let mut cards: Vec<BingoCard> = read_in_bingo_cards(&mut lines);
    play_bingo(call_order, &mut cards)
}

fn read_in_game_sequence(lines: &mut Split<&str>) -> Vec<u32> {
    lines.next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap()).collect()
}

fn read_in_bingo_cards(lines: &mut Split<&str>) -> Vec<BingoCard> {
    let mut cards: Vec<BingoCard> = vec![];
    let mut current_card = BingoCard::new();

    loop {
        match lines.next() {
            Some(line) if !line.is_empty() => {
                line.split_whitespace()
                    .map(|x| x.parse::<u32>().unwrap())
                    .for_each(|x|
                        current_card.numbers.push(x));

                if current_card.numbers.len() == 25 {
                    cards.push(current_card);
                    current_card = BingoCard::new();
                }
            }
            Some(_) => continue,
            None => break,
        };
    }
    cards
}

fn play_bingo(call_order: Vec<u32>, cards: &mut Vec<BingoCard>) -> u32 {
    for bingo_ball in call_order {
        for matching_card in cards.iter_mut().filter(|bc| bc.numbers.contains(&bingo_ball)) {
            if matching_card.mark_called_number(bingo_ball) {
                matching_card.print_card();
                return process_score(bingo_ball, matching_card);
            }
        }
        //println!("ball {:?} matched {}", bingo_ball, times_matched);
    }
    panic!("no one wins, squid for tea?")
}

fn process_score(bingo_ball: u32, matching_card: &mut BingoCard) -> u32 {
    let mut running_total = 0;
    for i in 0..25 {
        if matching_card.hits[i] == 0 {
            running_total += matching_card.numbers[i];
        }
    }
    running_total * bingo_ball
}

fn play_bingo_to_lose(call_order: Vec<u32>, cards: &mut Vec<BingoCard>) -> u32 {
    let num_cards = cards.len();
    let mut winning_cards = 0;

    for bingo_ball in call_order {
        for matching_card in cards.iter_mut().filter(|bc| bc.numbers.contains(&bingo_ball)) {
            let already_won = matching_card.winner;
            if matching_card.mark_called_number(bingo_ball) && !already_won {
                winning_cards +=1;
                // println!("WINNER {} of {}----------------------------------------------------------------------------------------------------------------",
                //          winning_cards, num_cards);
                // println!("last ball {}, matching card:", bingo_ball);
                // matching_card.print_card();
                // println!("-------------------------------------------------------------------------------------------------------------------------------");
                if winning_cards == num_cards {
                    matching_card.print_card();
                    return process_score(bingo_ball, matching_card);
                }
            }
        }
        // println!("ball {:?} matched {}", bingo_ball, times_matched);
    }
    panic!("no one wins, squid for tea?")
}

#[test]
fn day4_2_test() {
    assert_eq!(day4_2_result("./input"),17884);
    // and the answer is 17884
}

#[test]
fn day4_1_test()
{
    let score = day4_1_result("./day4");
    assert_eq!(score, 4512); //188 * 24 = 4512.

    let score = day4_1_result("./input");
    assert_eq!(score, 74320); // website agrees this is right
}

#[derive(Debug)]
struct BingoCard {
    numbers: Vec<u32>,
    hits: Vec<u8>,
    winner: bool
}

impl BingoCard {
    pub fn new() -> Self {
        Self {
            numbers: vec![],
            hits: vec![0; 25],
            winner: false
        }
    }
    pub fn is_winner(&self) -> bool {
        match &self.hits {
            x  if x[0] + x[1] + x[2] + x[3] + x[4] == 5 => true,
            x  if x[5] + x[6] + x[7] + x[8] + x[9] == 5 => true,
            x  if x[10] + x[11] + x[12] + x[13] + x[14] == 5 => true,
            x  if x[15] + x[16] + x[17] + x[18] + x[19] == 5 => true,
            x  if x[20] + x[21] + x[22] + x[23] + x[24] == 5 => true,
            x  if x[0] + x[5] + x[10] + x[15] + x[20] == 5 => true,
            x  if x[1] + x[6] + x[11] + x[16] + x[21] == 5 => true,
            x  if x[2] + x[7] + x[12] + x[17] + x[22] == 5 => true,
            x  if x[3] + x[8] + x[13] + x[18] + x[23] == 5 => true,
            x  if x[4] + x[9] + x[14] + x[19] + x[24] == 5 => true,
            _ => false
        }
    }
    pub fn mark_called_number(&mut self, called_number: u32) -> bool {
        for i in 0..25 {
            if called_number == self.numbers[i] {
                self.hits[i] = 1;
            }
        }
        self.winner = self.is_winner();
        self.winner
    }
    pub fn print_card(&mut self) {
        println!("{} {} {} {} {}", self.print_self(0), self.print_self(1), self.print_self(2), self.print_self(3), self.print_self(4));
        println!("{} {} {} {} {}", self.print_self(5), self.print_self(6), self.print_self(7), self.print_self(8), self.print_self(9));
        println!("{} {} {} {} {}", self.print_self(10), self.print_self(11), self.print_self(12), self.print_self(13), self.print_self(14));
        println!("{} {} {} {} {}", self.print_self(15), self.print_self(16), self.print_self(17), self.print_self(18), self.print_self(19));
        println!("{} {} {} {} {}", self.print_self(20), self.print_self(21), self.print_self(22), self.print_self(23), self.print_self(24));
        println!();
    }
    fn print_self(&mut self, i: usize) -> String {
        if self.hits[i] == 1 {
            format!("[{:02}]", self.numbers[i])
            } else {
            format!(" {:02} ", self.numbers[i])
        }
    }
}