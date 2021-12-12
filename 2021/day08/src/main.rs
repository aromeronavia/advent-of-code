use std::fs;

fn get_input() -> Vec<String> {
    let input = fs::read_to_string("src/input").expect("Cannot read file");
    let lines = input.trim().split("\n");

    lines
        .map(|line| line.trim().split("|").collect::<Vec<&str>>()[1])
        .map(|string| string.trim().to_string())
        .collect::<Vec<String>>()
}

fn first_part() {
    let lines = get_input();
    println!("lines: {:?}", lines);

    let mut result = 0;

    for line in lines {
        let splitted_line = line.split(" ");
        for number in splitted_line {
            let size = number.len() as i32;
            if [2, 3, 4, 7].contains(&size) {
                println!("Number: {}", number);
                result += 1;
            }
        }
    }

    println!("Result: {}", result);
}

fn get_second_input() -> (Vec<String>, Vec<String>) {
    let input = fs::read_to_string("src/input").expect("Cannot read file");
    let lines = input.trim().split("\n");

    (
        lines
            .clone()
            .map(|line| {
                line.trim()
                    .split("|")
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>()[0]
            })
            .map(|string| string.trim().to_string())
            .collect::<Vec<String>>(),
        lines
            .clone()
            .map(|line| line.trim().split("|").collect::<Vec<&str>>()[1])
            .map(|string| string.trim().to_string())
            .collect::<Vec<String>>(),
    )
}

fn find_number_patterns(left_line: &String) -> (Vec<String>, String, Vec<String>, Vec<String>) {
    let splitted_line: Vec<&str> = left_line.trim().split(" ").collect();
    let right_digits: Vec<String> = splitted_line
        .iter()
        .find(|number| number.len() == 2)
        .unwrap()
        .chars()
        .map(|x| x.to_string())
        .collect();

    let top_digits = splitted_line
        .iter()
        .find(|number| number.len() == 3)
        .unwrap()
        .chars()
        .filter(|x| !right_digits.contains(&x.to_string()))
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let top_digit = &top_digits[0];

    let top_left_or_middle = splitted_line
        .iter()
        .find(|number| number.len() == 4)
        .unwrap()
        .chars()
        .filter(|x| {
            !right_digits.contains(&x.to_string()) && x.to_string() != top_digit.to_string()
        })
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let bottom_and_bottom_left = splitted_line
        .iter()
        .find(|number| number.len() == 7)
        .unwrap()
        .chars()
        .filter(|x| {
            !right_digits.contains(&x.to_string())
                && x.to_string() != *top_digit
                && !top_left_or_middle.contains(&x.to_string())
        })
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    (
        right_digits,
        top_digit.clone(),
        top_left_or_middle,
        bottom_and_bottom_left,
    )
}

fn solve(left_line: &String, right_line: &String) -> i32 {
    let (right_digits, top_digit, top_left_or_middle, bottom_and_bottom_left) =
        find_number_patterns(left_line);

    let numbers_to_solve: Vec<&str> = right_line.trim().split(" ").collect();
    let mut result = String::new();

    for number in numbers_to_solve {
        if number.len() == 2 {
            result.push_str("1");
        } else if number.len() == 3 {
            result.push_str("7");
        } else if number.len() == 4 {
            result.push_str("4");
        } else if number.len() == 7 {
            result.push_str("8");
        } else if number.len() == 5 {
            let digits: Vec<String> = number.chars().map(|x| x.to_string()).collect();

            if right_digits.iter().all(|x| digits.contains(x))
                && digits.iter().any(|x| *x == top_digit)
            {
                result.push_str("3");
            } else if digits.iter().any(|x| *x == top_digit)
                && top_left_or_middle.iter().all(|x| digits.contains(x))
            {
                result.push_str("5");
            } else {
                result.push_str("2");
            }
        } else if number.len() == 6 {
            let digits: Vec<String> = number.chars().map(|x| x.to_string()).collect();

            if right_digits.iter().all(|x| digits.contains(x))
                && digits.iter().any(|x| *x == top_digit)
                && top_left_or_middle.iter().all(|x| digits.contains(x))
            {
                result.push_str("9");
            } else if right_digits.iter().all(|x| digits.contains(x))
                && digits.iter().any(|x| *x == top_digit)
                && bottom_and_bottom_left.iter().all(|x| digits.contains(x))
            {
                result.push_str("0");
            } else {
                result.push_str("6");
            }
        }
    }

    println!("number {}", result);
    result.parse().unwrap()
}

fn second_part() {
    let (left, right) = get_second_input();
    let mut result = 0;

    for line in 0..left.len() {
        print!("Processing line: {} ", right[line]);
        result += solve(&left[line], &right[line]);
    }

    println!("Result: {}", result);
}

fn main() {
    // first_part();
    second_part();
}
