// Advent of Code 2022
// Day 2, Part 2
// December 4, 2022

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {

    let file   = File::open("day2-input.dat")?;
    let reader = BufReader::new(file);
 
    // variables
    let mut op_move;
    let mut my_move;
    let mut my_score: u32 = 0;

    // loop through each line of the file
    for line in reader.lines() {
        let curr_line = line?.to_string();
        // split the line into opponent (op) and self (my) moves
        let moves: Vec<&str> = curr_line.split_whitespace().collect();
        op_move = moves[0];
        my_move = moves[1];
        
        // decode opponent moves
        match op_move {
          "A" => op_move = "rock",
          "B" => op_move = "paper",
          "C" => op_move = "scissors",
          _   => op_move = "invalid",
        };

        // decode desired outcome and related score
        match my_move {
          "X" => { my_move = "lose"; my_score = my_score + 0; }
          "Y" => { my_move = "draw"; my_score = my_score + 3; }
          "Z" => { my_move = "win";  my_score = my_score + 6; }
          _   => my_move = "invalid",
        };

        println!("Op move        : {}", op_move);
        println!("Desired Result : {}", my_move);

        // determine my proper move to match the desired outcome of the match
        if op_move == "rock" {
            match my_move {
                "lose" => { my_score = my_score + 3; println!("My move        : scissors"); }
                "win"  => { my_score = my_score + 2; println!("My move        : paper");    } 
                "draw" => { my_score = my_score + 1; println!("My move        : rock");     }
                _      => (),
            }
        }
        if op_move == "paper" {
            match my_move {
                "lose" => { my_score = my_score + 1; println!("My move        : rock");     }
                "win"  => { my_score = my_score + 3; println!("My move        : scissors"); } 
                "draw" => { my_score = my_score + 2; println!("My move        : paper");    }
                _          => (),
            }
        }
        if op_move == "scissors" {
            match my_move {
                "lose" => { my_score = my_score + 2; println!("My move        : paper");    }
                "win"  => { my_score = my_score + 1; println!("My move        : rock");    } 
                "draw" => { my_score = my_score + 3; println!("My move        : scissors"); }
                _          => (),
            }
        }
        
        println!("Current score  : {}\n", my_score);
    }

    println!("\nMy total score is: {} \n", my_score);

    Ok(())
}
