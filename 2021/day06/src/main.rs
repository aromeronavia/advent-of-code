//

fn first_part() {
    let input = [3,4,3,1,2];
    let mut fishes: Vec<i32> = input.iter().map(|x| *x).collect::<Vec<i32>>();

    let days = 256;

    for day in 0..days {
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

fn second_part() {
    let input = [6];
    let mut fishes: Vec<i32> = input.iter().map(|x| *x).collect::<Vec<i32>>();
    let days = 18;

    // 18 / 7 = 2         12 / 9 = 1
    // 11 / 7 = 1         3 / 7 = 0
    // 4 / 7 = 0

    let mut days_remainings: Vec<i32> = Vec::new();
    let mut tmp_days = days;

    days_remainings.push(days);
    let mut day_iterator = days;
    loop {
        if day_iterator - 7 < 0 {
            break;
        }

        days_remainings.push(day_iterator - 7);
        day_iterator -= 7;
    }

    println!("{:?}", days_remainings);

    let mut acc = 0;

    for index in 0..input.len() {
        for day_iterator in 0..days_remainings.len() {
            let mut day = days_remainings[day_iterator];
            let mut i = 0;

            loop {
                if i == 0 {
                    acc += days_remainings[i] - input[index] / 7;
                    day -= 7;
                }
                else {
                    acc += days_remainings[i] / 7;
                    day -= 9;
                }
            }
        }
    }

    println!("Len: {}", acc);
}

fn main() {
    //let input = [1,4,3,3,1,3,1,1,1,2,1,1,1,4,4,1,5,5,3,1,3,5,2,1,5,2,4,1,4,5,4,1,5,1,5,5,1,1,1,4,1,5,1,1,1,1,1,4,1,2,5,1,4,1,2,1,1,5,1,1,1,1,4,1,5,1,1,2,1,4,5,1,2,1,2,2,1,1,1,1,1,5,5,3,1,1,1,1,1,4,2,4,1,2,1,4,2,3,1,4,5,3,3,2,1,1,5,4,1,1,1,2,1,1,5,4,5,1,3,1,1,1,1,1,1,2,1,3,1,2,1,1,1,1,1,1,1,2,1,1,1,1,2,1,1,1,1,1,1,4,5,1,3,1,4,4,2,3,4,1,1,1,5,1,1,1,4,1,5,4,3,1,5,1,1,1,1,1,5,4,1,1,1,4,3,1,3,3,1,3,2,1,1,3,1,1,4,5,1,1,1,1,1,3,1,4,1,3,1,5,4,5,1,1,5,1,1,4,1,1,1,3,1,1,4,2,3,1,1,1,1,2,4,1,1,1,1,1,2,3,1,5,5,1,4,1,1,1,1,3,3,1,4,1,2,1,3,1,1,1,3,2,2,1,5,1,1,3,2,1,1,5,1,1,1,1,1,1,1,1,1,1,2,5,1,1,1,1,3,1,1,1,1,1,1,1,1,5,5,1];
    // first_part();
    second_part();
}

