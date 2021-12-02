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
    let lines = get_input();
    let mut depth = 0;
    let mut width = 0;
    let mut iterator = 0;

    loop {
        if iterator == lines.len() {
            break;
        }

        let mut line = lines[iterator].split(" ");
        let operator = line.next();
        let number: i32 = line.next().unwrap().parse().unwrap();

        match operator {
            Some("up") => depth -= number,
            Some("down") => depth += number,
            Some("forward") => width += number,
            Some(&_) => (),
            None => ()
        }

        iterator += 1;
    }
    println!("{}", depth * width);
}

fn second_part() {
    let lines = get_input();
    let mut aim = 0;
    let mut horizontal_position = 0;
    let mut iterator = 0;
    let mut depth = 0;

    loop {
        if iterator == lines.len() {
            break;
        }

        let mut line = lines[iterator].split(" ");
        let operator = line.next();
        let number: i32 = line.next().unwrap().parse().unwrap();

        match operator {
            Some("up") => aim -= number,
            Some("down") => aim += number,
            Some("forward") => {
                horizontal_position += number;
                depth += number * aim;
            },
            Some(&_) => (),
            None => ()
        }

        iterator += 1;
    }
    println!("{}", horizontal_position * depth);
}

fn main() {
    first_part();
    second_part();
}
