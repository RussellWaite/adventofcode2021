use std::{time::Instant, usize};

fn main() {
    let start = Instant::now();
    println!("TEST {}", day15_1_result("./test_input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

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
    // just wipe out first value, rules say ignore it
    caves[0][0] = 0;
    let gx = caves.len();
    let gy = caves[0].len();

    let mut matrix = precalculate_matrix(&caves);

    let (matrix_revisted, mut blast_radius) = bltr_transform(&caves, &mut matrix);

    let yet_another_grid = process_change_requests(&caves, matrix_revisted, (gx, gy), &mut blast_radius);

    return yet_another_grid[gx-1][gy-1];
/*
    loop {
   
        let mut matrix2 = bltr_transform(&caves, &mut matrix);
        let mut matrix3 = trbl_transform(&caves, &mut matrix2);
        let matrix4 = brtl_transform(&caves, &mut matrix3);
   
        if matrix2 == matrix3 && matrix3 == matrix4 {
            return matrix4[caves.len()-1][caves[0].len()-1];
        }
        //println!("repeating the triple process as something changed...{}{}{}", matrix2[99][99], matrix3[99][99], matrix4[99][99]);
   }
    panic!("I'm too tired for this shit");
*/
    /*      
        let mut visited = vec![vec![false; caves[0].len()]; caves.len()];
        let position = (0, 0);
        visited[0][0] = true;
        let end = (caves.len() - 1, caves[0].len() - 1);
        
        // need coord, running danger total, 4 options
        // (this asumed no down 3 across 3 up 5 across 4 down 9 style shenanigans)
        let start_danger = caves[0][0] as u32;
        
    let score_to_beat = direct_path(&caves, start_danger, position, end);

    println!(
        "position: {:?}, end:{:?}, (are they equal? {:?}) danger:{}, score to beat: {}",
        position,
        end,
        position == end,
        start_danger,
        score_to_beat
    );
    //   let (temp, d) = survey_danger(&caves, &mut visited, position, end);
    let (paths, lowest_score) = survey_danger(&caves, visited, position, end, score_to_beat);

    //println!("Paths = {:?}", paths);
    lowest_score
*/
}

fn precalculate_matrix(caves: &Vec<Vec<u8>>) -> Vec<Vec<u64>> {
    let grid_x = caves.len();
    let grid_y = caves[0].len();
    let mut optimus:Vec<Vec<u64>> = vec![vec![u64::MAX; grid_y]; grid_x];
    //println!("Caves = {:?}", caves);
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
    //println!("OUTPUT: {:?}", optimus);
    optimus
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
            //println!("Changed row {}, column {}, from {} to {}", jx, jy, risk_matrix[jx][jy], lowest_calculated_score);
            risk_matrix[jx][jy] = lowest_calculated_score;

            let mut potential_recalculations = neighbours.iter().filter(|(_, value)| value > lowest_neighbour).map(|(coords, _)| *coords).collect::<Vec<(usize, usize)>>();
            work.append(&mut &mut potential_recalculations);
        }
    }
    risk_matrix
}

fn bltr_transform(caves: &Vec<Vec<u8>>, optimus: &mut Vec<Vec<u64>>) -> (Vec<Vec<u64>>, Vec<(usize, usize)>) {
    let grid_x = caves.len();
    let grid_y = caves[0].len();
    let mut optimus_clone = optimus.clone();
    let mut work: Vec<(usize, usize)> = vec![];

    caves.iter().enumerate().rev().for_each(|(r, column)| {
        column.iter().enumerate().for_each(|(c, cell)| {
    // could do change here and save one iteration's processing power, maybe a nanosecond or two
                    
            if r == 0 && c == 0 {
                optimus_clone[r][c] = 0;
                return;
            }
            let mut neighbours = surrounding_values(&optimus_clone, (r,c), (grid_x, grid_y));
            let lowest_score_for_cell = neighbours.iter().map(|(_, value)|value).min().unwrap() + *cell as u64;
            if optimus_clone[r][c] != lowest_score_for_cell {
                work.push((r,c));
                
                //println!("Changed row {}, column {}, from {} to {}", r, c, optimus_clone[r][c], temp);
                //optimus_clone[r][c] = temp;
            }
            //let mut jobs = neighbours.iter().map(|(coords, _)| *coords).collect::<Vec<(usize, usize)>>();
            //work.append(&mut jobs);
        });
    });
    (optimus_clone, work)
}


