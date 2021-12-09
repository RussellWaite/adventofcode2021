use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};
use std::usize;
use std::borrow::Borrow;

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day9_1_result("./input"));
    let duration = Instant::now() - start;
    println!("how quick, this quick: {} μs", duration.as_micros());
    let start = Instant::now();
    println!("The second (PERF) answer is {}", day9_2_result("./input"));
    let duration = Instant::now() - start;
    println!("how quick, this quick: {} μs", duration.as_micros());
}

fn read_file(path: &str) -> (usize, Vec<u32>) { 
    let lines = io::BufReader::new(File::open(path).unwrap())
        .lines();
    let mut columns = 0;
    let data:Vec<u32> = 
        lines.flat_map(|x|{ 
            let data = x.unwrap().chars().collect::<Vec<char>>();
            columns = data.len();
            data
        })
        .map(|x:char| x.to_digit(10).unwrap())
        .collect();

    (columns, data)
}

fn day9_1_result(path: &str) -> u32 {
    let (columns, data) = read_file(path);
    
    let rows = data.len()/columns;
    let mut results = vec![];

    for cell in 0..data.len() {
        if is_low_point(cell, &data, rows, columns) {
            results.push(data[cell]);
        }
    }
    results.iter().fold(0,|acc, x| acc+x+1)
}

fn is_low_point(point: usize, data: &[u32], rows: usize, columns: usize) -> bool {
    let grid_x = point % columns;
    
    if grid_x > 0 { // not at left edge, check left
        if data[point] >= data[point - 1] { return false;}
    }
    if grid_x < columns-1 { // not at right edge, check right
        if data[point] >= data[point + 1] { return false; }
    }
    
    let grid_y = point / columns;

    if grid_y > 0 { // not at top, check check up
        if data[point] >= data[point - columns] { return false; }
    }
    if grid_y < rows-1 {// not at bottom, check below
        if data[point] >= data[point + columns] { return false; }
    }
    true // if we got here, it's the lowest of it's surroundings
}

fn day9_2_result(path: &str) -> usize {
    let (columns, data) = read_file(path);
    
    let rows = data.len()/columns;
    let mut results = vec![];

    for cell in 0..data.len() {
        if is_low_point(cell, &data, rows, columns) {
            results.push(data[cell]);
        }
    }

    let mut taken: Vec<usize> = vec![];
    let mut basins: Vec<Vec<usize>> = vec![];
    // let "dirty mutations reaching out of functions thanks to closure binding"  commence
    data.iter().enumerate().filter(|(_, &x)| x != 9)
    .for_each(|(idx, _)| {

        let mut this_basin: Vec<usize> = vec![];
         
        expand(idx, &data, rows, columns, &mut taken, &mut this_basin);
        if !this_basin.is_empty() {
            basins.push(this_basin);
        }
    });

    let mut basin_sizes :Vec<usize> = basins.iter().map(|x|  x.len()).collect();
    
    basin_sizes.sort_unstable();

    basin_sizes.reverse();
    basin_sizes[0..3].iter().product()
}

fn expand(idx: usize, area: &[u32], rows: usize, columns:usize, taken: &mut Vec<usize>, basin: &mut Vec<usize>) {
    if taken.contains(&idx) {
        return;
    }
    taken.push(idx);
    basin.push(idx);

    if let Some(x) = move_right(idx, area, columns, taken) { expand(x, area, rows, columns, taken, basin) }
    if let Some(x) = move_down(idx, area, rows, columns, taken) { expand(x, area, rows, columns, taken, basin) }
    if let Some(x) = move_left(idx, area, columns, taken) { expand(x, area, rows, columns, taken, basin) }
    if let Some(x) = move_up(idx, area, columns, taken) { expand(x, area, rows, columns, taken, basin) }
}

fn move_left(idx: usize, area: &[u32], columns: usize, taken: &Vec<usize>) -> Option<usize> {
    let grid_x = idx % columns;
    if grid_x > 0 && area[idx -1] != 9 && !taken.contains(&(idx - 1)) { // not at left edge, check left
        return Some(idx - 1);
    }
    None
}

fn move_down(idx:usize, area: &[u32], rows: usize, columns: usize, taken: &Vec<usize>) -> Option<usize> {
    let grid_y = idx / columns;
    if grid_y < rows-1 && area[idx + columns] != 9 && !taken.contains(&(idx + columns)) {// not at bottom, check below
        return Some(idx + columns);
    }
    None
}

fn move_right(idx: usize, area: &[u32], columns: usize, taken: &Vec<usize>) -> Option<usize> {
   let grid_x = idx % columns;
    if grid_x < columns-1 && area[idx + 1] != 9 && !taken.contains((idx + 1).borrow()){ // not at right edge, check right
        return Some(idx + 1);
    }
   None 
}

fn move_up(idx: usize, area: &[u32], columns: usize, taken: &Vec<usize>) -> Option<usize> {
    let grid_y = idx / columns;
    if grid_y > 0 && area[idx - columns] != 9 && !taken.contains(&(idx - columns)) { // not at top, check check up
        return Some(idx - columns); 
    }
    None
}

#[test]
fn read_file_test() {
    let (columns, data) = read_file("test_input");
    assert_eq!(data.len(), 50);
    assert_eq!(columns, 10);
}

#[test]
fn day9_1_result_test() {
    //100*100
    assert_eq!(day9_1_result("test_input"), 15)
}

#[test]
fn day9_2_result_test() {
    assert_eq!(day9_2_result("test_input"), 1134);
}
