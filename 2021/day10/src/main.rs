use std::collections::HashMap;
use std::fs;

const OPEN_SYMBOLS: [char; 4] = ['[', '(', '{', '<'];
const CLOSED_SYMBOLS: [char; 4] = [']', ')', '}', '>'];

fn get_input() -> Vec<String> {
    let lines = fs::read_to_string("src/input").expect("File not found");

    lines
        .trim()
        .split("\n")
        .map(|line| line.to_string())
        .collect()
}

fn first_part() {
    let input: Vec<String> = get_input();
    let mut map: HashMap<char, i32> = HashMap::new();
    map.insert(')', 3);
    map.insert(']', 57);
    map.insert('}', 1197);
    map.insert('>', 25137);

    let mut result = 0;

    for line in input {
        let mut stack: Vec<char> = Vec::new();

        for symbol in line.chars() {
            if OPEN_SYMBOLS.contains(&symbol) {
                stack.push(symbol);
            }
            if CLOSED_SYMBOLS.contains(&symbol) {
                let open_position = OPEN_SYMBOLS
                    .iter()
                    .position(|x| *x == *stack.last().unwrap());
                let closed_position = CLOSED_SYMBOLS.iter().position(|x| *x == symbol);

                if open_position == closed_position {
                    stack.pop();
                } else {
                    result += map[&symbol];
                    break;
                }
            }
        }
    }

    println!("Result: {}", result);
}

fn clean_stack(stack: Vec<char>) -> Vec<char> {
    let mut new_stack = stack.clone();

    let mut removed: bool = true;
    let mut i = 0;

    loop {
        if !removed { break; }

        removed = false;

        loop {
            let open_position = OPEN_SYMBOLS
                .iter()
                .position(|x| *x == stack[i]);
            let closed_position = CLOSED_SYMBOLS.iter().position(|x| *x == stack[i + 1]);

            if open_position == closed_position {
                new_stack.remove(i + 1);
                new_stack.remove(i);
                removed = true;
            } else {
                i += 1;
            }

            break;
        }
    }

    stack
}

fn second_part() {
    println!("0x5: {}", 0 * 5);
    let input: Vec<String> = get_input();
    let mut map: HashMap<char, i64> = HashMap::new();
    map.insert(')', 1);
    map.insert(']', 2);
    map.insert('}', 3);
    map.insert('>', 4);

    let mut scores: Vec<i64> = Vec::new();

    for line in input {
        let mut score: i64 = 0;
        let mut stack: Vec<char> = Vec::new();

        let mut corrupt = false;

        for symbol in line.chars() {
            if OPEN_SYMBOLS.contains(&symbol) {
                stack.push(symbol);
            }
            if CLOSED_SYMBOLS.contains(&symbol) {
                let open_position = OPEN_SYMBOLS
                    .iter()
                    .position(|x| *x == *stack.last().unwrap());
                let closed_position = CLOSED_SYMBOLS.iter().position(|x| *x == symbol);

                if open_position == closed_position {
                    stack.pop();
                } else {
                    corrupt = true;
                    break;
                }
            }
        }

        if corrupt { continue; }

        if stack.len() > 0 {
            stack = clean_stack(stack);
            let mut i = stack.len() - 1;
            println!("{:?}", stack);

            loop {
                let position = OPEN_SYMBOLS
                    .iter()
                    .position(|x| *x == stack[i])
                    .unwrap();

                let closing_character = CLOSED_SYMBOLS[position];
                score = (score * 5) + map[&closing_character];

                if i == 0 { break; }
                i-=1;
            }

            scores.push(score);
        }
    }

    scores.sort();
    println!("Scores: {:?}", scores[scores.len() / 2]);
}

fn main() {
    // first_part();
    second_part();
}
