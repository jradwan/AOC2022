// Advent of Code 2022
// Day 3, Part 1
// December 4, 2022

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {

    let mut priority: u32     = 0;
    let mut common: char      = '!';
    let mut common_ascii      = 0;

    let file   = File::open("day3-input.dat")?;
    let reader = BufReader::new(file);
 
    // loop through each line of the file
    for line in reader.lines() {
        let curr_line = line?.to_string();

        // split the line into the two compartments
        let len = curr_line.chars().count();
        let comp1 = &curr_line[0 .. (len / 2)];
        let comp2 = &curr_line[(len / 2) .. len];
        println!("\nComp #1 : {}", comp1);
        println!("Comp #2 : {}", comp2);

        // loop through the items (characters) in the first compartment
        for curr_char in comp1.chars() {
           // look for the current character in the second compartment
           if comp2.contains(curr_char) {
               common = curr_char;
               common_ascii = common as u32;
           }
        }

        print!("{} exists in both compartments", common);
        println!(" (ASCII value: {})", common_ascii);

        // convert common character (ascii value) to priority value
        if common_ascii >= 97 {
            common_ascii = common_ascii - 96;
        }
        else {
            common_ascii = common_ascii - 38;
        }

        println!("Priority: {}", common_ascii);
        priority = priority + common_ascii;

    }

    println!("\nSum of priorities is: {} \n", priority);

    Ok(())
}
