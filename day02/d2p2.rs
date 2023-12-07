use std::fs;

fn main() {
    // let (rmax, gmax, bmax) = (12, 13, 14);
    let read = fs::read_to_string("input.txt").unwrap();
    let data = read.lines();
    let mut sum = 0;
    for line in data {
        if line.len() == 0 {
            break;
        }
        let split_lines: Vec<&str> = line.split(":").collect();
        // let lhs = split_lines[0];
        // let lhs_vec: Vec<&str> = lhs.split(" ").collect();
        // let line_num: i32 = lhs_vec[1].parse().unwrap();
        let rhs = split_lines[1];
        let cube_sets: Vec<&str> = rhs.split(";").collect();
        let (mut rmax, mut gmax, mut bmax) = (1, 1, 1);
        let mut prod = 0;
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
                            rmax = ball_count;
                            // broken = true;
                            // break;
                        }
                    }
                    "green" => {
                        if ball_count > gmax {
                            gmax = ball_count;
                            // broken = true;
                            // break;
                        }
                    }
                    "blue" => {
                        if ball_count > bmax {
                            bmax = ball_count;
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
        // if add_sum == true {
        //     // println!("Line number {line_num}");
        //     sum += line_num;
        // }
        // println!("{:?}", line);
        prod = rmax * gmax * bmax;
        sum += prod;
    }
    println!("Sum = {}", sum)
}
