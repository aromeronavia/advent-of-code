use std::fs;
use std::collections::HashMap;

fn get_input<'a>() -> Vec<String> {
    let input = fs::read_to_string("src/input").expect("Unable to read file");
    let lines = input
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    lines
}


fn process_appearing_numbers(raw_line: String) -> Vec<i32> {
    let line_splitted = raw_line.trim().split(",");
    println!("{:?}", line_splitted);
    let numbers: Vec<i32> = line_splitted
        .filter(|x| *x != "")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    numbers
}

fn process_bingo_line(raw_line: String) -> [i32;5] {
    let line_splitted = raw_line.split(" ");
    let numbers: Vec<i32> = line_splitted
        .filter(|x| *x != "")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    [numbers[0], numbers[1], numbers[2], numbers[3], numbers[4]]
}

fn mark_bingo(bingo_matrixes: &mut Vec<Vec<[i32;5]>>, number: i32) {
    for matrix_number in 0..bingo_matrixes.len() {
        for matrix_row in 0..bingo_matrixes[matrix_number].len() {
            for matrix_column in 0..5 {
                if bingo_matrixes[matrix_number][matrix_row][matrix_column] == number {
                    bingo_matrixes[matrix_number][matrix_row][matrix_column] = -1;
                }
            }
        }
    }
}

fn find_winner(bingo_matrixes: &Vec<Vec<[i32;5]>>) -> i32 {
    for matrix_number in 0..bingo_matrixes.len() {
        for row in 0..bingo_matrixes[matrix_number].len() {
            let sum_row: i32 = bingo_matrixes[matrix_number][row].iter().sum();
            if sum_row == -5 {
                return matrix_number as i32;
            }
        }
    }

    for matrix_number in 0..bingo_matrixes.len() {
        for column in 0..bingo_matrixes[matrix_number].len() {
            let mut column_vector: Vec<i32> = Vec::new();
            for row in 0..bingo_matrixes[matrix_number][column].len() {
                column_vector.push(bingo_matrixes[matrix_number][row][column]);
            }

            if column_vector.iter().sum::<i32>() == -5 {
                return matrix_number as i32;
            }
        }
    }

    -1
}

fn calculate_answer(winner_board: &mut Vec<[i32;5]>) -> i32 {
    for row in 0..winner_board.len() {
        for column in 0..winner_board[row].len() {
            if winner_board[row][column] == -1 {
                winner_board[row][column] = 0;
            }
        }
    }

    winner_board.iter().fold(0i32, |acc, row| {
        let sum: i32 = row.iter().sum();
        acc + sum
    })
}

fn fill_with_zeroes(board: &mut Vec<[i32;5]>) {
    for matrix_row in 0..board.len() {
        for matrix_column in 0..5 {
            board[matrix_row][matrix_column] = 0;
        }
    }
}

fn find_winners(bingo_matrixes: &Vec<Vec<[i32;5]>>) -> Vec<i32> {
    let mut winners: Vec<i32> = Vec::new();
    for matrix_number in 0..bingo_matrixes.len() {
        for row in 0..bingo_matrixes[matrix_number].len() {
            let sum_row: i32 = bingo_matrixes[matrix_number][row].iter().sum();
            if sum_row == -5 {
                winners.push(matrix_number as i32)
            }
        }
    }

    for matrix_number in 0..bingo_matrixes.len() {
        for column in 0..bingo_matrixes[matrix_number].len() {
            let mut column_vector: Vec<i32> = Vec::new();
            for row in 0..bingo_matrixes[matrix_number][column].len() {
                column_vector.push(bingo_matrixes[matrix_number][row][column]);
            }

            if column_vector.iter().sum::<i32>() == -5 {
                winners.push(matrix_number as i32);
            }
        }
    }

    winners
}

fn first_part() {
    let lines = get_input();
    let appearing_numbers = process_appearing_numbers(lines[0].clone());
    let mut bingo_matrixes: Vec<Vec<[i32;5]>> = Vec::new();

    let mut matrix_number = 0;
    let mut matrix_index = 0;

    for i in 1..lines.len() {
        if matrix_index == 0 {
            bingo_matrixes.push(Vec::new());
        }

        matrix_number = (i - 1) / 5;
        matrix_index = (i - 1) % 5;

        bingo_matrixes[matrix_number].push(process_bingo_line(lines[i].clone()));
    }

    for number in appearing_numbers {
        mark_bingo(&mut bingo_matrixes, number);
        let winner_board = find_winner(&bingo_matrixes);
        if winner_board != -1 {
            let answer = calculate_answer(&mut bingo_matrixes[winner_board as usize]);
            println!("{}", answer * number);

            break;
        }
    }
}

fn get_bingo_numbers(raw_line: String) -> Vec<HashMap<i32, bool>> {
    let line_splitted = raw_line.split(" ");
    let numbers: Vec<i32> = line_splitted
        .filter(|x| *x != "")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let mut numbers_hash = HashMap::new();
    for number in numbers {
        numbers_hash.insert(number, false);
    }

    let vec: Vec<HashMap<i32, bool>> = Vec::new();
    vec
}


fn build_bingo_boards(lines: Vec<String>) -> Vec<Vec<Vec<HashMap<i32, bool>>>>{
    let mut bingo_boards: Vec<Vec<Vec<HashMap<i32, bool>>>> = Vec::new();

    let mut matrix_number = 0;
    let mut matrix_index = 0;

    for i in 1..lines.len() {
        if matrix_index == 0 {
            bingo_boards.push(Vec::new());
        }

        matrix_number = (i - 1) / 5;
        matrix_index = (i - 1) % 5;

        bingo_boards[matrix_number].push(get_bingo_numbers(lines[i].clone()));
    }

    bingo_boards
}

// fn mark_bingo_cell(bingo_boards: Vec<Vec<Vec<HashMap<i32, bool>>>>, number: i32) {
//     for matrix_number in 0..bingo_boards.len() {
//         for matrix_row in 0..bingo_boards[matrix_number].len() {
//             for matrix_column in 0..5 {
//                 bingo_boards[matrix_number][matrix_row][matrix_column].insert(number, true);
//             }
//         }
//     }
// }

// fn second_part() {
//     let lines = get_input();
//     let appearing_numbers = process_appearing_numbers(lines[0].clone());
//     let mut bingo_boards: Vec<Vec<Vec<HashMap<i32, bool>>>> = build_bingo_boards(lines);
//     println!("bingo boards: {:?}", bingo_boards);
//
//     for number in appearing_numbers {
//         println!("number: {}", number);
//         mark_bingo_cell(&mut bingo_boards, number);
//
//         let winners = find_winners(&bingo_boards);
//
//         if winners.len() >= 1{
//             for winner in &winners {
//                 if !order_winners.contains(winner) {
//                     order_winners.push(*winner);
//                     last_number = number
//                 }
//             }
//         }
//     }
//
//     last_winner = order_winners[order_winners.len() - 1];
//     println!("Last winner {}", last_winner);
//     println!("{}", calculate_answer(&mut bingo_boards[last_winner as usize]) * last_number)
// }

fn main() {
    first_part();
    //second_part();
}
