use std::fs;

fn main() {
    // Day 1
    // let file = fs::read_to_string("quiz1_input.txt").expect("Error, file not there");
    // let fat_elf = get_the_elf(file);

    // println!("Day 1 ----------------------------------------------------------");
    // println!("The fatest elf is {}", fat_elf.0);
    // println!("The sum is {}", fat_elf.0 + fat_elf.1 + fat_elf.2);
    // println!("----------------------------------------------------------------");

    // Day 2
    let file = fs::read_to_string("quiz2_input.txt").expect("Error, file not read");
    let points = get_total_points(file.clone());

    println!("The total point is {}", points);

    let second_points = get_total_points2(file);

    println!("The total point is {}", second_points);
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

    let mut sum = content.lines().into_iter().fold(0, |mut total, line| {
        let decision_string = line.split(" ").collect::<Vec<&str>>();
        let elf_decision = decision_string[0];
        let my_decision = decision_string[1];
        match elf_decision {
            "A" => {
                if my_decision == "X" {
                    total += (1 + 3);
                } else if my_decision == "Y" {
                    total += (2 + 6);
                }
                else if my_decision == "Z"{
                    total += (3);
                }
            }
            "B" => {
                if my_decision == "X" {
                    total += (1);
                } else if my_decision == "Y" {
                    total += (2 + 3);
                }
                else if my_decision == "Z"{
                    total += (3 + 6);
                }
            }
            "C" => {
                if my_decision == "X" {
                    total += (1 + 6);
                } else if my_decision == "Y" {
                    total += (2);
                }
                else if my_decision == "Z"{
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
                }
                else if my_decision == "Z"{
                    total += (2 + 6);
                }
            }
            "B" => {
                if my_decision == "X" {
                    total += (1 + 0);
                } else if my_decision == "Y" {
                    total += (2 + 3);
                }
                else if my_decision == "Z"{
                    total += (3 + 6);
                }
            }
            "C" => {
                if my_decision == "X" {
                    total += (2 + 0);
                } else if my_decision == "Y" {
                    total += (3 + 3);
                }
                else if my_decision == "Z"{
                    total += (1 + 6);
                }
            }
            _ => {}
        }
        total
    });
    sum
}
