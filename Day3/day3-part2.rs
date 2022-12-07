// Advent of Code 2022
// Day 3, Part 2
// December 4, 2022

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {

    let file   = File::open("day3-input.dat")?;
    let reader = BufReader::new(file);

    let mut comp1             = "".to_string();
    let mut comp2             = "".to_string();
    let mut comp3             = "".to_string();
    let mut common: char      = '!';
    let mut common_ascii      = 0;
    let mut counter           = 0;
    let mut priority: u32     = 0;
 
    // loop through each line of the file
    for line in reader.lines() {
        let curr_line = line?.to_string();
        counter = counter + 1;

        // need to process three compartments (a group) at a time, so populate them
        match counter {
           1 => comp1 = curr_line.clone(),
           2 => comp2 = curr_line.clone(),
           3 => {
               comp3 = curr_line.clone();
               println!("\nComp 1: {}", comp1);
               println!("Comp 2: {}", comp2);
               println!("Comp 3: {}", comp3);

               // loop through the items (characters) in the first compartment
               for curr_char in comp1.chars() {
                   // look for the current character in the other compartments for a match
                   if comp2.contains(curr_char) && comp3.contains(curr_char) {
                       common = curr_char;
                       common_ascii = common as u32;
                       print!("{} exists in all three compartments", common);
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
  
                       // the badge has been found, break out of the loop scanning comp1 chars
                       break;
                   }     
               }
           

               // reset counter to load the next three compartments
               counter = 0;
           }
           _ => (), // end of match
        }
    }

    println!("\nSum of priorities is: {} \n", priority);

    Ok(())
}
