use std::collections::HashMap;
use std::fs;

fn main() {
    let read = fs::read_to_string("test.txt").unwrap();
    let data = read.lines();
    let mut sum: i32 = 0;
    let mut revisit = HashMap::new();

    for line in data {
        let line_split: Vec<&str> = line.split(":").collect();
        let num_data: Vec<&str> = line_split[1].trim().split(" | ").collect();
        let card_win: Vec<i32> = num_data[0]
            .trim()
            .split_whitespace()
            .map(|num| num.trim().parse().unwrap())
            .collect();
        let card_pos: Vec<i32> = num_data[1]
            .trim()
            .split_whitespace()
            .map(|num| num.trim().parse().unwrap())
            .collect();
        let mut count: u32 = 0;
        for num in &card_pos {
            if card_win.contains(num) {
                // println!("Winning num: {num}");
                count += 1;
            }
        }

        // println!(
        //     "Win: {:?}, Possessed: {:?}, Count: {}",
        //     card_win, card_pos, count
        // );
        if count > 0 {
            println!("Sum: {sum}, Count: {count}");
            sum = sum + i32::pow(2, count - 1);
        }
    }

    println!("Sum = {}", sum);
}
