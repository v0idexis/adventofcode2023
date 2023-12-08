use std::collections::HashSet;
// use std::convert::TryInto;
use std::fs;

fn idx_str(i: usize, j: usize) -> String {
    let indexstring = format!("{i},{j}");
    return indexstring;
}

fn main() {
    let special: Vec<char> = vec!['#', '$', '@', '&', '*', '-', '+', '=', '%', '/'];
    let mut temp_vec: Vec<i32> = vec![];
    let read = fs::read_to_string("input.txt").unwrap();
    let data = read.lines();
    let mut matrix: Vec<Vec<char>> = vec![];
    let mut visited: HashSet<String> = HashSet::new();
    // let unit: usize = 1;
    let mut sum: i32 = 0;

    // let mut line = 0;
    for line in data {
        let mut row: Vec<char> = vec![];
        for char in line.chars() {
            row.push(char);
        }
        matrix.push(row);
    }

    let row_len = matrix[0].len();
    let col_len = matrix.len();
    for i in 0..col_len {
        for j in 0..row_len {
            let element: &char = &matrix[i][j];
            // traversal pointer toggles
            let mut left = true;
            let mut top_left = true;
            let mut top = true;
            let mut top_right = true;
            let mut right = true;
            let mut bott_right = true;
            let mut bott = true;
            let mut bott_left = true;
            if special.contains(element) {
                // edge cases (corners of the matrix)
                if 0 == j {
                    left = false;
                    top_left = false;
                    bott_left = false;
                } else if row_len - 1 == j {
                    right = false;
                    top_right = false;
                    bott_right = false;
                }
                if 0 == i {
                    top_left = false;
                    top = false;
                    top_right = false;
                } else if col_len - 1 == i {
                    bott_left = false;
                    bott = false;
                    bott_right = false;
                }

                // traversal begins -----------------------------------------------------------------------------------------------------------------

                if left {
                    let mut k = j - 1;
                    // let mut k2: i32 = k.try_into().unwrap();
                    let mut numstring: String = "".to_string();
                    let mut flag = true;
                    while flag {
                        // println!("Left: Flag: {}, position {},{}", flag, i, k);

                        if 0 == k {
                            flag = false
                        }
                        let idxstr: String = idx_str(i, k).to_string();
                        // let idxstr = idxstring.as_ref();
                        if !visited.contains(&idxstr) {
                            if matrix[i][k].is_numeric() {
                                visited.insert(idxstr);
                                numstring.insert(0, matrix[i][k]);
                            } else {
                                flag = false;
                            }
                        }
                        k = if k > 0 { k - 1 } else { 0 };
                    }
                    if numstring.len() > 0 {
                        let final_num: i32 = numstring.parse::<i32>().unwrap();
                        temp_vec.push(final_num);
                        sum += final_num;
                    }
                }

                if top_left {
                    let mut k = j - 1;
                    let mut l = i - 1;
                    let mut numstring: String = "".to_string();
                    let mut flag = true;
                    while flag {
                        // println!("Top Left: Flag: {}, position {},{}", flag, l, k);

                        if 0 == k {
                            flag = false
                        }
                        let idxstr: String = idx_str(l, k).to_string();
                        if !visited.contains(&idxstr) {
                            if matrix[l][k].is_numeric() {
                                visited.insert(idxstr);
                                k = if k > 0 { k - 1 } else { 0 };
                            } else {
                                flag = false;
                                k = if k != j - 1 { k + 1 } else { j - 1 }; // if number ends - reset pointer back to position with number
                            }
                        } else {
                            flag = false;
                            k = j;
                            l = i;
                        }
                    }
                    flag = true;
                    while flag {
                        if matrix[l][k].is_numeric() {
                            numstring.push(matrix[l][k]);
                            let idxstr: String = idx_str(l, k).to_string();
                            if !visited.contains(&idxstr) {
                                // marking visited notes while traversing from back to front
                                visited.insert(idxstr);
                            }
                            // k += 1;
                            if k < row_len - 1 {
                                k += 1
                            } else {
                                flag = false;
                            }
                        } else {
                            flag = false;
                        }
                    }

                    if numstring.len() > 0 {
                        let final_num: i32 = numstring.parse::<i32>().unwrap();
                        temp_vec.push(final_num);
                        sum += final_num;
                    }
                }

                if top {
                    let mut k = j;
                    let l = i - 1;
                    let mut numstring: String = "".to_string();
                    let mut flag = true;
                    while flag {
                        // println!("Top: Flag: {}, position {},{}", flag, l, k);

                        if row_len - 1 == k {
                            flag = false
                        }
                        let idxstr: String = idx_str(l, k).to_string();
                        if !visited.contains(&idxstr) {
                            if matrix[l][k].is_numeric() {
                                visited.insert(idxstr);
                                numstring.push(matrix[l][k]);
                                k = if k < row_len - 1 { k + 1 } else { k };
                            } else {
                                flag = false;
                            }
                        } else {
                            flag = false;
                        }
                    }

                    if numstring.len() > 0 {
                        let final_num: i32 = numstring.parse::<i32>().unwrap();
                        temp_vec.push(final_num);
                        sum += final_num;
                    }
                }

                if top_right {
                    let mut k = j + 1;
                    let l = i - 1;
                    let mut numstring: String = "".to_string();
                    let mut flag = true;
                    while flag {
                        // println!("Top Right: Flag: {}, position {},{}", flag, l, k);
                        if row_len - 1 == k {
                            flag = false
                        }
                        let idxstr: String = idx_str(l, k).to_string();
                        if !visited.contains(&idxstr) {
                            if matrix[l][k].is_numeric() {
                                visited.insert(idxstr);
                                numstring.push(matrix[l][k]);
                                k = if k < row_len - 1 { k + 1 } else { k };
                            } else {
                                flag = false;
                            }
                        } else {
                            flag = false;
                        }
                    }

                    if numstring.len() > 0 {
                        let final_num: i32 = numstring.parse::<i32>().unwrap();
                        temp_vec.push(final_num);
                        sum += final_num;
                    }
                }

                if right {
                    let mut k = j + 1;
                    let l = i;
                    let mut numstring: String = "".to_string();
                    let mut flag = true;
                    while flag {
                        // println!("Right: Flag: {}, position {},{}", flag, l, k);
                        if row_len - 1 == k {
                            flag = false
                        }
                        let idxstr: String = idx_str(l, k).to_string();
                        if !visited.contains(&idxstr) {
                            if matrix[l][k].is_numeric() {
                                visited.insert(idxstr);
                                numstring.push(matrix[l][k]);
                                k = if k < row_len - 1 { k + 1 } else { k };
                            } else {
                                flag = false;
                            }
                        } else {
                            flag = false;
                        }
                    }

                    if numstring.len() > 0 {
                        let final_num: i32 = numstring.parse::<i32>().unwrap();
                        temp_vec.push(final_num);
                        sum += final_num;
                    }
                }

                if bott_right {
                    let mut k = j + 1;
                    let mut l = i + 1;
                    let mut numstring: String = "".to_string();
                    let mut flag = true;
                    while flag {
                        // println!("Bott Right: Flag: {}, position {},{}", flag, l, k);

                        if 0 == k {
                            flag = false
                        }
                        let idxstr: String = idx_str(l, k).to_string();
                        if !visited.contains(&idxstr) {
                            if matrix[l][k].is_numeric() {
                                visited.insert(idxstr);
                                k = if k > 0 { k - 1 } else { 0 };
                            } else {
                                flag = false;
                                k = if k != j + 1 { k + 1 } else { j + 1 }; // if number ends - reset pointer back to position with number
                            }
                        } else {
                            flag = false;
                            k = j;
                            l = i;
                        }
                    }
                    flag = true;
                    // let mut here: i32 = 0;
                    while flag {
                        // println!("Stuck {}", here);
                        // here += 1;
                        if matrix[l][k].is_numeric() {
                            numstring.push(matrix[l][k]);
                            let idxstr: String = idx_str(l, k).to_string();
                            if !visited.contains(&idxstr) {
                                // marking visited notes while traversing from back to front
                                visited.insert(idxstr);
                            }
                            // k += 1;
                            if k < row_len - 1 {
                                k = k + 1;
                            } else {
                                flag = false;
                            }
                        } else {
                            flag = false;
                        }
                    }

                    if numstring.len() > 0 {
                        let final_num: i32 = numstring.parse::<i32>().unwrap();
                        temp_vec.push(final_num);
                        sum += final_num;
                    }
                }

                if bott {
                    let mut k = j;
                    let l = i + 1;
                    let mut numstring: String = "".to_string();
                    let mut flag = true;
                    while flag {
                        // println!("Bott: Flag: {}, position {},{}", flag, l, k);

                        if 0 == k {
                            flag = false
                        }
                        let idxstr: String = idx_str(l, k).to_string();
                        if !visited.contains(&idxstr) {
                            if matrix[l][k].is_numeric() {
                                visited.insert(idxstr);
                                numstring.insert(0, matrix[l][k]);
                                k = if k > 0 { k - 1 } else { 0 };
                            } else {
                                flag = false;
                            }
                        } else {
                            flag = false;
                        }
                    }

                    if numstring.len() > 0 {
                        let final_num: i32 = numstring.parse::<i32>().unwrap();
                        temp_vec.push(final_num);
                        sum += final_num;
                    }
                }

                if bott_left {
                    let mut k = j - 1;
                    let l = i + 1;
                    let mut numstring: String = "".to_string();
                    let mut flag = true;
                    while flag {
                        // println!("Bott Left: Flag: {}, position {},{}", flag, l, k);

                        if 0 == k {
                            flag = false
                        }
                        let idxstr: String = idx_str(l, k).to_string();
                        if !visited.contains(&idxstr) {
                            if matrix[l][k].is_numeric() {
                                visited.insert(idxstr);
                                numstring.insert(0, matrix[l][k]);
                                k = if k > 0 { k - 1 } else { 0 };
                            } else {
                                flag = false;
                            }
                        } else {
                            flag = false;
                        }
                    }

                    if numstring.len() > 0 {
                        let final_num: i32 = numstring.parse::<i32>().unwrap();
                        temp_vec.push(final_num);
                        sum += final_num;
                    }
                }

                // end of traversal -----------------------------------------------------------------------------------------------------
            } //end if (special character)
        }
    }

    // println!("{:?}", matrix[0][0]);
    println!("{:?}", temp_vec);
    println!("\n\n\n\n\n\n");
    // println!("{:?}", visited);
    // println!("\n\n\n\n\n\n");
    println!("Sum = {sum}");
}
