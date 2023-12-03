use std::fs;
static SYMBOLS: [char; 4] = ['*', '#', '$', '+'];

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
    let mut sum = 0;

    for (row, string) in strings.iter().enumerate() {
        for (column, char) in string.chars().enumerate() {
            if SYMBOLS.contains(&char) {
                let number = get_adjacent_numbers_sum(&strings, row, column);
                sum += number;
            }
        }
    }

    println!("total sum: {}", sum);
}

fn get_adjacent_numbers_sum(strings: &Vec<String>, row: usize, column: usize) -> i32 {
    let mut sum = 0;

    if let Some(number) = number_left(strings, row, column) {
        let var = find_left(strings.get(row).unwrap(), column - 1, String::from(number))
            .parse::<i32>()
            .unwrap();

        sum += var;
    }

    if let Some(number) = number_right(strings, row, column) {
        let var = find_left(strings.get(row).unwrap(), column + 2, String::from(number))
            .parse::<i32>()
            .unwrap();

        sum += var;
    }

    if let Some(number) = number_right_above(strings, row, column) {
        let left = find_left(strings.get(row - 1).unwrap(), column, String::from(number));
        let right = find_right(strings.get(row - 1).unwrap(), column + 1, String::from(""));
        let var = format!("{}{}", left, right).parse::<i32>().unwrap();
        println!("Var number right above {}", var);
        sum += var;
    } else {
        if let Some(number) = number_top_left(strings, row, column) {
            let var = find_left(
                strings.get(row - 1).unwrap(),
                column - 1,
                String::from(number),
            )
            .parse::<i32>()
            .unwrap();

            println!("Var top left: {}", var);

            sum += var;
        }

        if let Some(number) = number_top_right(strings, row, column) {
            let var = find_right(
                strings.get(row - 1).unwrap(),
                column + 2,
                String::from(number),
            )
            .parse::<i32>()
            .unwrap();

            println!("Var top right: {}", var);

            sum += var;
        }
    }
    if let Some(number) = number_right_below(strings, row, column) {
        let left = find_left(strings.get(row + 1).unwrap(), column, String::from(number));
        let right = find_right(strings.get(row + 1).unwrap(), column + 1, String::from(""));
        let var = format!("{}{}", left, right).parse::<i32>().unwrap();
        println!("Var number right below {}", var);
        sum += var;
    } else {
        if let Some(number) = number_bottom_left(strings, row, column) {
            let var = find_left(
                strings.get(row + 1).unwrap(),
                column - 1,
                String::from(number),
            )
            .parse::<i32>()
            .unwrap();

            println!("Var bottom left: {}", var);
            sum += var;
        }

        if let Some(number) = number_bottom_right(strings, row, column) {
            let var = find_right(
                strings.get(row + 1).unwrap(),
                column + 2,
                String::from(number),
            )
            .parse::<i32>()
            .unwrap();

            println!("Var bottom right  {}", var);
            sum += var;
        }
    }

    sum
}

fn number_right_above(strings: &Vec<String>, row: usize, column: usize) -> Option<char> {
    if row == 0 {
        return None;
    }

    let row_string = strings.get(row - 1).unwrap();
    if row_string.chars().nth(column).unwrap().is_digit(10) {
        return Some(row_string.chars().nth(column).unwrap());
    }

    None
}

fn number_left(strings: &Vec<String>, row: usize, column: usize) -> Option<char> {
    if column == 0 {
        return None;
    }

    let row_string = strings.get(row).unwrap();
    if row_string.chars().nth(column - 1).unwrap().is_digit(10) {
        return Some(row_string.chars().nth(column - 1).unwrap());
    }

    None
}

fn number_right(strings: &Vec<String>, row: usize, column: usize) -> Option<char> {
    if column == strings.get(0).unwrap().len() - 1 {
        return None;
    }

    let row_string = strings.get(row).unwrap();
    if row_string.chars().nth(column + 1).unwrap().is_digit(10) {
        return Some(row_string.chars().nth(column + 1).unwrap());
    }

    None
}

fn number_right_below(strings: &Vec<String>, row: usize, column: usize) -> Option<char> {
    if row == strings.len() - 1 {
        return None;
    }

    let row_string = strings.get(row + 1).unwrap();
    if row_string.chars().nth(column).unwrap().is_digit(10) {
        return Some(row_string.chars().nth(column).unwrap());
    }

    None
}

fn number_top_left(strings: &Vec<String>, row: usize, column: usize) -> Option<char> {
    if row == 0 || column == 0 {
        return None;
    }

    let row_string = strings.get(row - 1).unwrap();
    let char = row_string.chars().nth(column - 1).unwrap();
    if char.is_digit(10) {
        return Some(char);
    }

    None
}

fn number_top_right(strings: &Vec<String>, row: usize, column: usize) -> Option<char> {
    if row == 0 || column == strings.get(0).unwrap().len() - 1 {
        return None;
    }

    let row_string = strings.get(row - 1).unwrap();
    if row_string.chars().nth(column + 1).unwrap().is_digit(10) {
        return Some(row_string.chars().nth(column + 1).unwrap());
    }

    None
}

fn number_bottom_left(strings: &Vec<String>, row: usize, column: usize) -> Option<char> {
    if row == strings.len() - 1 || column == strings.get(0).unwrap().len() - 1 {
        return None;
    }

    let row_string = strings.get(row + 1).unwrap();
    if row_string.chars().nth(column - 1).unwrap().is_digit(10) {
        return Some(row_string.chars().nth(column - 1).unwrap());
    }

    None
}

fn number_bottom_right(strings: &Vec<String>, row: usize, column: usize) -> Option<char> {
    if row == strings.len() - 1 || column == strings.get(0).unwrap().len() - 1 {
        return None;
    }

    let row_string = strings.get(row + 1).unwrap();
    if row_string.chars().nth(column + 1).unwrap().is_digit(10) {
        return Some(row_string.chars().nth(column + 1).unwrap());
    }

    None
}

fn find_left(string: &String, index: usize, number: String) -> String {
    if index == 0 {
        return number.clone();
    }

    match string.get(index - 1..index) {
        Some(char) => {
            if char != "." {
                let appended_string = format!("{}{}", char, number);
                find_left(string, index - 1, appended_string)
            } else {
                number.clone()
            }
        }
        None => number.clone(),
    }
}

fn find_right(string: &String, index: usize, number: String) -> String {
    match string.get(index..index + 1) {
        Some(char) => {
            if char != "." {
                let appended_string = format!("{}{}", number, char);
                find_right(string, index + 1, appended_string)
            } else {
                number.clone()
            }
        }
        None => number.clone(),
    }
}

fn main() {
    first_part();
}
