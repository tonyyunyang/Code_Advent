use core::num;
use std::{fs, iter::Sum};

fn main(){
    // Day 1
    let file = fs::read_to_string("quiz1_input.txt").expect("Error, file not there");
    let fat_elf = get_the_elf(file);

    println!("The fatest elf is {}", fat_elf.0);
    println!("The sum is {}", fat_elf.0 + fat_elf.1 + fat_elf.2);
}

// Day 1
fn get_the_elf(content: String) -> (u64,u64,u64){
    let mut calories = content.lines().into_iter().fold(Vec::<u64>::new(), |mut sums, line| {
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
    (calories.pop().unwrap(), calories.pop().unwrap(), calories.pop().unwrap())
}
