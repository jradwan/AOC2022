// Advent of Code 2022
// Day 2, Part 1
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

        // decode my moves and related score
        match my_move {
          "X" => { my_move = "rock";     my_score = my_score + 1; }
          "Y" => { my_move = "paper";    my_score = my_score + 2; }
          "Z" => { my_move = "scissors"; my_score = my_score + 3; }
          _   => my_move = "invalid",
        };

        println!("Op move       : {}", op_move);
        println!("My move       : {}", my_move);
        print!("Result        : ");

        // determine outcome of the match
        if op_move == "rock" {
            match my_move {
                "rock"     => { my_score = my_score + 3; println!("draw"); }
                "paper"    => { my_score = my_score + 6; println!("win");  } 
                "scissors" => { my_score = my_score + 0; println!("lose"); }
                _          => (),
            }
        }
        if op_move == "paper" {
            match my_move {
                "rock"     => { my_score = my_score + 0; println!("lose"); }
                "paper"    => { my_score = my_score + 3; println!("draw"); }
                "scissors" => { my_score = my_score + 6; println!("win");  }
                _          => (),
            }
        }
        if op_move == "scissors" {
            match my_move {
                "rock"     => { my_score = my_score + 6; println!("win");  }
                "paper"    => { my_score = my_score + 0; println!("lose"); }
                "scissors" => { my_score = my_score + 3; println!("draw"); }
                _          => (),
            }
        }
        
        println!("Current score : {}\n", my_score);
    }

    println!("\nMy total score is: {} \n", my_score);

    Ok(())
}
