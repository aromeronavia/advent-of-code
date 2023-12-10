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
    let lines = get_input();
    let mut sum = 0;

    for (i, line) in lines.iter().enumerate() {
        let rolls = line.split(";").map(|x| x.trim()).collect::<Vec<&str>>();
        let mut valid = true;

        'outer: for roll in rolls {
            let numbers = roll.split(",").map(|x| x.trim()).collect::<Vec<&str>>();
            for number in numbers {
                let split = number.split(" ").collect::<Vec<&str>>();
                let number = split.get(0).unwrap().parse::<i32>().unwrap();
                let color = split.get(1).unwrap().clone();

                if number > 12 && color == "red" {
                    valid = false;
                    println!("Line is invalid omitting sum {}", i);
                    break 'outer;
                }
                if number > 13 && color == "green" {
                    valid = false;
                    println!("Line is invalid omitting sum {}", i);
                    break 'outer;
                }
                if number > 14 && color == "blue" {
                    valid = false;
                    break 'outer;
                }
            }
        }

        if !valid {
            println!("Line is invalid omitting sum {}", i);
            continue;
        } else {
            sum += i + 1;
        }
    }

    println!("First part: {}", sum);
}

fn second_part() {
    let lines = get_input();
    let mut sum = 0;

    for (_, line) in lines.iter().enumerate() {
        let rolls = line.split(";").map(|x| x.trim()).collect::<Vec<&str>>();
        let mut min_of_red = 0;
        let mut min_of_green = 0;
        let mut min_of_blue = 0;

        for roll in rolls {
            let numbers = roll.split(",").map(|x| x.trim()).collect::<Vec<&str>>();
            for number in numbers {
                let split = number.split(" ").collect::<Vec<&str>>();
                let number = split.get(0).unwrap().parse::<i32>().unwrap();
                let color = split.get(1).unwrap().clone();

                if color == "red" {
                    if number > min_of_red {
                        min_of_red = number;
                    }
                }

                if color == "green" {
                    if number > min_of_green {
                        min_of_green = number;
                    }
                }

                if color == "blue" {
                    if number > min_of_blue {
                        min_of_blue = number;
                    }
                }
            }
        }

        sum += min_of_red * min_of_green * min_of_blue;
    }

    println!("SEcond part: {}", sum);
}

fn main() {
    // first_part();
    second_part();
}