fn surrounding_values(grid: &Vec<Vec<u64>>, origin: (usize, usize), grid_size: (usize, usize)) -> Vec<((usize, usize), u64)> {
    let (gx, gy) = grid_size;
    let (x, y) = origin;
    let mut result: Vec<((usize, usize), u64)> = vec![];

    if x == 0 && y == 0 {
      return result;
    }
  
    if x > 0 {
        result.push(((x-1, y), grid[x-1][y]));
    }
  
    if y > 0 {
        result.push(((x, y-1), grid[x][y-1]));
    }
  
    if x < gx - 1 { 
        result.push(((x+1, y), grid[x+1][y]));
    }
  
    if y < gy-1 {
        result.push(((x, y+1), grid[x][y+1]));
    }
    result
}


fn trbl_transform(caves: &Vec<Vec<u8>>, optimus: &mut Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let grid_x = caves.len();
    let grid_y = caves[0].len();
    let mut optimus_clone = optimus.clone();
    let mut surrounding_values: Vec<u64> = vec![0;4];

    caves.iter().enumerate().for_each(|(r, column)| {
        column.iter().enumerate().rev().for_each(|(c, cell)| {
            if r == 0 && c == 0 {
                optimus_clone[r][c] = 0;
                return;
            }
            surrounding_values.clear();

            if r > 0 {
                surrounding_values.push(optimus_clone[r-1][c]);
            }
            
            if c > 0 {
                surrounding_values.push(optimus_clone[r][c-1]);
            }

            if r < grid_x - 1 {
                surrounding_values.push(optimus_clone[r+1][c]);
            }

            if c < grid_y-1 {
                surrounding_values.push(optimus_clone[r][c+1]);
            }

            let temp =  surrounding_values.iter().min().unwrap() + *cell as u64;
            if optimus_clone[r][c] != temp {
                println!("Changed row {}, column {}, to {}", r, c, temp);
                optimus_clone[r][c] = temp;
            }
        });
    });
    //println!("OUTPUT: {:?}", optimus_clone);
    optimus_clone
}
fn brtl_transform(caves: &Vec<Vec<u8>>, optimus: &mut Vec<Vec<u64>>) -> Vec<Vec<u64>>  {
    let grid_x = caves.len();
    let grid_y = caves[0].len();
    let mut optimus_clone = optimus.clone();
    let mut surrounding_values: Vec<u64> = vec![0;4];

    caves.iter().enumerate().rev().for_each(|(r, column)| {
        column.iter().enumerate().rev().for_each(|(c, cell)| {
            if r == 0 && c == 0 {
                optimus_clone[r][c] = 0;
                return;
            }
            surrounding_values.clear();

            if r > 0 {
                surrounding_values.push(optimus_clone[r-1][c]);
            }
            
            if c > 0 {
                surrounding_values.push(optimus_clone[r][c-1]);
            }

            if r < grid_x - 1 {
                surrounding_values.push(optimus_clone[r+1][c]);
            }

            if c < grid_y-1 {
                surrounding_values.push(optimus_clone[r][c+1]);
            }

            let temp =  surrounding_values.iter().min().unwrap() + *cell as u64;
            if optimus_clone[r][c] != temp {
                println!("Changed row {}, column {}, to {}", r, c, temp);
                optimus_clone[r][c] = temp;
            }
        });
    });
    //println!("OUTPUT: {:?}", optimus_clone);
    optimus_clone
}
// how about, thanks to searching for and finding this: https://www.techiedelight.com/find-minimum-cost-reach-last-cell-matrix-first-cell/
// transform the input as a min against top left

// slow/brute force  way below


// move all the way through with one strategy, baseline it.
// for each decision point push a state to a state store so [0,1] visited_map, route taken vec
// pop off storew, pick a route not in route taken, take next step
//  check running total, compare to "best route" plus distance to end (assume each step is one for
//  now as we don't know so if end is 10,10 and we are at 8,8, its 2 across and 2 down at a
//  minimum, so add 4. If higher than best route - ro99ute dead.
//
//
fn direct_path(
    caves: &Vec<Vec<u8>>,
    start_score: u32,
    start: (usize, usize),
    destination: (usize, usize),
) -> u32 {
    let mut running_total = start_score;
    let (mut pos_x, mut pos_y) = start;
    let (dest_x, dest_y) = destination;

    let pox_x_move: i8 = if dest_x as i32 - pos_x as i32 >= 0 {
        1
    } else {
        -1
    };
    let pox_y_move: i8 = if dest_y as i32 - pos_y as i32 >= 0 {
        1
    } else {
        -1
    };

    while pos_x != dest_x && pos_y != dest_y {
        if pos_x != dest_x {
            pos_x = ((pos_x as i8) + pox_x_move) as usize;
            running_total += caves[pos_x][pos_y] as u32;
        }
        if pos_y != dest_y {
            pos_y = ((pos_y as i8) + pox_y_move) as usize;
            running_total += caves[pos_x][pos_y] as u32;
        }
    }
    running_total
}

