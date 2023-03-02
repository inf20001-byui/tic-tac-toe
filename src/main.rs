//  Christopher Infante
//  CSE 310 - Winter 2023

// This program is a two player tic tac toe game.
// You win by putting three of your game pieces in a row, column, or diagonal.

// import modules
use std::{fmt, io};
use colored::*;

// Main function that runs the game by calling other functions
fn main() {
    let mut gameboard = [GameStatus::Blank; 9]; // Create a gameboard array with 9 blank cells

    print!("{}", "\n Welcome to Chris' Tic Tac Toe! \n \n".bright_magenta()); // Display welcome message

    print_gameboard(&gameboard); 
    // Loop until the game is over by either a plyaer winning or there is tie
    loop { 
        println!("{}", "\nPlayer 1 (X) it's your turn".cyan());
        take_turn(GameStatus::Player1, &mut gameboard); // Take player 1's turn
        print_gameboard(&gameboard); // Print the gameboard after each turn
        print_win(check_for_winner(&gameboard)); // Check if there is a winner after each turn
        print_tie(&gameboard); // Check if there is a tie after each turn

        println!("{}", "\nPlayer 2 (O) it's your turn".cyan());
        take_turn(GameStatus::Player2, &mut gameboard); // Take player 2's turn
        print_gameboard(&gameboard); // Print the gameboard after each turn
        print_win(check_for_winner(&gameboard)); // Check if there is a winner after each turn
        print_tie(&gameboard); // Check if there is a tie after each turn
    }
}

// Game status enum to store the status of each cell
#[derive(Copy, Clone, PartialEq)]
enum GameStatus {
    Blank,
    Player1,
    Player2,
}

// Display formatting for GameStatus
impl fmt::Display for GameStatus { //
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        // Match the GameStatus to the correct character
        match *self { 
            GameStatus::Blank => write!(f, " "),
            GameStatus::Player1 => write!(f, "X"),
            GameStatus::Player2 => write!(f, "O"),
        }
    }
}

// Print the gameboard
fn print_gameboard(gameboard: &[GameStatus; 9]) {
    println!("");
    // Loop through each row and print the cells and the lines between them
    for i in 0..2 { 
        println!("{}|{}|{}", get_cell_display(gameboard[i * 3], i * 3 + 1), get_cell_display(gameboard[1 + i * 3], i * 3 + 2), get_cell_display(gameboard[2 + i * 3], i * 3 + 3)); 
        println!("-----");
    }
    // Print the last row (this is needed because there are no lines after the last row)
    println!("{}|{}|{}", get_cell_display(gameboard[2 * 3], 7), get_cell_display(gameboard[1 + 2 * 3], 8), get_cell_display(gameboard[2 + 2 * 3], 9));
}

// Get the character to display for the cell
fn get_cell_display(status: GameStatus, num: usize) -> String {
    match status { //
        GameStatus::Blank => num.to_string(),
        GameStatus::Player1 => "X".to_string(),
        GameStatus::Player2 => "O".to_string(),
    }
}

// User turn logic
fn take_turn(p: GameStatus, f: &mut [GameStatus; 9]) {
    loop { // Loop until the user enters a valid input
        let choice = (read_user_input() - 1) as usize;

        if f[choice] != GameStatus::Blank {
            println!("{}", "That spot is already taken! Pick another place.".red());
            continue;
        } else { // If the spot is not taken, place the player's mark there
            f[choice] = p;
            return;
        }
    }
}

// Read user input and check if is a number from 1 to 9 and if that spot is already taken
fn read_user_input() -> u8 {
    loop {
        let mut ans = String::new();

        io::stdin() // Read user input
            .read_line(&mut ans) // Store user input in ans
            .expect("Failed to read input"); // If there is an error, print "Failed to read input"

        let ans: i16 = match ans.trim().parse() { // Convert ans to i16, trims and spaces and verifies it is an integer
            Ok(n) => n,  // If ans is a number, store it in n
            Err(_) => { // If ans is not a number ask for a new input
                println!("{}", "Please enter a spot from 1 to 9".purple());
                continue;
            }
        };

        if ans < 1 || ans > 9 { // If ans is not a number from 1 to 9 ask for a new input
            println!("{}", "Please enter a spot from 1 to 9".purple());
            continue;
        } else { // If ans is a number from 1 to 9 return it
            return ans.try_into().unwrap();
        }
    }
}

// Logic to check if any of the winning conditions are met (3 in a row, column, or diagonal)
fn check_for_winner(f: &[GameStatus; 9]) -> GameStatus {
    // Winner -> Columns in a line
    for x in 0..3 { // Loop through the columns and check if the first cell in the column is not blank and the first cell is equal to the second cell and third cell
        if f[x] != GameStatus::Blank && f[x] == f[3 + x] && f[x] == f[6 + x] { 
            return f[x];
        }
    }

    // Winner -> Rows in a line by checking that the first cell in the row is not blank and the first cell is equal to the second cell and third cell
    for y in 0..3 { 
        if f[3 * y] != GameStatus::Blank
            && f[3 * y] == f[1 + 3 * y] 
            && f[3 * y] == f[2 + 3 * y]
        {
            return f[3 * y];
        }
    }

    // Winner -> Diagonals in a line by checking that the first cell in the diagonal is not blank and the first cell is equal to the middle cell and bottom right cell
    if f[0] != GameStatus::Blank && f[0] == f[4] && f[0] == f[8] {  //
        return f[0];
    }
    // Winner -> Diagonals in a line by checking that the top rright cell is not blank and it is equal to the middle cell and bottom left cell
    if f[2] != GameStatus::Blank && f[2] == f[4] && f[2] == f[6] {
        return f[2];
    }

    GameStatus::Blank
}

// Check if the game is a tie and exit the program
fn print_tie(f: &[GameStatus; 9]) {
    for i in 0..9 { 
        if f[i] == GameStatus::Blank { // If there is a blank cell, the game is not a tie
            return;
        }
    }
    // If there are no blank cells, the game is a tie
    println!("{}", "It's a tie!".bright_blue());
    std::process::exit(0); // Exit the program normally
}

// Display the winner and exit the program 
fn print_win(winner: GameStatus) { 
    if winner == GameStatus::Player1 {
        println!("{}", "\nPlayer 1 (X) wins!".green());
        std::process::exit(0);  // Exit the program normally
    } else if winner == GameStatus::Player2 {
        println!("{}", "\nPlayer 2 (O) wins!".green());
        std::process::exit(0); // Exit the program normally
    }
}
