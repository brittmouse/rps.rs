use rand::Rng;

fn main() {
    let win = 0;
    let tie = 0;
    let lose = 0;
    println!("## ROCK | PAPER | SCISSORS ##");
}

fn get_computer_choice() -> String {
    let rand_num: u8 = rand::thread_rng().gen_range(0..=2);
    match rand_num {
        0 => String::from("rock"),
        1 => String::from("paper"),
        2 => String::from("scissors"),
        _ => get_computer_choice(),
    }
}
