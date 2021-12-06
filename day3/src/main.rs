use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};
use std::time::Instant;
use std::iter::Map;

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day3_1_result("./input", 0b111111111111));
    let duration = Instant::now() - start;
    println!("how quick, this quick: {} μs", duration.as_micros());
    let start = Instant::now();
    println!("The second answer is {}", day3_2_result("./input", 0b100000000000));
    let duration = Instant::now() - start;
    println!("how quick, this quick: {} μs", duration.as_micros());
}

fn read_file(path: &str) -> Map<Lines<BufReader<File>>, fn(std::io::Result<String>) -> String> {
    io::BufReader::new(File::open(path).unwrap()).lines().map(|l| l.unwrap())
}

fn day3_1_result(path: &str, mask: u32) -> u32 {
    let (gamma, inverted_gamma) = filth_for_the_first_star(path);
    let epsilon = inverted_gamma & mask;
    println!("gamma {}, epsilon {}", gamma, epsilon);
    return gamma * epsilon;
}

fn day3_2_result(path: &str, mask: u32) -> u32 {
    oxygen_calculation(path,  mask)
}

fn filth_for_the_first_star(path: &str) -> (u32, u32) {
    let (a, b, c, d, e, f, g, h, i, j, k, l, count) = read_file(path)
        .map(|x| u32::from_str_radix(x.as_str(), 2))
        .map(|x|
            match x {
                Ok(y) => (y >> 11, (y & 0b10000000000) >> 10, (y & 0b01000000000) >> 9,
                          (y & 0b00100000000) >> 8, (y & 0b00010000000) >> 7, (y & 0b00001000000) >> 6,
                          (y & 0b00000100000) >> 5, (y & 0b00000010000) >> 4, (y & 0b00000001000) >> 3,
                          (y & 0b00000000100) >> 2, (y & 0b00000000010) >> 1, y & 0b00000000001, 1),
                Err(_) => panic!("meh")
            })
        .fold((0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
              |(a, b, c, d, e, f, g, h, i, j, k, l, count),
               (ax, bx, cx, dx, ex, fx, gx, hx, ix, jx, kx, lx, _)|
                  (a + ax, b + bx, c + cx, d + dx, e + ex, f + fx, g + gx, h + hx, i + ix, j + jx, k + kx, l + lx, count + 1));
    let half = count / 2;
    let gamma: u32 = u32::from(a > half) << 11 | u32::from(b > half) << 10 |
        u32::from(c > half) << 9 | u32::from(d > half) << 8 | u32::from(e > half) << 7 |
        u32::from(f > half) << 6 | u32::from(g > half) << 5 | u32::from(h > half) << 4 |
        u32::from(i > half) << 3 | u32::from(j > half) << 2 | u32::from(k > half) << 1 |
        u32::from(l > half);
    println!("gamma {:b}", gamma);
    (gamma, !gamma)
}

fn custom_log2(input: u32) -> u32 {
    for i in (0..=31).rev() {
        if input & 2u32.pow(i) == 2u32.pow(i) {
            return i + 1;
        }
    }
    0 // I guess you are getting this
}

fn one_if_bit_set(mask: u32, x: u32) -> u32 {
    (x & mask) >> custom_log2(mask) - 1
}

fn most_common_bit_for_masked_location_vec(data: &Vec<u32>, mask: u32) -> u32 {
    let (one_count, half) = parse_ones_in_vec(data, mask);
    u32::from(one_count >= half)
}

fn least_common_bit_for_masked_location_vec(data: &Vec<u32>, mask: u32) -> u32 {
    let (one_count, half) = parse_ones_in_vec(data, mask);
    u32::from(one_count < half) // wrong
}

fn parse_ones_in_vec(data: &Vec<u32>, mask: u32) -> (u32, u32) {
    let (one_count, total_count) = data.into_iter()

        .map(|&x| one_if_bit_set(mask, x))
        .fold((0, 0),
              |(bit_set_count, overall_count), x|
                  (bit_set_count + x, overall_count + 1));

    let half = if total_count & 0b1 == 0b1 { (total_count + 1)/2 } else { total_count/2 };

    (one_count, half)
}

fn oxygen_calculation(path: &str, mask: u32) -> u32 {
    let index = custom_log2(mask);
    let data = input_as_vec_u32(path);
    let mut oxygen_generator = 0;
    let mut co2_scrubber = 0;


    let mut mdata = data.clone();
    for i in (0..index).rev() {
        let mask = 1 << i;
        let most_common_bit = most_common_bit_for_masked_location_vec(&mdata, mask);
        println!("mcb = {}, for {} placed bit", most_common_bit, i);
        mdata = filter_data_matching_bit_mask(&mdata, mask, most_common_bit);
        if mdata.len() == 1 {
            oxygen_generator = mdata[0];
            break;
        }
    }

    let mut ldata = data.clone();
    for i in (0..index).rev() {
        let mask = 1 << i;
        let least_common_bit = least_common_bit_for_masked_location_vec(&ldata, mask);
        println!("lcb = {}, for {} placed bit", least_common_bit, i);

        ldata = filter_data_matching_bit_mask(&ldata, mask, least_common_bit);
        if ldata.len() == 1 {
            co2_scrubber = ldata[0];
            break;
        }
    }

    oxygen_generator * co2_scrubber
}

fn filter_data_matching_bit_mask(data: &Vec<u32>, bit_mask: u32, value_expected: u32) -> Vec<u32> {
    data.into_iter()
        .filter(|&&x| one_if_bit_set(bit_mask, x) == value_expected)
        .map(|&x| x)
        .collect()
}

fn input_as_vec_u32(path: &str) -> Vec<u32> {
    read_file(path)
        .filter_map(|x| u32::from_str_radix(x.as_str(), 2).ok())
        //.filter(|x|*x > 0)
        .map(|x| x)
        .collect()
}

#[test]
fn test_3_1() {
    let power = day3_1_result("./test3_1", 0b11111);
    assert_eq!(power, 198);
    // code produced 845186 - website agreed
}

#[test]
fn test_3_2() {
    let oxygen = oxygen_calculation("./test3_1", 0b11111);
    assert_eq!(oxygen, (230));
    // code produced 4636702 - website agreed
}

#[test]
fn one_if_bit_set_tests() {
    assert_eq!(one_if_bit_set(0b1, 1), 1);
    assert_eq!(one_if_bit_set(0b1, 2), 0);
    assert_eq!(one_if_bit_set(0b10000, 16), 1);
    assert_eq!(one_if_bit_set(0b10000, 31), 1);
    assert_eq!(one_if_bit_set(0b10000, 30), 1);
    assert_eq!(one_if_bit_set(0b10000, 15), 0);
}

#[test]
fn custom_log2_tests() {
    assert_eq!(custom_log2(0b00001), 1);
    assert_eq!(custom_log2(0b01001), 4);
    assert_eq!(custom_log2(0b01101), 4);
    assert_eq!(custom_log2(0b00100), 3);
    assert_eq!(custom_log2(0b10001), 5);
    assert_eq!(custom_log2(0b11001), 5);
    assert_eq!(custom_log2(0b10000), 5);
    assert_eq!(custom_log2(0b11111), 5);
    assert_eq!(custom_log2(0b10000000000000000000000000000000), 32);
}