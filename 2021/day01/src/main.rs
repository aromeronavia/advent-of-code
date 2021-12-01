use std::fs;

fn main() {
    let input = fs::read_to_string("src/input").expect("Unable to read file");
    let mut increased_times = 0;
    let numbers = input
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

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
