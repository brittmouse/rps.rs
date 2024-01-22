use rand::Rng;
use std::io;

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
        player_in = player_in.trim().to_lowercase(); // Avoiding "Rock" as a failed input
        if player_in != "rock" && player_in != "paper" && player_in != "scissors" {
            println!("Invalid input, please try again.");
            continue;
        }
        let result = play_round(player_in, get_computer_choice());
        if result == "W" {
            win += 1;
        } else if result == "L" {
            lose += 1;
        } else {
            tie += 1;
        }

        println!("Wins: {}\nLosses: {}\nTies: {}", win, lose, tie);

        if is_game_over(win, lose) {
            break;
        }
    }
    if win > lose {
        println!("Congratulations!");
    } else {
        println!("Better luck next time...");
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
