use std::{time::Instant, usize};

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day15_1_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());
    
    let start = Instant::now();
    println!("The second answer is {}", day15_2_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());
}

fn read_file(path: &str) -> Vec<Vec<u8>> {
    let contents = &std::fs::read_to_string(path).unwrap();
    contents.lines().fold(vec![], |mut rows, line| {
        rows.push(line.chars().fold(vec![], |mut columns, danger| {
            columns.push(danger.to_string().parse::<u8>().unwrap());
            columns
        }));
        rows
    })
}

fn day15_1_result(path: &str) -> u64 {
    let mut caves = read_file(path);
    caves[0][0] = 0;
    let gx = caves.len();
    let gy = caves[0].len();

    let mut matrix = precalculate_matrix(&caves);
    let (matrix_revisted, mut blast_radius) = bottom_left_up_pass(&caves, &mut matrix);
    let yet_another_grid = process_change_requests(&caves, matrix_revisted, (gx, gy), &mut blast_radius);

    return yet_another_grid[gx-1][gy-1];
}

fn precalculate_matrix(caves: &Vec<Vec<u8>>) -> Vec<Vec<u64>> {
    let grid_x = caves.len();
    let grid_y = caves[0].len();
    let mut optimus:Vec<Vec<u64>> = vec![vec![u64::MAX; grid_y]; grid_x];
    caves.iter().enumerate().for_each(|(r, column)| {
        column.iter().enumerate().for_each(|(c, cell)| {
            if r == 0 && c == 0 {
                optimus[r][c] = 0;
                return;
            }
            if r == 0 {
                optimus[r][c] = optimus[r][c-1] + *cell as u64;//caves[r][c]
                return;
            }

            if c == 0 {
                optimus[r][c] = optimus[r-1][c] + *cell as u64;//caves[r][c];
                return;
            }

            optimus[r][c] = (optimus[r-1][c]).min(optimus[r][c-1]) + *cell as u64;//caves[r][c];
        });
    });
    optimus
}

fn bottom_left_up_pass(caves: &Vec<Vec<u8>>, optimus: &mut Vec<Vec<u64>>) -> (Vec<Vec<u64>>, Vec<(usize, usize)>) {
    let grid_x = caves.len();
    let grid_y = caves[0].len();
    let mut optimus_clone = optimus.clone();
    let mut work: Vec<(usize, usize)> = vec![];

    caves.iter().enumerate().rev().for_each(|(r, column)| {
        column.iter().enumerate().for_each(|(c, cell)| {
            if r == 0 && c == 0 {
                optimus_clone[r][c] = 0;
                return;
            }
            let neighbours = surrounding_values(&optimus_clone, (r,c), (grid_x, grid_y));
            let lowest_score_for_cell = neighbours.iter().map(|(_, value)|value).min().unwrap() + *cell as u64;
            if optimus_clone[r][c] != lowest_score_for_cell {
                work.push((r,c));
            }
        });
    });
    (optimus_clone, work)
}

fn process_change_requests(caves: &Vec<Vec<u8>>, 
                           mut risk_matrix: Vec<Vec<u64>>, 
                           grid_size: (usize, usize),
                           work: &mut Vec<(usize, usize)>) -> Vec<Vec<u64>> {
    
    while let Some(job) = work.pop() {
        let (jx, jy) = job;

        if jx == 0 && jy == 0 {                                                             
            risk_matrix[jx][jy] = 0;                                                                                                                                                                                                                                                                                                                                                  
            continue;
        }

        let neighbours = surrounding_values(&risk_matrix, job, grid_size);
        let lowest_neighbour = neighbours.iter().map(|(_, value)| value).min().unwrap();
        let lowest_calculated_score = lowest_neighbour + caves[jx][jy] as u64;

        if risk_matrix[jx][jy] != lowest_calculated_score {    
            risk_matrix[jx][jy] = lowest_calculated_score;

            let mut potential_recalculations = neighbours.iter().filter(|(_, value)| value > lowest_neighbour).map(|(coords, _)| *coords).collect::<Vec<(usize, usize)>>();
            work.append(&mut &mut potential_recalculations);
        }
    }
    risk_matrix
}

fn surrounding_values(grid: &Vec<Vec<u64>>, origin: (usize, usize), grid_size: (usize, usize)) -> Vec<((usize, usize), u64)> {
    let (gx, gy) = grid_size;
    let (x, y) = origin;
    let mut result: Vec<((usize, usize), u64)> = vec![];

    if x == 0 && y == 0 { return result; }
    
    if x > 0        { result.push(((x-1, y), grid[x-1][y])); }
    if y > 0        { result.push(((x, y-1), grid[x][y-1])); }
    if x < gx - 1   { result.push(((x+1, y), grid[x+1][y])); }
    if y < gy-1     { result.push(((x, y+1), grid[x][y+1])); }

    result
}

fn day15_2_result(path: &str) -> u64 {
    let _caves = read_file(path);
    0
}


#[test]
fn day15_1_result_test() {
    assert_eq!(day15_1_result("test_input"), 40);
}
