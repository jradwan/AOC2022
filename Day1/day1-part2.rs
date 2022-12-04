// Advent of Code 2022
// Day 1, Part 2
// December 2, 2022

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {

    let file   = File::open("day1-input.dat")?;
    let reader = BufReader::new(file);

    let mut curr_cal_val  :u32;
    let mut curr_cal_cnt  :u32 = 0;
    let mut high_cal_cnt1 :u32 = 0;
    let mut high_cal_cnt2 :u32 = 0;
    let mut high_cal_cnt3 :u32 = 0;
    let mut elf           :u32 = 1;

    // loop through each line of the file
    for line in reader.lines() {
        let curr_line = line?.to_string();

        // if this isn't a blank line, proceed
        if curr_line.ne("") {
            curr_cal_val = curr_line.parse::<u32>().unwrap();
            curr_cal_cnt = curr_cal_cnt + curr_cal_val;
        } else {
            print!("\nTotal for elf #{0}: {1}", elf, curr_cal_cnt);
            // a blank line means time for a new elf
            // decide if the current calorie count is one of the top three highest so far, shift accordingly
            if curr_cal_cnt.gt(&high_cal_cnt1) {
                high_cal_cnt3 = high_cal_cnt2;
                high_cal_cnt2 = high_cal_cnt1;
                high_cal_cnt1 = curr_cal_cnt;
                print!(" ** New high calorie count #1 found, elf #{}", elf);
            } else if curr_cal_cnt.gt(&high_cal_cnt2) {
                high_cal_cnt3 = high_cal_cnt2;
                high_cal_cnt2 = curr_cal_cnt;
                print!(" ** New high calorie count #2 found, elf #{}", elf);
            } else if curr_cal_cnt.gt(&high_cal_cnt3) {
                high_cal_cnt3 = curr_cal_cnt;
                print!(" ** New high calorie count #3 found, elf #{}", elf);
            }
            // increment the counter and reset the current total for the next elf
            elf += 1;
            curr_cal_cnt = 0;
        }
    }

    println!("\n\nThe higest total calorie counts are:\n");
    println!(" #1: {}", high_cal_cnt1);
    println!(" #2: {}", high_cal_cnt2);
    println!(" #3: {}", high_cal_cnt3);
    println!("\nFor a grand total of: {}", high_cal_cnt1 + high_cal_cnt2 + high_cal_cnt3);
    println!("\n");

    Ok(())
}
