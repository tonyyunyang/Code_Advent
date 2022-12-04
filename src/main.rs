use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    num,
    ops::Div,
};

fn main() {
    // Day 1
    let file = fs::read_to_string("quiz1_input.txt").expect("Error, file not there");
    let fat_elf = get_the_elf(file);

    println!("Day 1 ----------------------------------------------------------");
    println!("The fatest elf is {}", fat_elf.0);
    println!("The sum is {}", fat_elf.0 + fat_elf.1 + fat_elf.2);

    // Day 2
    let file = fs::read_to_string("quiz2_input.txt").expect("Error, file not read");
    let points = get_total_points(file.clone());
    println!("Day 2 ----------------------------------------------------------");

    println!("The total point is {}", points);

    let second_points = get_total_points2(file);

    println!("The total point is {}", second_points);

    // Day 3
    println!("Day 3 ----------------------------------------------------------");

    let file = fs::read_to_string("quiz3_input.txt").expect("Error, file not here");
    let each_match = get_each_char(file.clone());

    let total_sum = get_total_sum(each_match.clone());

    println!("The total sum of today is {}", total_sum);

    let second_match = get_each_char2();

    let total_sum2 = get_total_sum(second_match.clone());

    println!("The second total sum of today is {}", total_sum2);

    // Day 4
    println!("Day 4 ----------------------------------------------------------");

    let file = fs::read_to_string("quiz4_input.txt").expect("Error, file not here");
    let fully_overlaps = camp_cleanup(file.clone());
    
    println!("The number of  fully overlaps group is {}", fully_overlaps);

    let partly_overlaps = camp_cleanup2(file.clone());

    println!("The number of partly overlaps group is {}", partly_overlaps);

}

// Day 1
fn get_the_elf(content: String) -> (u64, u64, u64) {
    let mut calories = content
        .lines()
        .into_iter()
        .fold(Vec::<u64>::new(), |mut sums, line| {
            match line.trim().parse::<u64>() {
                Ok(sum) => {
                    let sum = *sums.pop().get_or_insert(0) + sum;
                    sums.push(sum);
                }
                Err(_) => {
                    sums.push(0);
                }
            }
            sums
        });

    calories.sort();
    (
        calories.pop().unwrap(),
        calories.pop().unwrap(),
        calories.pop().unwrap(),
    )
}

// Day 2
fn get_total_points(content: String) -> i32 {
    // let mut total: i32 = 0;

    let sum = content.lines().into_iter().fold(0, |mut total, line| {
        let decision_string = line.split(" ").collect::<Vec<&str>>();
        let elf_decision = decision_string[0];
        let my_decision = decision_string[1];
        match elf_decision {
            "A" => {
                if my_decision == "X" {
                    total += (1 + 3);
                } else if my_decision == "Y" {
                    total += (2 + 6);
                } else if my_decision == "Z" {
                    total += (3);
                }
            }
            "B" => {
                if my_decision == "X" {
                    total += (1);
                } else if my_decision == "Y" {
                    total += (2 + 3);
                } else if my_decision == "Z" {
                    total += (3 + 6);
                }
            }
            "C" => {
                if my_decision == "X" {
                    total += (1 + 6);
                } else if my_decision == "Y" {
                    total += (2);
                } else if my_decision == "Z" {
                    total += (3 + 3);
                }
            }
            _ => {}
        }
        total
    });
    sum
}

fn get_total_points2(content: String) -> i32 {
    // let mut total: i32 = 0;

    let mut sum = content.lines().into_iter().fold(0, |mut total, line| {
        let decision_string = line.split(" ").collect::<Vec<&str>>();
        let elf_decision = decision_string[0];
        let my_decision = decision_string[1];
        match elf_decision {
            "A" => {
                if my_decision == "X" {
                    total += (3 + 0);
                } else if my_decision == "Y" {
                    total += (1 + 3);
                } else if my_decision == "Z" {
                    total += (2 + 6);
                }
            }
            "B" => {
                if my_decision == "X" {
                    total += (1 + 0);
                } else if my_decision == "Y" {
                    total += (2 + 3);
                } else if my_decision == "Z" {
                    total += (3 + 6);
                }
            }
            "C" => {
                if my_decision == "X" {
                    total += (2 + 0);
                } else if my_decision == "Y" {
                    total += (3 + 3);
                } else if my_decision == "Z" {
                    total += (1 + 6);
                }
            }
            _ => {}
        }
        total
    });
    sum
}

