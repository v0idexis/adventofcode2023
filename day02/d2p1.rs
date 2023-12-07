use std::fs;

fn main() {
    let (rmax, gmax, bmax) = (12, 13, 14);
    let read = fs::read_to_string("input.txt").unwrap();
    let data = read.lines();
    let mut sum = 0;
    for line in data {
        if line.len() == 0 {
            break;
        }
        let mut add_sum = true;
        let split_lines: Vec<&str> = line.split(":").collect();
        let lhs = split_lines[0];
        let lhs_vec: Vec<&str> = lhs.split(" ").collect();
        let line_num: i32 = lhs_vec[1].parse().unwrap();
        let rhs = split_lines[1];
        let cube_sets: Vec<&str> = rhs.split(";").collect();
        // let mut broken = false;
        for set in cube_sets {
            // println!("Set: {}", set);
            // if broken == true {
            //     break;
            // }
            // broken = true;
            let balls = set.split(",");
            for ball in balls {
                let ball_in: Vec<&str> = ball.trim().split(" ").collect();
                let ball_count: i32 = ball_in[0].parse().unwrap();
                // println!("Ball {}, count: {}", ball_in[1], ball_count);
                match ball_in[1] {
                    "red" => {
                        if ball_count > rmax {
                            add_sum = false;
                            // broken = true;
                            // break;
                        }
                    }
                    "green" => {
                        if ball_count > gmax {
                            add_sum = false;
                            // broken = true;
                            // break;
                        }
                    }
                    "blue" => {
                        if ball_count > bmax {
                            add_sum = false;
                            // broken = true;
                            // break;
                        }
                    }
                    _ => {
                        println!("if you are here, something is fucked");
                    }
                }
            }
        }
        if add_sum == true {
            // println!("Line number {line_num}");
            sum += line_num;
        }
        // println!("{:?}", line);
    }
    println!("Sum = {}", sum)
}
