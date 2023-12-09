use std::collections::HashMap;
use std::fs;

fn main() {
    let read = fs::read_to_string("input.txt").unwrap();
    let data = read.lines();
    let mut sum: i32 = 0;
    let mut re_count: HashMap<i32, i32> = HashMap::new();

    for line in data {
        let line_split: Vec<&str> = line.split(":").collect();
        let line_count: i32 = line_split[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        let mut curr_count: i32 = 0;

        match re_count.get(&line_count) {
            Some(count) => {
                sum = sum + count + 1;
                curr_count = *count;
            }
            None => {
                sum = sum + 1;
            }
        }

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
        let mut count: i32 = 0;
        for num in &card_pos {
            if card_win.contains(num) {
                count += 1;
            }
        }

        let start = line_count + 1;
        let end = line_count + count + 1;
        for num in start..end {
            match re_count.get(&num) {
                Some(count) => {
                    re_count.insert(num, count + 1 + curr_count);
                }
                None => {
                    re_count.insert(num, 1 + curr_count);
                }
            }
        }
    }
    // println!("{:?}", re_count);
    println!("Sum = {}", sum);
}
