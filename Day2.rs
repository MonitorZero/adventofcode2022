use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    part_one();
    part_two();
}

fn part_one(){
    let mut total_score = 0;

    let round_win = 6;
    let round_loss = 0;
    let round_draw = 3;

    let rock_throw = 1;
    let paper_throw = 2;
    let scissors_throw = 3;

    // A = Rock, B = Paper, C = Scissors
    // Y = Paper, X = Rock, Z = Scissors

    let player_rock = "A";
    let player_paper = "B";
    let player_scissors = "C";

    let my_rock = "X";
    let my_paper = "Y";
    let my_scissors = "Z";

    // Read the input file match.txt
    let file = File::open(r"G:\rust\adventofcode2\src\match.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    // Loop through each line of the file
    for line in reader.lines() {
        let line = line.unwrap();

        // Split the line into two parts
        let mut parts = line.split(" ");

        // Get the player's throw
        let player_throw = parts.next().unwrap();

        // Get the computer's throw
        let my_throw = parts.next().unwrap();

        // Check if the player won
        if player_throw == player_rock && my_throw == my_rock {
            total_score += round_draw + rock_throw;
        } else if player_throw == player_rock && my_throw == my_paper{
            total_score += round_win + paper_throw;
        } else if player_throw == player_rock && my_throw == my_scissors{
            total_score += round_loss + scissors_throw;
        }

        if player_throw == player_paper && my_throw == my_rock {
            total_score += round_loss + rock_throw;
        } else if player_throw == player_paper && my_throw == my_paper{
            total_score += round_draw + paper_throw;
        } else if player_throw == player_paper && my_throw == my_scissors{
            total_score += round_win + scissors_throw;
        }

        if player_throw == player_scissors && my_throw == my_rock {
            total_score += round_win + rock_throw;
        } else if player_throw == player_scissors && my_throw == my_paper{
            total_score += round_loss + paper_throw;
        } else if player_throw == player_scissors && my_throw == my_scissors{
            total_score += round_draw + scissors_throw;
        }
    }
    print!("Part One: {} \n", total_score);
    
}

fn part_two(){
    let mut total_score = 0;

    let round_win = 6;
    let round_loss = 0;
    let round_draw = 3;

    let rock_throw = 1;
    let paper_throw = 2;
    let scissors_throw = 3;

    // A = Rock, B = Paper, C = Scissors
    // X = Need loss, Y = Need draw, Z = Need win

    let player_rock = "A";
    let player_paper = "B";
    let player_scissors = "C";

    let need_loss = "X";
    let need_draw = "Y";
    let need_win = "Z";

    // Read the input file match.txt
    let file = File::open(r"G:\rust\adventofcode2\src\match.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    // Loop through each line of the file
    for line in reader.lines() {
        let line = line.unwrap();

        // Split the line into two parts
        let mut parts = line.split(" ");

        // Get the player's throw
        let player_throw = parts.next().unwrap();

        // Get the computer's throw
        let my_throw = parts.next().unwrap();

        // Check the need
        if player_throw == player_rock && my_throw == need_loss{
            total_score += round_loss + scissors_throw;
        }else if player_throw == player_rock && my_throw == need_draw{
            total_score += round_draw + rock_throw;
        }else if player_throw == player_rock && my_throw == need_win{
            total_score += round_win + paper_throw;
        }

        if player_throw == player_paper && my_throw == need_loss{
            total_score += round_loss + rock_throw;
        }else if player_throw == player_paper && my_throw == need_draw{
            total_score += round_draw + paper_throw;
        }else if player_throw == player_paper && my_throw == need_win{
            total_score += round_win + scissors_throw;
        }

        if player_throw == player_scissors && my_throw == need_loss{
            total_score += round_loss + paper_throw;
        }else if player_throw == player_scissors && my_throw == need_draw{
            total_score += round_draw + scissors_throw;
        }else if player_throw == player_scissors && my_throw == need_win{
            total_score += round_win + rock_throw;
        }
    }
    print!("Part Two: {} \n", total_score);

}
