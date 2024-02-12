pub mod algorithms;


const WORDS_LENGTH : usize = 5;
pub fn play<G : Guesser>(answer: &'static str, mut guesser : G, max_attempts : usize) -> Option<usize> {
let mut history = Vec::new();

    // ENSURE GUESS AND ANSWER ARE LOWERCASE
    let answer = answer.to_lowercase();
    for i in 1..= max_attempts {
        let guess = guesser.guess(&history[..]).to_lowercase();

        if guess == answer {
            println!("You win!");
            return Some(i);
        }

        let mut correctness = Correctness::compute(&answer, &guess);

        history.push(Guess { word: guess, correctness });
    }
None
}


impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Correctness; 5] {

        // Ensure the word and guess are the same length (5)
        assert_eq!(answer.len(), WORDS_LENGTH);
        assert_eq!(guess.len(), WORDS_LENGTH);


        // We assume the word and guess are all wrong
        let mut correctness = [Correctness::Incorrect; 5];
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
    pub correctness : [Correctness; 5]
}

pub trait Guesser {
    fn guess(&mut self, history : &[Guess]) -> String;
}

#[cfg(test)]
mod tests {


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


