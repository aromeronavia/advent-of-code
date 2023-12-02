use std::fs;

fn get_input() -> Vec<String> {
    let input = fs::read_to_string("src/input").expect("Unable to read file");
    let numbers = input
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| String::from(x))
        .collect::<Vec<String>>();

    numbers
}

fn first_part() {
    let strings = get_input();
    let mut numbers: Vec<u32> = Vec::new();
    let mut sum = 0;

    for string in strings {
        for char in string.chars() {
            if char.is_numeric() {
                numbers.push(char.to_digit(10).unwrap());
            }
        }

        let number = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
            .parse::<u32>()
            .unwrap();

        sum += number;

        numbers = Vec::new()
    }

    println!("{}", sum);
}

fn main() {
    first_part();
}
