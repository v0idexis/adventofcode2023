use std::fs;

fn main() {
    let read = fs::read_to_string("input.txt").unwrap();
    let data = read.lines();
    let mut sum = 0;
    for line in data {
        if line.len() == 0 {
            break;
        }
        let split_lines: Vec<&str> = line.split(":").collect();
        let rhs = split_lines[1];
        let cube_sets: Vec<&str> = rhs.split(";").collect();
        let (mut rmax, mut gmax, mut bmax) = (1, 1, 1);
        let mut prod = 0;
        for set in cube_sets {
            let balls = set.split(",");
            for ball in balls {
                let ball_in: Vec<&str> = ball.trim().split(" ").collect();
                let ball_count: i32 = ball_in[0].parse().unwrap();
                match ball_in[1] {
                    "red" => {
                        if ball_count > rmax {
                            rmax = ball_count;
                        }
                    }
                    "green" => {
                        if ball_count > gmax {
                            gmax = ball_count;
                        }
                    }
                    "blue" => {
                        if ball_count > bmax {
                            bmax = ball_count;
                        }
                    }
                    _ => {
                        println!("if you are here, something is fucked");
                    }
                }
            }
        }
        prod = rmax * gmax * bmax;
        sum += prod;
    }
    println!("Sum = {}", sum)
}
