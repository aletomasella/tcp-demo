use wordle_solver::Wordle;
use wordle_solver::algorithms::{Naive};
const GAMES : &str = include_str!("../answers.txt");
const MAX_ATTEMPTS : usize = 32;

fn main() -> std::io::Result<()> {
    let wordle = Wordle::new();

    let mut total = 0;

    // for line in GAMES.lines() {
    //
    //     let result = wordle.play(word, Naive::new(), guesses);
    //
    //     if result.is_none() {
    //         println!("Failed to solve {}", word);
    //     }
    //
    //     total += 1;
    // }

    let first_line = GAMES.lines().next().unwrap();

    let word = first_line.trim();

    let result = wordle.play(word, Naive::new(), MAX_ATTEMPTS);

    if result.is_none() {
        println!("Failed to solve {}", word);
    } else {
        println!("Solved {} in {} guesses", word, result.unwrap());
    }


    Ok(())
}
