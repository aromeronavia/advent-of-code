use std::fs;
use std::collections::HashMap;
use std::num;
use std::cmp;

#[derive(Hash)]
struct Pair {
    first: i32,
    second: i32
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second
    }
}

impl Eq for Pair {}

fn read_input() -> Vec<i32>{
    let string = fs::read_to_string("src/input").expect("Unable to read file");
    string
        .trim()
        .split(",")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>()
}

fn get_median(input: Vec<i32>) -> i32 {
    if input.len() % 2 == 0 {
        (input[(input.len() as f32 / 2 as f32).ceil() as usize] -
        input[(input.len() as f32 / 2 as f32).floor() as usize]) +
        input[(input.len() as f32 / 2 as f32).floor() as usize]
    } else {
        input[(input.len() as i32 / 2 as i32) as usize]
    }
}

fn get_mean(input: Vec<i32>) -> i32 {
    let sum = input.iter().map(|x| *x as f32).sum::<f32>();

    (sum / input.len() as f32).floor() as i32
}

fn first_part() {
    let input: Vec<i32> = read_input();
    println!("{:?}", input);
    let mut input2 = input.clone();
    input2.sort();

    let median = get_median(input2);
    let distances = input.iter().map(|x| (median - x).abs());

    println!("Fuel: {}", distances.sum::<i32>());
}

fn second_part() {
    let input: Vec<i32> = read_input();
    let mut input2 = input.clone();
    input2.sort();

    let mean = get_mean(input2);
    let distances: Vec<i32> = input.iter().map(|x| (mean - x).abs()).collect();
    let fuel = distances.iter().map(|x| (x * (x + 1)) / 2);

    println!("Fuel: {}", fuel.sum::<i32>());
}

fn main() {
    first_part();
    second_part();
}
