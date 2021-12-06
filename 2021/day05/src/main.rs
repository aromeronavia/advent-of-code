use std::fs;
use std::cmp;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Rect {
    first_coordinate: Coordinate,
    second_coordinate: Coordinate
}

#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Coordinate {}

impl Hash for Coordinate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn get_input<'a>() -> Vec<String> {
    let input = fs::read_to_string("src/input").expect("Unable to read file");
    let lines = input
        .split("\n")
        .filter(|x| *x != "")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    lines
}

fn get_rects(lines: &Vec<String>) -> Vec<Rect> {
    let mut rects: Vec<Rect> = Vec::new();

    for line in lines {
        let splitted_line = line.split("->").collect::<Vec<&str>>();
        let splitted_first_coordinate = splitted_line[0].split(",").map(|x| x.trim()).collect::<Vec<&str>>();
        let splitted_second_coordinate = splitted_line[1].split(",").map(|x| x.trim()).collect::<Vec<&str>>();

        let first_coordinate = Coordinate {
            x: splitted_first_coordinate[0].parse().unwrap(), y: splitted_first_coordinate[1].parse().unwrap()
        };
        let second_coordinate = Coordinate {
            x: splitted_second_coordinate[0].parse().unwrap(), y: splitted_second_coordinate[1].parse().unwrap()
        };

        let rect = Rect { first_coordinate, second_coordinate };
        rects.push(rect);
    }

    rects
}

fn draw_space(rects: &Vec<Rect>) -> HashMap<Coordinate, i32>{
    let mut space: HashMap<Coordinate, i32> = HashMap::new();

    for rect in rects.clone() {
        let Rect { first_coordinate, second_coordinate } = &rect;

        // diagonal case
        if first_coordinate.x != second_coordinate.x && first_coordinate.y != second_coordinate.y {
            continue;
        }

        if first_coordinate.y == second_coordinate.y {
            let max = cmp::max(first_coordinate.x, second_coordinate.x);
            let min = cmp::min(first_coordinate.x, second_coordinate.x);

            for i in min..=max {
                let coordinate = Coordinate {x: i, y: first_coordinate.y};
                match space.get(&coordinate) {
                    Some(&value) => {
                        space.insert(coordinate, value + 1);
                    },
                    None => { space.insert(coordinate, 1); }
                }
            }
        }

        else if first_coordinate.x == second_coordinate.x ||
            first_coordinate.y > second_coordinate.y {
            let max = cmp::max(first_coordinate.y, second_coordinate.y);
            let min = cmp::min(first_coordinate.y, second_coordinate.y);

            for i in min..=max {
                let coordinate = Coordinate {x: first_coordinate.x, y: i};
                match space.get(&coordinate) {
                    Some(&value) => { space.insert(coordinate, value + 1); },
                    None => { space.insert(coordinate, 1); }
                }
            }
        }

    }

    space
}

fn draw_with_diagonals(rects: &Vec<Rect>) -> HashMap<Coordinate, i32> {
    let mut space: HashMap<Coordinate, i32> = HashMap::new();

    for rect in rects.clone() {
        let Rect { first_coordinate, second_coordinate } = &rect;

        // diagonal case
        if first_coordinate.x != second_coordinate.x && first_coordinate.y != second_coordinate.y {
            if first_coordinate.x < second_coordinate.x && first_coordinate.y < second_coordinate.y {
                let distance = second_coordinate.x - first_coordinate.x;

                for i in 0..=distance {
                    let coordinate = Coordinate {x: first_coordinate.x + i, y: first_coordinate.y + i};
                    match space.get(&coordinate) {
                        Some(&value) => {
                            space.insert(coordinate, value + 1);
                        },
                        None => { space.insert(coordinate, 1); }
                    }
                }
            }

            if first_coordinate.x > second_coordinate.x && first_coordinate.y > second_coordinate.y {
                let distance = first_coordinate.x - second_coordinate.x;

                for i in 0..=distance {
                    let coordinate = Coordinate {x: second_coordinate.x + i, y: second_coordinate.y + i};
                    match space.get(&coordinate) {
                        Some(&value) => {
                            space.insert(coordinate, value + 1);
                        },
                        None => { space.insert(coordinate, 1); }
                    }
                }
            }

            if first_coordinate.x < second_coordinate.x && first_coordinate.y > second_coordinate.y {
                let distance = second_coordinate.x - first_coordinate.x;

                for i in 0..=distance {
                    let coordinate = Coordinate {x: first_coordinate.x + i, y: first_coordinate.y - i};
                    match space.get(&coordinate) {
                        Some(&value) => {
                            space.insert(coordinate, value + 1);
                        },
                        None => { space.insert(coordinate, 1); }
                    }
                }
            }

            if first_coordinate.x > second_coordinate.x && first_coordinate.y < second_coordinate.y {
                let distance = first_coordinate.x - second_coordinate.x;

                for i in 0..=distance {
                    let coordinate = Coordinate {x: first_coordinate.x - i, y: first_coordinate.y + i};
                    match space.get(&coordinate) {
                        Some(&value) => {
                            space.insert(coordinate, value + 1);
                        },
                        None => { space.insert(coordinate, 1); }
                    }
                }
            }
        }

        else if first_coordinate.y == second_coordinate.y {
            let max = cmp::max(first_coordinate.x, second_coordinate.x);
            let min = cmp::min(first_coordinate.x, second_coordinate.x);

            for i in min..=max {
                let coordinate = Coordinate {x: i, y: first_coordinate.y};
                match space.get(&coordinate) {
                    Some(&value) => {
                        space.insert(coordinate, value + 1);
                    },
                    None => { space.insert(coordinate, 1); }
                }
            }
        }

        else if first_coordinate.x == second_coordinate.x ||
            first_coordinate.y > second_coordinate.y {
            let max = cmp::max(first_coordinate.y, second_coordinate.y);
            let min = cmp::min(first_coordinate.y, second_coordinate.y);

            for i in min..=max {
                let coordinate = Coordinate {x: first_coordinate.x, y: i};
                match space.get(&coordinate) {
                    Some(&value) => { space.insert(coordinate, value + 1); },
                    None => { space.insert(coordinate, 1); }
                }
            }
        }

    }

    space
}

fn first_part() {
    let lines = get_input();
    let rects = get_rects(&lines);
    let space = draw_space(&rects);
    let mut count = 0;

    for value in space.values() {
        if *value > 1 {
            count += 1;
        }
    }

    println!("{}", count);
}

fn second_part() {
    let lines = get_input();
    let rects = get_rects(&lines);
    let space = draw_with_diagonals(&rects);
    let mut count = 0;

    for value in space.values() {
        if *value > 1 {
            count += 1;
        }
    }

    println!("{}", count);
}

fn main() {
    first_part();
    second_part();
}
