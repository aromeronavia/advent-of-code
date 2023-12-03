use std::fs;

static NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

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

fn second_part() {
    let strings = get_input();
    let mut sum = 0;

    for string in strings {
        let mut numbers: Vec<u32> = Vec::new();

        for (i, char) in string.chars().enumerate() {
            if char.is_numeric() {
                numbers.push(char.to_digit(10).unwrap());
            } else if let Some(number) = detect_number(&string, i) {
                numbers.push(number);
            }
        }

        let number = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
            .parse::<u32>()
            .unwrap();

        sum += number;
    }

    println!("{}", sum);
}

fn detect_number(string: &String, index: usize) -> Option<u32> {
    if (index + 2) > string.len() {
        return None;
    }

    match string.get(index..index + 3) {
        Some("one") => return Some(1),
        Some("two") => return Some(2),
        Some("six") => return Some(6),
        _ => 0,
    };

    match string.get(index..index + 4) {
        Some("zero") => return Some(0),
        Some("four") => return Some(4),
        Some("five") => return Some(5),
        Some("nine") => return Some(9),
        _ => 0,
    };

    match string.get(index..index + 5) {
        Some("three") => return Some(3),
        Some("seven") => return Some(7),
        Some("eight") => return Some(8),
        _ => 0,
    };

    None
}

fn main() {
    // first_part();
    second_part();
}