// Day 3
fn get_each_char(content: String) -> Vec<char> {
    let chars = content
        .lines()
        .into_iter()
        .fold(vec![], |mut all_chars, line| {
            let num_of_char = line.len() as u32;
            let half_num_of_char = num_of_char.div(2);
            'outer: for i in 0..half_num_of_char {
                let first_each_char = line.clone().chars().nth(i as usize).unwrap();
                for j in half_num_of_char..num_of_char {
                    if line.chars().nth(j as usize).unwrap() == first_each_char {
                        all_chars.push(first_each_char);
                        break 'outer;
                    }
                }
            }
            all_chars
        });
    chars
}

fn get_total_sum(chars: Vec<char>) -> u32 {
    let mut sum: u32 = 0;
    // let mut test = vec![];
    // test.push('p');
    // test.push('L');
    // test.push('P');
    // test.push('v');
    // test.push('t');
    // test.push('s');
    for i in chars {
        let temp = i as u32;
        if (temp >= 65) && (temp <= 90) {
            sum += (temp - 38);
        } else if (temp >= 97) && (temp <= 122) {
            sum += (temp - 96);
        }
    }
    sum
}

fn get_each_char2() -> Vec<char> {
    let file = File::open("quiz3_input.txt").unwrap();
    let reader = BufReader::new(&file);
    let vec_all_lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let chars = vec_all_lines
        .chunks(3)
        .into_iter()
        .fold(vec![], |mut all_chars, chunk| {
            let first = chunk[0].clone();
            'outer: for i in first.chars() {
                let second = chunk[1].clone();
                for j in second.chars() {
                    if i == j {
                        let third = chunk[2].clone();
                        for z in third.chars() {
                            if j == z {
                                all_chars.push(z);
                                break 'outer;
                            }
                        }
                    }
                }
            }
            all_chars
        });
    chars
}

// Day 4
fn camp_cleanup(content: String) -> u32 {
    let sum = content.lines().into_iter().fold(0, |mut total, line|{
        let each_line = line.split(",").collect::<Vec<&str>>();
        // for i in each_line {
        //     println!("{}", i);
        // }
        let first_group = each_line[0].split("-").collect::<Vec<&str>>();
        let first_range:Vec<u32> = first_group.into_iter().map(|a| a.trim().parse::<u32>().expect("Not a number")).collect();
        // for i in first_range {
        //     println!("{}", i);
        // }
        let second_group = each_line[1].split("-").collect::<Vec<&str>>();
        let second_range:Vec<u32> = second_group.into_iter().map(|a| a.trim().parse::<u32>().expect("Not a number")).collect();

        let mut flag = true;

        if (first_range[0] <= second_range[0]) && (first_range[1] >= second_range[1]) {
            total += 1;
            flag = false;
        }

        if flag && (first_range[0] >= second_range[0]) && (first_range[1] <= second_range[1]) {
            total += 1;
        }

        total
    });
    sum
}

fn camp_cleanup2(content: String) -> u32 {
    let sum = content.lines().into_iter().fold(0, |mut total, line|{
        let each_line = line.split(",").collect::<Vec<&str>>();
        // for i in each_line {
        //     println!("{}", i);
        // }
        let first_group = each_line[0].split("-").collect::<Vec<&str>>();
        let first_range:Vec<u32> = first_group.into_iter().map(|a| a.trim().parse::<u32>().expect("Not a number")).collect();
        // for i in first_range {
        //     println!("{}", i);
        // }
        let second_group = each_line[1].split("-").collect::<Vec<&str>>();
        let second_range:Vec<u32> = second_group.into_iter().map(|a| a.trim().parse::<u32>().expect("Not a number")).collect();

        if (first_range[0] <= second_range[1]) && (first_range[1] >= second_range[0]) {
            total += 1;
        }

        total
    });
    sum
}