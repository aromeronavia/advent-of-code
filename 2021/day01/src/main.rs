use std::fs;

fn get_input() -> Vec<i32> {
    let input = fs::read_to_string("src/input").expect("Unable to read file");
    let numbers = input
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    numbers
}

fn first_part() {
    let numbers = get_input();
    let mut increased_times = 0;

    let mut current_number = numbers[0];
    let mut index = 1;

    loop {
        if index == numbers.len() {
            break;
        }

        let next_number = numbers[index];
        if next_number > current_number {
            increased_times += 1;
        }
        current_number = numbers[index];
        index += 1;
    }

    println!("{}", increased_times);
}

fn second_part() {
    let numbers = get_input();

    let mut left_index = 1;
    let mut right_index = 4;
    let mut current_sum: i32 = numbers[0..3].iter().sum();
    let mut increased_times = 0;

    loop {
        if right_index > numbers.len() {
            break;
        }

        let next_sum: i32 = numbers[left_index..right_index].iter().sum();
        if next_sum > current_sum {
            increased_times += 1;
        }

        current_sum = next_sum;
        left_index += 1;
        right_index += 1;
    }

    println!("{}", increased_times);
}

fn main() {
    first_part();
    second_part();
}