fn survey_danger(
    caves: &Vec<Vec<u8>>,
    visited_map: Vec<Vec<bool>>,
    current_position: (usize, usize),
    destination: (usize, usize),
    best_score: u32,
) -> (Vec<Journey>, u32) {
    let mut paths_not_yet_taken: Vec<Journey> = vec![];
    let mut current_best_score = best_score;
    let mut arrivals: Vec<Journey> = vec![];

    let start_of_quest = Journey {
        visited: visited_map.clone(),
        position: (current_position.0 as i8, current_position.1 as i8),
        branches_taken: vec![],
        danger_level: 0, // problem says ignore start unless you re-enter 
        steps: vec![(0, 0)],
    };

    let (dest_x, dest_y) = destination;
    // add first cell of grid to "stack"
    paths_not_yet_taken.push(start_of_quest);

    // stack saving loop, we add to our own stack which is a vec of Journey
    while let Some(mut journey) = paths_not_yet_taken.pop() {
        match find_next_path_in_grid(destination, &journey) {
            Some((x, y)) => {
                if (journey.position.0 as i8 + x == dest_x as i8
                    && journey.position.1 as i8 == dest_y as i8) ||
                    (journey.position.0 as i8 == dest_x as i8 && 
                     journey.position.1 as i8 + y == dest_y as i8)
                {
                    // journey complete, tot up and add to winners
                    journey.make_move(caves, (x, y));
                    if journey.danger_level < current_best_score {
                        current_best_score = journey.danger_level;
                    }
                    println!("Found a path to end, cost was {}, current best:{}", journey.danger_level, current_best_score);
                    arrivals.push(journey.clone());
                    continue;
                }

                // only if journey still on track to be better score, do the following
                // take a copy of our journey,
                // take a path from the ones available for our journey,
                // mark it as used on the Journey
                // put that back on stack
                // progress on our copied journey
                //
                // if journey is a higher score already, it will not ba continued or added
                // bacl to "stack" therefore it's over.
                if journey.danger_level + 2*((dest_x - journey.position.0 as usize + 
                                            dest_y - journey.position.1 as usize) as u32)
                    <= current_best_score
                {

                    let mut next = journey.clone();
                    journey.branches_taken.push((x, y));
                    paths_not_yet_taken.push(journey);

                    if next.danger_level >= current_best_score {
                        println!("HOW IS THIS POSSIBLE, danger level is greater than best score in if statements supposedly preventing it");
                    }

                    next.make_move(caves, (x, y));

                    paths_not_yet_taken.push(next);
                }
            }
            None => {
                //println!("Ran out of options for current journey...#{}",
                //    paths_not_yet_taken.len()
                //);
                continue;
            }
        };
    }
    (arrivals, current_best_score)
}

fn find_next_path_in_grid(destination: (usize, usize), journey: &Journey) -> Option<(i8, i8)> {
    let (x, y) = journey.position;

    // first right, then down, then up then left
    // check tranfrom is avail && doesn't go out of bounds && isn't already a visited cell
    if !journey.branches_taken.contains(&(1, 0))
        && (x as usize) < destination.0
        && !journey.visited[(x + 1) as usize][y as usize]
    {
        return Some((1, 0));
    }
    if !journey.branches_taken.contains(&(0, 1))
        && (y as usize) < destination.1
        && !journey.visited[x as usize][(y + 1) as usize]
    {
        return Some((0, 1));
    }
    if !journey.branches_taken.contains(&(-1, 0))
        && x > 0
        && !journey.visited[(x - 1) as usize][y as usize]
    {
        return Some((-1, 0));
    }
    if !journey.branches_taken.contains(&(0, -1))
        && y > 0
        && !journey.visited[x as usize][(y - 1) as usize]
    {
        return Some((0, -1));
    }
    None
}

#[derive(Clone, Debug)]
struct Journey {
    visited: Vec<Vec<bool>>,
    position: (i8, i8), // make it small as we might have lots in store...
    branches_taken: Vec<(i8, i8)>,
    danger_level: u32,
    steps: Vec<(i8, i8)>,
}
impl Journey {
    fn make_move(&mut self, caves: &Vec<Vec<u8>>, coords: (i8, i8)) {
        self.position = (self.position.0 + coords.0, self.position.1 + coords.1);
        self.steps.push(coords);
        self.visited[self.position.0 as usize][self.position.1 as usize] = true;
        self.danger_level += caves[self.position.0 as usize][self.position.1 as usize] as u32;
        self.branches_taken.clear();
        // this should be the opposite of the move that got us here  - i.e. where we have just been
        self.branches_taken.push((coords.0 * -1, coords.1 * -1));
    }
}

fn day15_2_result(path: &str) -> u64 {
    let _caves = read_file(path);
    0
}


#[test]
fn day15_1_result_test() {
    assert_eq!(day15_1_result("test_input"), 40);
}
