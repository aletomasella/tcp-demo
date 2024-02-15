use std::collections::HashSet;

pub mod algorithms;

const DICTIONARY : &str = include_str!("../dictionary.txt");
const WORDS_LENGTH : usize = 5;

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Wordle {
        // Create a HashSet from the dictionary
        let dictionary = HashSet::from_iter(DICTIONARY.lines().map(|line| line.split_once(' ').expect("Every line is word + space + frequency").0));

        Wordle { dictionary }
    }

    pub fn play<G : Guesser>(&self, answer: &'static str, mut guesser : G, max_attempts : usize) -> Option<usize> {
        let mut history = Vec::new();
        // ENSURE GUESS AND ANSWER ARE LOWERCASE
        let answer = answer.to_lowercase();
        for i in 1..= max_attempts {
            let guess = guesser.guess(&history[..]).to_lowercase();
            // Ensure the guess is in the dictionary

            if self.dictionary.len() == 0 {
                println!("Dictionary is empty");
                return None;
            }

            if !self.dictionary.contains(&guess[..]) {
                println!("{} is not in the dictionary", guess);
                continue;
            }
            if guess == answer {
                println!("You win!");
                return Some(i);
            }
            let mut correctness = Correctness::compute(&answer, &guess);
            history.push(Guess { word: guess, correctness });
        }
        println!("You lose!");
        None
    }
}


impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Correctness; WORDS_LENGTH] {

        // Ensure the word and guess are the same length (5)
        assert_eq!(answer.len(), WORDS_LENGTH);
        assert_eq!(guess.len(), WORDS_LENGTH);


        // We assume the word and guess are all wrong
        let mut correctness = [Correctness::Incorrect; WORDS_LENGTH];
        let mut used = [false; 5];

        // Mark the letters that are correct
        for (i, (a,g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                correctness[i] = Correctness::Correct;
                used[i] = true;
            }
        }

        // Mark the letters that are present
        for (i, g) in guess.chars().enumerate() {
            if correctness[i] == Correctness::Correct {
                continue;
            }

        // If the letter is present, but it is not already used and mark as correct, then mark as present
            if answer.chars().enumerate().any(|(i,a)| {
            if a == g && !used[i] {
                used[i] = true;
                return true;

            }
            return  false;
            }) {
                correctness[i] = Correctness::Present;

            }
        }

        correctness
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    pub correctness : [Correctness; WORDS_LENGTH]
}

impl Guess {
     pub fn new(word : String, correctness : [Correctness; WORDS_LENGTH]) -> Self {
          Guess { word, correctness }
     }


    pub fn matches(&self, word : &str) -> bool {
        // Ensure the word and guess are the same length (5)
        assert_eq!(word.len(), WORDS_LENGTH);
        assert_eq!(self.word.len(), WORDS_LENGTH);


        // Here we compare the correctness of the word and the guess
        // If the correctness of the word and the guess are the same, then the guess matches the word
        // Otherwise, the guess does not match the word
        // We can use the zip function to iterate over the correctness of the word and the guess (zip returns a tuple of the elements of the two iterators,
        // and we can use the & operator to get a reference to the elements of the tuple)
        //
        for ((g, &c), w) in self.word.chars().zip(&self.correctness).zip(word.chars()){
            match c {
                Correctness::Correct => {
                    if g != w {
                        return false;
                    }
                }
                Correctness::Present => {
                    if g != w && !self.word.contains(w) {
                        return false;
                    }
                }
                Correctness::Incorrect => {
                    if g == w {
                        return false;
                    }
                }
            }
        }
        true
    }
}
pub trait Guesser {
    fn guess(&mut self, history : &[Guess]) -> String;
}

impl Guesser for fn(history : &[Guess]) -> String {
    fn guess(&mut self, history : &[Guess]) -> String {
        self(history)
    }
}



#[cfg(test)]
macro_rules! guesser {
(|$history:ident| $impl:block) => {{
    struct G;
    impl $crate::Guesser for G {
        fn guess(&mut self, $history : &[Guess]) -> String {
            $impl
        }
    }
    G
}};
}

#[cfg(test)]
mod tests {

    mod game {
        use crate::{Guess,Wordle};

        #[test]
        fn guess_one () {
            let wordle = Wordle::new();

            let guesser = guesser!(|_history| {
                "hello".to_string()
            });

            assert_eq!(wordle.play("hello", guesser, 10), Some(1));
            }


        #[test]
        fn guess_two () {
            let wordle = Wordle::new();

            let guesser = guesser!(|_history| {
                if _history.len() == 1 {
                    return "hello".to_string()
               }
                "world".to_string()
            });

            assert_eq!(wordle.play("hello", guesser, 10), Some(2));
        }


        #[test]
        fn wrong_guess () {
            let wordle = Wordle::new();

            let guesser = guesser!(|_history| {
                "world".to_string()
            });

            assert_eq!(wordle.play("hello", guesser, 10), None);
        }

    }

    mod compute {
        use super::super::*;

        // MACRO TO TEST
        macro_rules! mask {
       (C) => {Correctness::Correct};
       (P) => {Correctness::Present};
       (I) => {Correctness::Incorrect};
        ($($c:tt)+) => {[
          $(mask!($c)),+
        ]}
        }


        #[test]
        fn correct() {
            let answer = "hello";
            let guess = "hello";
            let correctness = Correctness::compute(answer, guess);
            assert_eq!(correctness, mask![C C C C C]);
        }
        #[test]
        fn present() {
            let answer = "abcde";
            let guess = "eabcd";
            let correctness = Correctness::compute(answer, guess);
            assert_eq!(correctness, mask![P P P P P]);
        }
        #[test]
        fn incorrect() {
            let answer = "hello";
            let guess = "xmkts";
            let correctness = Correctness::compute(answer, guess);
            assert_eq!(correctness, mask![I I I I I]);

        }
    }
}


