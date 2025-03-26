use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;
use super::dice;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ResultType {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FourOfAKind,
    SmallStraight,
    LargeStraight,
    FullHouse,
    Chance,
    Yatsy
}

impl Display for ResultType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let name = match self {
            ResultType::Ones => "Ones",
            ResultType::Twos => "Twos",
            ResultType::Threes => "Threes",
            ResultType::Fours => "Fours",
            ResultType::Fives => "Fives",
            ResultType::Sixes => "Sixes",
            ResultType::Pair => "Pair",
            ResultType::TwoPairs => "Two Pairs",
            ResultType::ThreeOfAKind => "Three of a Kind",
            ResultType::FourOfAKind => "Four of a Kind",
            ResultType::SmallStraight => "Small Straight",
            ResultType::LargeStraight => "Large Straight",
            ResultType::FullHouse => "Full House",
            ResultType::Chance => "Chance",
            ResultType::Yatsy => "Yatsy",
        };
        write!(f, "{}", name)
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct DiceResult {
    pub result_type: ResultType,
    pub score: i32
}

impl Display for DiceResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}p\t{}", self.score, self.result_type)
    }
}

pub fn get_results(dice: dice::DiceRoll) -> Vec<DiceResult> {
    let mut results = Vec::<DiceResult>::new();

        let ones = dice.ones();
        if ones > 0 {
            results.push(DiceResult { result_type: ResultType::Ones, score: ones });
        }

        let twos = dice.twos();
        if twos > 0 {
            results.push(DiceResult { result_type: ResultType::Twos, score: twos });
        }
        
        let threes = dice.threes();
        if threes > 0 {
            results.push(DiceResult { result_type: ResultType::Threes, score: threes });
        }

        let fours = dice.fours();
        if fours > 0 {
            results.push(DiceResult { result_type: ResultType::Fours, score: fours });
        }

        let fives = dice.fives();
        if fives > 0 {
            results.push(DiceResult { result_type: ResultType::Fives, score: fives });
        }

        let sixes = dice.sixes();
        if sixes > 0 {
            results.push(DiceResult { result_type: ResultType::Sixes, score: sixes });
        }

        let pair = dice.pair();
        if pair > 0 {
            results.push(DiceResult { result_type: ResultType::Pair, score: pair });
        }

        let two_pairs = dice.two_pairs();
        if two_pairs > 0 {
            results.push(DiceResult { result_type: ResultType::TwoPairs, score: two_pairs });
        }

        let three_of_a_kind = dice.three_of_a_kind();
        if three_of_a_kind > 0 {
            results.push(DiceResult { result_type: ResultType::ThreeOfAKind, score: three_of_a_kind });
        }

        let four_of_a_kind = dice.four_of_a_kind();
        if four_of_a_kind > 0 {
            results.push(DiceResult { result_type: ResultType::FourOfAKind, score: four_of_a_kind });
        }

        let small_straight = dice.small_straight();
        if small_straight > 0 {
            results.push(DiceResult { result_type: ResultType::SmallStraight, score: small_straight });
        }

        let large_straight = dice.large_straight();
        if large_straight > 0 {
            results.push(DiceResult { result_type: ResultType::LargeStraight, score: large_straight });
        }

        let full_house = dice.full_house();
        if full_house > 0 {
            results.push(DiceResult { result_type: ResultType::FullHouse, score: full_house });
        }

        let chance = dice.chance();
        if chance > 0 {
            results.push(DiceResult { result_type: ResultType::Chance, score: chance });
        }

        let yatsy = dice.yatsy();
        if yatsy > 0 {
            results.push(DiceResult { result_type: ResultType::Yatsy, score: yatsy });
        }

        results
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_result() {
        let results = get_results(dice::DiceRoll { dice: [1, 2, 3, 4, 5] });
        assert_eq!(results, vec![
            DiceResult { result_type: ResultType::Ones, score: 1 },
            DiceResult { result_type: ResultType::Twos, score: 2 },
            DiceResult { result_type: ResultType::Threes, score: 3 },
            DiceResult { result_type: ResultType::Fours, score: 4 },
            DiceResult { result_type: ResultType::Fives, score: 5 },
            DiceResult { result_type: ResultType::SmallStraight, score: 15 },
            DiceResult { result_type: ResultType::Chance, score: 15 },
        ]);

        let results = get_results(dice::DiceRoll { dice: [2, 2, 2, 3, 3] });
        assert_eq!(results, vec![
            DiceResult { result_type: ResultType::Twos, score: 6 },
            DiceResult { result_type: ResultType::Threes, score: 6 },
            DiceResult { result_type: ResultType::Pair, score: 6 },
            DiceResult { result_type: ResultType::ThreeOfAKind, score: 6 },
            DiceResult { result_type: ResultType::FullHouse, score: 12 },
            DiceResult { result_type: ResultType::Chance, score: 12 },
        ]);

        let results = get_results(dice::DiceRoll { dice: [1, 1, 1, 1, 1] });
        assert_eq!(results, vec![
            DiceResult { result_type: ResultType::Ones, score: 5 },
            DiceResult { result_type: ResultType::Pair, score: 2 },
            DiceResult { result_type: ResultType::ThreeOfAKind, score: 3 },
            DiceResult { result_type: ResultType::FourOfAKind, score: 4 },
            DiceResult { result_type: ResultType::Chance, score: 5 },
            DiceResult { result_type: ResultType::Yatsy, score: 50 },
        ]);
    }
}