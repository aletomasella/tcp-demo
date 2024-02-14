use std::collections::HashMap;
use crate::{Guesser, Guess, DICTIONARY, Correctness};

pub struct Naive {
    // STATIC STRING TO FREQUENCY
    remaining_words: HashMap<&'static str, usize>,
}


impl Naive {
   pub fn new () -> Self {
        let remaining = HashMap::from_iter(DICTIONARY.lines().map(|line| {
            let (word,count) = line.split_once(' ').expect("Every line should have a word + space + frequency");
            let count = count.parse::<usize>().expect("Every frequency should be a number");
            (word, count)
        }).into_iter());

        Naive { remaining_words: remaining }
    }

}

struct Candidate {
    word: &'static str,
    count: usize,
    goodness: f64,
}

impl  Candidate {
    fn new() -> Self {
        Candidate {
            word: "",
            count: 0,
            goodness: 0.0,
        }
    }
}

impl Guesser for Naive {

    fn guess(&mut self, history : &[Guess]) -> String {

        if let Some(guess) = history.last() {
            // TODO: Update the goodness of the words based on the last guess

            for i in 0..guess.word.len() {
                if guess.correctness[i] == Correctness::Correct {
                    // Remove all words that don't have the same letter in the same position
                    self.remaining_words.retain(|word, _| word.chars().nth(i) == guess.word.chars().nth(i));
                } else if guess.correctness[i] == Correctness::Present {
                    // Remove all words that does not contain the letter
                    self.remaining_words.retain(|word, _| word.contains(guess.word.chars().nth(i).expect("Every guess should have 5 letters")));

                } else {
                    // Remove all words that have the same letter in the same position
                    self.remaining_words.retain(|word, _| word.chars().nth(i) != guess.word.chars().nth(i));
                }
            }

        }

        // Initialize the best guess
        let mut best_guess = Candidate::new();



        for (&word, &count) in &self.remaining_words {
            // TODO: Calculate the goodness of the word
            let goodness = count as f64;

            // Get best guess based on frequency
            if count > best_guess.count {
                best_guess = Candidate { word, count, goodness };
            }
        }

        // Clear the best guess from the remaining words
        self.remaining_words.remove(best_guess.word);


        best_guess.word.to_string()

    }
}