use super::dice_result;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Score {
    score_type:  dice_result::ResultType,
    score: i32,
    striked: bool
}

impl Score {
    pub fn scored(&self) -> bool {
        self.striked || self.score != 0 
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.striked {
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
                Score { score_type: dice_result::ResultType::Ones, score: 0, striked: false },
                Score { score_type: dice_result::ResultType::Twos, score: 0, striked: false },
                Score { score_type: dice_result::ResultType::Threes, score: 0, striked: false },
                Score { score_type: dice_result::ResultType::Fours, score: 0, striked: false },
                Score { score_type: dice_result::ResultType::Fives, score: 0, striked: false },
                Score { score_type: dice_result::ResultType::Sixes, score: 0, striked: false },
                Score { score_type: dice_result::ResultType::Pair, score: 0, striked: false },
                Score { score_type: dice_result::ResultType::TwoPairs, score: 0, striked: false },
                Score { score_type: dice_result::ResultType::ThreeOfAKind, score: 0, striked: false },
                Score { score_type: dice_result::ResultType::FourOfAKind, score: 0, striked: false },
                Score { score_type: dice_result::ResultType::SmallStraight, score: 0, striked: false },
                Score { score_type: dice_result::ResultType::LargeStraight, score: 0, striked: false },
                Score { score_type: dice_result::ResultType::FullHouse, score: 0, striked: false },
                Score { score_type: dice_result::ResultType::Chance, score: 0, striked: false },
                Score { score_type: dice_result::ResultType::Yatsy, score: 0, striked: false },
            ]
        }
    }

    pub fn is_complete(&self) -> bool {
        self.scores.iter().all(|s| s.scored())
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
        self.scores[score_index] = Score { score_type: result.result_type, score: result.score, striked: false };
    }

    pub fn strike(&mut self, result_type: dice_result::ResultType) {
        let score_index = self.scores.iter().position(|s| s.score_type == result_type).unwrap();
        self.scores[score_index].striked = true;
    }

    pub fn print_scores(&self) {
        println!("\n--- Score Card ---");

        let upper_scores = self.get_upper_scores();
        let lower_scores = self.get_lower_scores();

        self.print_upper_scores(&upper_scores);
        self.print_lower_scores(&lower_scores);

        let total = upper_scores.into_iter().map(|f| f.score).sum::<i32>() + 
                       lower_scores.into_iter().map(|f| f.score).sum::<i32>() + 
                       self.get_bonus();
        println!("Total:\t{}p", total);
    }
 
    fn print_upper_scores(&self, scores: &Vec<Score>) {
        if scores.len() == 0 {
            return
        }
        
        let mut sum = 0;
        for score in scores {
            println!("{}", score);
            sum += score.score;
        }
        println!("---\nSum: {}p", sum);

        if scores.into_iter().all(|s| s.scored() && scores.into_iter().count() == 6) {
            println!("Bonus: {}p", self.get_bonus());
        }
        println!("---");
    }

    fn print_lower_scores(&self, scores: &Vec<Score>) {
        if scores.len() == 0 {
            return
        }
        
        for score in scores {
            println!("{}", score);
        }

        println!("---");
    }

    fn get_upper_scores(&self) -> Vec<Score> {
        self.scores.iter().take(6).cloned().collect()
    }

    fn get_lower_scores(&self) -> Vec<Score> {
        self.scores.iter().skip(6).cloned().collect()
    }

    fn get_bonus(&self) -> i32 {
        let upper_sum = self.scores.iter().take(6).map(|s| s.score).sum::<i32>();
        if upper_sum >= 63 {
            50
        } else {
            0
        }
    }
}