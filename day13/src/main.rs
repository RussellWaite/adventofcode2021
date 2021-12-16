use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("The first answer is {}", day13_1_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());

    let start = Instant::now();
    println!("The second answer is {}", day13_2_result("./input"));
    let duration = Instant::now() - start;
    println!("this quick: {} μs", duration.as_micros());
}

fn read_file(path: &str) -> (Vec<String>, Vec<(usize, usize)>) {
    let contents = &std::fs::read_to_string(path).unwrap();
    let mut instructions: Vec<String> = vec![];

    let result: Vec<(usize, usize)> = contents.lines().fold(vec![], |mut coords, line| {
        if let Ok((x, y)) = line
            .split_once(',')
            .map(|(x, y)| {
                (
                    x.parse::<usize>().unwrap(),
                    y.parse::<usize>().unwrap(),
                )
            })
            .ok_or_else(|| if !line.is_empty() { instructions.push(line.replace("fold along ", "").to_string()); })
        {
            coords.push((x, y));
        }
        coords
    });

    (instructions, result)
}

fn day13_1_result(path: &str) -> usize {
    let (instructions, coords) = read_file(path);

    let (axis, position) = match instructions[0].split_once('=') {
        Some((axis, position)) => (axis, position.parse::<usize>().unwrap()),
        None => panic!("Let's give up as this has not gone well")
    };
    // find grid size
    let (grid_x, grid_y) = coords.iter().fold((0,0), |(max_x, max_y), (x,y)| (max_x.max(*x+1), max_y.max(*y+1)));
    let mut grid: Vec<Vec<bool>> = vec![vec![false; grid_y]; grid_x];
    // plot grid
    coords.iter().for_each(|&(x, y)| grid[x][y] = true);
    println!("Staring grid, x:{}, y:{}",  grid_x, grid_y);
    let (new_x, new_y) = fold_the_paper(&mut grid, axis, grid_x, grid_y, position);
    read_number_of_dots(&grid, new_x, new_y)
}
    
fn fold_the_paper(grid: &mut Vec<Vec<bool>>, axis: &str, grid_x:usize, grid_y: usize, fold_at: usize) -> (usize, usize) {
    if axis == "y" {
        println!("HORIZONTAL FOLD: grid x:{} grid y:{} fold at:{}", grid_x, grid_y, fold_at);
        fold_horizontally(fold_at, grid_x, grid_y, grid)
    }
    else {
       println!("VERTICAL FOLD: grid x:{} grid y:{} fold at:{}", grid_x, grid_y, fold_at);
       fold_vertically(fold_at, grid_x, grid_y, grid)
    }
}

fn read_number_of_dots(grid: &Vec<Vec<bool>>, max_x: usize, max_y: usize) -> usize {
        grid.iter()
            .enumerate()
            .filter(|(idx, _)| idx < &max_x)
            .fold(0, |count, (_, cols)| count + 
                  cols.iter()
                    .enumerate()
                    .filter(|(idx, _)| idx < &max_y)
                    .fold(0, |count, (_, &cell)| count + usize::from(cell)) 
                  )
}



fn fold_horizontally(y_pos: usize, x_max: usize, y_max: usize, grid: &mut Vec<Vec<bool>>) -> (usize, usize) {
    let new_y_max = y_max-(y_pos+1);
    //println!("Horizontal fold  requested : {}, with grid columns of {} after fold will be {}, rows {}",y_pos, y_max, new_y_max, x_max);
    (0..=new_y_max).for_each(|col| {
        (0..x_max).for_each(|row| {
            grid[row][y_pos - col] |= grid[row][y_pos+col];       
            if row == 0 {
                print!("{}", if grid[row][y_pos - col] {"#"} else {"."});
            }
        });
    });
    println!();
    (x_max, new_y_max)
}

fn fold_vertically(x_pos: usize, x_max: usize, y_max: usize, grid: &mut Vec<Vec<bool>>) -> (usize, usize) {
    let new_x_max = x_max-(x_pos+1);
    (0..y_max).for_each(|col| {
        (0..=new_x_max).for_each(|row| {
            grid[x_pos - row][col] |= grid[x_pos +row][col];
            if x_pos - row == 0 {
                print!("{}", if grid[x_pos - row][col] {"#"} else {"."});
            }
        });
    });
    println!();
    (new_x_max, y_max)
}

fn day13_2_result(path: &str) -> usize {
    let (instructions, coords) = read_file(path);

    // find grid size
    let (bounds_x, bounds_y) = coords.iter().fold((0,0), |(max_x, max_y), (x,y)| (max_x.max(*x+1), max_y.max(*y+1)));
    let mut grid: Vec<Vec<bool>> = vec![vec![false; bounds_y]; bounds_x];
    // plot grid
    coords.iter().for_each(|&(x, y)| grid[x][y] = true);
    

    let mut grid_x = bounds_x;
    let mut grid_y = bounds_y;
    // do the folds, mutating grid... mutating = :-( saving memory = :-)
    instructions.iter().for_each(|fold_instruction|{
        println!("Instruction {:?}", fold_instruction);
        let (axis, position) = match fold_instruction.split_once('=') {
            Some((axis, position)) => (axis, position.parse::<usize>().unwrap()),
            None => panic!("Let's give up as this has not gone well")
        };

        let (temp_x, temp_y) = fold_the_paper(&mut grid, axis, grid_x, grid_y, position);
        println!("bounds about to be set to... x={}, y={}", temp_x, temp_y);
        grid_x = temp_x;
        grid_y = temp_y;
    });

    println!("FINAL GRID");
    for c in 0..=grid_y {
        for r in 0..=grid_x {
            print!("{} ", if grid[r][c] { "#" } else { "." });
        }
        println!();
    }
    println!("x = {}, y = {}", grid_x, grid_y);

    read_number_of_dots(&grid, grid_x, grid_y)
}

#[test]
fn read_file_test() {
    let (instructions, coords) = read_file("test_input");
println!("Instructions: {:?} \n\ncoords: {:?}", instructions, coords);
    assert_eq!(instructions.len(), 2);
    assert_eq!(coords[0], (6,10));
}

#[test]
fn day13_1_result_1_test() {
    assert_eq!(day13_1_result("test_input"), 17);
    // refactor testing 
    assert_eq!(day13_1_result("input"), 759); // this was right answer according to site.
}

#[test]
fn day13_2_result_1_test() {
    assert_eq!(day13_2_result("test_input"), 16);
}
