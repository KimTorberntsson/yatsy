use super::dice_result;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Score {
    score_type:  dice_result::ResultType,
    score: i32,
    closed: bool
}

impl Score {
    pub fn scored(&self) -> bool {
        self.closed || self.score != 0 
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.closed {
            write!(f, "x\t{}", self.score_type)
        } else {
            write!(f, "{}p\t{}", self.score, self.score_type)
        }
    }
}

pub struct ScoreCard {
    scores: [Score; 15]
}

impl ScoreCard {
    pub fn new() -> ScoreCard {
        ScoreCard {
            scores: [
                Score { score_type: dice_result::ResultType::Ones, score: 0, closed: false },
                Score { score_type: dice_result::ResultType::Twos, score: 0, closed: false },
                Score { score_type: dice_result::ResultType::Threes, score: 0, closed: false },
                Score { score_type: dice_result::ResultType::Fours, score: 0, closed: false },
                Score { score_type: dice_result::ResultType::Fives, score: 0, closed: false },
                Score { score_type: dice_result::ResultType::Sixes, score: 0, closed: false },
                Score { score_type: dice_result::ResultType::Pair, score: 0, closed: false },
                Score { score_type: dice_result::ResultType::TwoPairs, score: 0, closed: false },
                Score { score_type: dice_result::ResultType::ThreeOfAKind, score: 0, closed: false },
                Score { score_type: dice_result::ResultType::FourOfAKind, score: 0, closed: false },
                Score { score_type: dice_result::ResultType::SmallStraight, score: 0, closed: false },
                Score { score_type: dice_result::ResultType::LargeStraight, score: 0, closed: false },
                Score { score_type: dice_result::ResultType::FullHouse, score: 0, closed: false },
                Score { score_type: dice_result::ResultType::Chance, score: 0, closed: false },
                Score { score_type: dice_result::ResultType::Yatsy, score: 0, closed: false },
            ]
        }
    }

    pub fn get_available_types(&self) -> Vec<dice_result::ResultType> {
        self.scores.iter().filter(|s| !s.scored()).clone().map(|&s| s.score_type).collect()
    }

    pub fn add_result(&mut self, result: dice_result::DiceResult) {
        let available_types = self.get_available_types();
        if !available_types.contains(&result.result_type) {
            return
        }

        let score_index = self.scores.iter().position(|s| s.score_type == result.result_type).unwrap();
        self.scores[score_index] = Score { score_type: result.result_type, score: result.score, closed: false };
    }

    pub fn print_scores(&self) {
        println!("\n--- Score Card ---");
        for score in self.get_scores() {
            println!("{}", score);
        }
        println!("---\nTotal:\t\t{}p", self.get_scores().into_iter().map(|f| f.score).sum::<i32>())
    }

    fn get_scores(&self) -> Vec<Score> {
        self.scores.iter().filter(|s| s.score > 0 || s.closed).cloned().collect()
    }
}