// Advent of Code 2022
// Day 1, Part 1
// December 2, 2022

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {

    let file   = File::open("day1-input.dat")?;
    let reader = BufReader::new(file);

    let mut curr_cal_val :u32;
    let mut curr_cal_cnt :u32 = 0;
    let mut high_cal_cnt :u32 = 0;
    let mut elf          :u32 = 1;

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
            // decide if the current calorie count is the highest so far
            if curr_cal_cnt.gt(&high_cal_cnt) {
                high_cal_cnt = curr_cal_cnt;
                print!(" ** New high calorie count found, elf #{}", elf);
            }
            // increment the counter and reset the current total for the next elf
            elf += 1;
            curr_cal_cnt = 0;
        }
    }

    println!("\n\nThe higest total calorie count is: {}\n", high_cal_cnt);
    Ok(())
}
