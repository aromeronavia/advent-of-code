use std::collections::HashMap;

fn first_part(input: &[i64]) {
    let mut fishes: Vec<i64> = input.iter().map(|x| *x).collect();

    let days = 80;

    for _day in 0..days {
        for index in 0..fishes.len() {
            if fishes[index] == 0 {
                fishes[index] = 6;
                fishes.push(8);
            } else {
                fishes[index] = fishes[index] - 1;
            }
        }
    }

    println!("{}", fishes.len());
}

fn second_part(input: &[i64]) {
    let mut fishes_per_day: HashMap<i64, i64> = HashMap::new();
    let days = 256;

    let mut result: i64 = input.len() as i64;

    for i in 0..=8 {
        fishes_per_day.insert(i, 0);
    }

    for i in input {
        fishes_per_day.insert(*i, fishes_per_day[&i] + 1);
    }

    for _day in 0..days {
        let mut tmp: HashMap<i64, i64> = HashMap::new();
        for i in 0..=8 {
            tmp.insert(i, 0);
        }

        result += fishes_per_day[&0];
        tmp.insert(6, fishes_per_day[&0]);
        tmp.insert(8, fishes_per_day[&0]);

        for i in 1..=8 {
            tmp.insert(i - 1, tmp[&(i-1)] + fishes_per_day[&i]);
        }

        fishes_per_day = tmp;
    }

    println!("Result: {}", result);
}

fn main() {
    let input = [1,4,3,3,1,3,1,1,1,2,1,1,1,4,4,1,5,5,3,1,3,5,2,1,5,2,4,1,4,5,4,1,5,1,5,5,1,1,1,4,1,5,1,1,1,1,1,4,1,2,5,1,4,1,2,1,1,5,1,1,1,1,4,1,5,1,1,2,1,4,5,1,2,1,2,2,1,1,1,1,1,5,5,3,1,1,1,1,1,4,2,4,1,2,1,4,2,3,1,4,5,3,3,2,1,1,5,4,1,1,1,2,1,1,5,4,5,1,3,1,1,1,1,1,1,2,1,3,1,2,1,1,1,1,1,1,1,2,1,1,1,1,2,1,1,1,1,1,1,4,5,1,3,1,4,4,2,3,4,1,1,1,5,1,1,1,4,1,5,4,3,1,5,1,1,1,1,1,5,4,1,1,1,4,3,1,3,3,1,3,2,1,1,3,1,1,4,5,1,1,1,1,1,3,1,4,1,3,1,5,4,5,1,1,5,1,1,4,1,1,1,3,1,1,4,2,3,1,1,1,1,2,4,1,1,1,1,1,2,3,1,5,5,1,4,1,1,1,1,3,3,1,4,1,2,1,3,1,1,1,3,2,2,1,5,1,1,3,2,1,1,5,1,1,1,1,1,1,1,1,1,1,2,5,1,1,1,1,3,1,1,1,1,1,1,1,1,5,5,1].map(|x| x as i64);
    first_part(&input);
    second_part(&input);
}

