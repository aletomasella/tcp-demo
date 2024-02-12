use crate::{Guesser, Guess, Correctness};

pub struct Naive {
    words : Vec<String>,
    word_length : usize
}

impl Guesser for Naive {
    fn guess(&mut self, history : &[Guess]) -> String {
        todo!("Implement the Naive guesser")
    }
}