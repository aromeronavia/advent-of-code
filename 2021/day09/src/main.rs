use std::fs;

fn get_input() -> Vec<Vec<i32>> {
    let input = fs::read_to_string("src/input").expect("File does not exist");
    let mut vector: Vec<Vec<i32>> = Vec::new();

    for line in input.split("\n") {
        vector.push(
            line.chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect(),
        );
    }

    vector.pop();
    vector
}

fn first_part() {
    let input = get_input();
    let mut low_points: Vec<i32> = Vec::new();

    for row in 0..input.len() {
        for column in 0..input[row].len() {
            let above = {
                if row as i32 - 1 < 0 {
                    10
                } else {
                    input[row - 1][column]
                }
            };
            let right = {
                if column + 1 >= input[row].len() {
                    10
                } else {
                    input[row][column + 1]
                }
            };
            let below = {
                if row + 1 >= input.len() {
                    10
                } else {
                    input[row + 1][column]
                }
            };
            let left = {
                if column as i32 - 1 < 0 {
                    10
                } else {
                    input[row][column - 1]
                }
            };

            let adjacent_numbers = [above, right, below, left];
            let number = input[row][column];

            if adjacent_numbers.iter().all(|adjacent| *adjacent > number) {
                low_points.push(number);
            }
        }
    }

    println!(
        "{:?}",
        low_points.iter().sum::<i32>() + low_points.len() as i32
    );
}

struct Coordinate {
    row: i32,
    column: i32,
}

fn find_size(input: &Vec<Vec<i32>>, visited: &mut Vec<(i32, i32)>, row: i32, column: i32, mut size: i32) -> i32 {
    if row < 0 || row >= input.len() as i32 || column < 0 || column >= input[0].len() as i32 {
        return size;
    }
    if visited.iter().any(|(visited_row, visited_column)| row == *visited_row && column == *visited_column) {
        return size;
    }

    visited.push((row, column));

    let above = {
        if row as i32 - 1 < 0 {
            10
        } else {
            input[row as usize - 1][column as usize]
        }
    };
    let right = {
        if column + 1 >= input[row as usize].len() as i32 {
            10
        } else {
            input[row as usize][column as usize + 1]
        }
    };
    let below = {
        if row + 1 >= input.len() as i32 {
            10
        } else {
            input[row as usize + 1][column as usize]
        }
    };
    let left = {
        if column as i32 - 1 < 0 {
            10
        } else {
            input[row as usize][column as usize - 1]
        }
    };

    println!("Above {}", above);
    println!("right {}", right);
    println!("below {}", below);
    println!("left {}", left);
    println!("Size: {}", size);

    if left < 9 {
        size += find_size(input, visited, row, column - 1, 0);
    }
    if below < 9 {
        size += find_size(input, visited, row + 1, column, 0);
    }
    if right < 9 {
        size += find_size(input, visited, row, column + 1, 0);
    }
    if above < 9 {
        size += find_size(input, visited, row - 1, column, 0);
    }

    size + 1
}

fn second_part() {
    let input = get_input();
    let mut low_points: Vec<Coordinate> = Vec::new();

    for row in 0..input.len() {
        for column in 0..input[row].len() {
            let above = {
                if row as i32 - 1 < 0 {
                    10
                } else {
                    input[row - 1][column]
                }
            };
            let right = {
                if column + 1 >= input[row].len() {
                    10
                } else {
                    input[row][column + 1]
                }
            };
            let below = {
                if row + 1 >= input.len() {
                    10
                } else {
                    input[row + 1][column]
                }
            };
            let left = {
                if column as i32 - 1 < 0 {
                    10
                } else {
                    input[row][column - 1]
                }
            };

            let adjacent_numbers = [above, right, below, left];
            let number = input[row][column];

            if adjacent_numbers.iter().all(|adjacent| *adjacent > number) {
                low_points.push(Coordinate {
                    row: row as i32,
                    column: column as i32,
                });
            }
        }
    }

    let mut sizes: Vec<i32> = Vec::new();

    for low_point in low_points {
        sizes.push(find_size(&input, &mut Vec::new(), low_point.row, low_point.column, 0));
    }

    sizes.sort_by(|a, b| b.cmp(a));

    println!("Sizes: {:?}", sizes);

    println!("{}", sizes[0] * sizes[1] * sizes[2]);
}

fn main() {
    // first_part();
    second_part();
}
