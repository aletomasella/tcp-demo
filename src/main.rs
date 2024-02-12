use wordle_solver::Wordle;

const GAMES : &str = include_str!("../answers.txt");


fn main() -> std::io::Result<()> {
    let wordle = Wordle::new();
    Ok(())
}
