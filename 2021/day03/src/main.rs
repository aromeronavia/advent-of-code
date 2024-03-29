use std::fs;

fn get_input<'a>() -> Vec<String> {
    let input = fs::read_to_string("src/input").expect("Unable to read file");
    let lines = input
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    lines
}

fn first_part() {
    let input = get_input();

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for column_index in 0..12 {
        let mut zeroes = 0;
        let mut ones = 0;

        if column_index == 12 {
            break;
        }

        for number_index in 0..12 {
            if input[number_index].chars().nth(column_index).unwrap() == '1' {
                ones += 1;
            }
            if input[number_index].chars().nth(column_index).unwrap() == '0' {
                zeroes += 1;
            }
        }

        if ones > zeroes {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }

    let final_gamma = isize::from_str_radix(&gamma, 2).unwrap();
    let final_epsilon = isize::from_str_radix(&epsilon, 2).unwrap();

    println!("{}", final_gamma * final_epsilon);
}

fn second_part(i: Vec<String>) {
    let mut input = i.clone();
    let mut column_index = 0;

    let mut oxigen_generator_rating = String::new();
    let mut co2 = String::new();

    loop {
        let mut number_index = 0;
        let mut zeroes = 0;
        let mut ones = 0;

        if input.len() == 1 {
            break;
        }

        loop {
            if number_index == input.len() {
                break;
            }

            if input[number_index].chars().nth(column_index).unwrap() == '1' {
                ones += 1;
            }
            if input[number_index].chars().nth(column_index).unwrap() == '0' {
                zeroes += 1;
            }

            number_index += 1;
        }

        let mut new_numbers: Vec<String> = Vec::new();

        if ones >= zeroes {
            number_index = 0;
            loop {
                if number_index == input.len() {
                    break;
                }

                let character = input[number_index].chars().nth(column_index).unwrap();
                if character == '1' {
                    new_numbers.push(input[number_index].clone());
                }

                number_index += 1;
            }

            oxigen_generator_rating.push_str("1");
        }

        if zeroes > ones {
            number_index = 0;
            loop {
                if number_index == input.len() {
                    break;
                }

                let character = input[number_index].chars().nth(column_index).unwrap();
                if character == '0' {
                    new_numbers.push(input[number_index].clone());
                }

                number_index += 1;
            }

            oxigen_generator_rating.push_str("0");
        }

        input = new_numbers;
    }

    input = i.clone();

    column_index = 0;

    loop {
        let mut number_index = 0;
        let mut zeroes = 0;
        let mut ones = 0;

        if input.len() == 1 {
            break;
        }

        loop {
            if number_index == input.len() {
                break;
            }

            if input[number_index].chars().nth(column_index).unwrap() == '1' {
                ones += 1;
            }
            if input[number_index].chars().nth(column_index).unwrap() == '0' {
                zeroes += 1;
            }

            number_index += 1;
        }

        let mut new_numbers: Vec<String> = Vec::new();

        if zeroes <= ones {
            number_index = 0;

            loop {
                if number_index == input.len() {
                    break;
                }

                let character = input[number_index].chars().nth(column_index).unwrap();
                if character == '0' {
                    new_numbers.push(input[number_index].clone());
                }

                number_index += 1;
            }

            co2.push_str("0");
        }

        if ones < zeroes {
            number_index = 0;
            loop {
                if number_index == input.len() {
                    break;
                }

                let character = input[number_index].chars().nth(column_index).unwrap();
                if character == '1' {
                    new_numbers.push(input[number_index].clone());
                }

                number_index += 1;
            }

            co2.push_str("1");
        }

        column_index += 1;
        input = new_numbers;
    }

    oxigen_generator_rating = input[0].clone();
    co2 = input[0].clone();

    let final_oxigen = isize::from_str_radix(&oxigen_generator_rating, 2).unwrap();
    let final_co2 = isize::from_str_radix(&co2, 2).unwrap();

    println!("{}", final_oxigen * final_co2);
}

fn main() {
    first_part();
    let input = get_input();
    second_part(input);
}
