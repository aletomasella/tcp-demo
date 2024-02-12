pub mod algorithms;


pub fn play (answer: &'static str, guesser : impl Guesser) {
    todo!("Implement the play function")
}



pub enum Correctness {
    //Green
    Correct,
    //Yellow
    Present,
    //Grey
    Incorrect
}

pub struct Guess {
    pub word : String,
    pub mask : [Correctness; 5]
}

pub trait Guesser {
    fn guess(&mut self, history : &[Guess]) -> String;
}


