use std::io;

use rand::Rng;

fn main() {
    let mut win = 0;
    let mut tie = 0;
    let mut lose = 0;
    println!("## ROCK | PAPER | SCISSORS ##");
    loop {
        println!("Choose your fighter!");
        let mut player_in = String::new();
        io::stdin()
            .read_line(&mut player_in)
            .expect("Failure to read line");
        if player_in != "rock" && player_in != "paper" && player_in != "scissors" {
            println!("Invalid input, please try again.");
            continue;
        } else {
            let result = play_round(player_in, get_computer_choice());
        }
    }
}

fn get_computer_choice() -> String {
    let rand_num: u8 = rand::thread_rng().gen_range(0..=2);
    match rand_num {
        0 => String::from("rock"),
        1 => String::from("paper"),
        _ => String::from("scissors"),
        // For some reason, `match` is exhaustive across all possible values
        // of u8 instead of just over the range [0, 2]. The _ is there for the
        // compiler to not complain - rand_num should never be anything other
        // than 0, 1, or 2.
    }
}

fn play_round(player_selection: String, computer_selection: String) -> String {
    if player_selection == computer_selection {
        println!("It's a tie - you both picked {}.", player_selection);
        String::from("T")
    } else if (player_selection == "rock" && computer_selection == "scissors")
        || (player_selection == "paper" && computer_selection == "rock")
        || (player_selection == "scissors" && computer_selection == "paper")
    {
        println!(
            "You win - {} beats {}.",
            player_selection, computer_selection
        );
        String::from("W")
    } else {
        println!(
            "You lose - {} beats {}.",
            computer_selection, player_selection
        );
        String::from("L")
    }
}

fn is_game_over(w: i32, l: i32) -> bool {
    w == 5 || l == 5
}
