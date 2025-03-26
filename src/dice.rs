#[allow(dead_code)]

use rand::Rng;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt;

#[derive(Clone)]
pub struct DiceRoll {
    pub dice: [i32; 5],
}

impl Display for DiceRoll {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}, {}]", self.dice[0], self.dice[1], self.dice[2], self.dice[3], self.dice[4])
    }
}

impl DiceRoll {
    pub fn new() -> DiceRoll {
        DiceRoll {
            dice: [
                roll_dice(),
                roll_dice(),
                roll_dice(),
                roll_dice(),
                roll_dice(),
            ],
        }
    }

    pub fn reroll(&mut self, indices: Vec::<usize>) -> DiceRoll {
        let mut new_dice = DiceRoll { dice: self.dice };
        for i in indices {
            if (0..5).contains(&i) {
                new_dice.dice[i] = roll_dice();
            }
        }
        new_dice
    }

    pub fn ones(&self) -> i32 {
        self.get_value_sum(1)
    }

    pub fn twos(&self) -> i32 {
        self.get_value_sum(2)
    }

    pub fn threes(&self) -> i32 {
        self.get_value_sum(3)
    }

    pub fn fours(&self) -> i32 {
        self.get_value_sum(4)
    }

    pub fn fives(&self) -> i32 {
        self.get_value_sum(5)
    }

    pub fn sixes(&self) -> i32 {
        self.get_value_sum(6)
    }

    pub fn pair(&self) -> i32 {
        for i in (1..=6).rev() {
            if self.get_count_for_value(i) >= 2 {
                return i * 2;
            }
        }
        0
    }

    pub fn two_pairs(&self) -> i32 {
        let mut pairs = vec![];
        for i in 1..=6 {
            if self.get_count_for_value(i) >= 2 {
                pairs.push(i);
            }
        }

        match pairs.len() {
            2 => (pairs[0] + pairs[1]) * 2,
            _ => 0
        }
    }

    pub fn three_of_a_kind(&self) -> i32 {
        self.get_value_for_count(3) * 3
    }

    pub fn four_of_a_kind(&self) -> i32 {
        self.get_value_for_count(4) * 4
    }

    pub fn small_straight(&self) -> i32 {
        let mut dice = self.dice;
        dice.sort();
        match dice {
            [1, 2, 3, 4, 5] => 15,
            _ => 0
        }
    }

    pub fn large_straight(&self) -> i32 {
        let mut dice = self.dice;
        dice.sort();
        match dice {
            [2, 3, 4, 5, 6] => 20,
            _ => 0
        }
    }

    pub fn full_house(&self) -> i32 {
        let mut triple = 0;
        let mut double = 0;
        for i in 1..7 {
            if self.get_count_for_value(i) == 3 {
                triple = i;
            } else if self.get_count_for_value(i) == 2 {
                double = i;
            }
        }
        if triple != 0 && double != 0 {
            return triple * 3 + double * 2;
        }
        0
    }

    pub fn chance(&self) -> i32 {
        self.dice.iter().sum()
    }

    pub fn yatsy(&self) -> i32 {
        if self.dice.iter().all(|&x| x == self.dice[0]) {
            return 50;
        }
        0
    }

    fn get_value_sum(&self, value: i32) -> i32 {
        self.dice.iter().filter(|&&x| x == value).sum()
    }

    fn get_count_for_value(&self, value: i32) -> i32 {
        self.dice.iter().filter(|&&x| x == value).count() as i32
    }

    fn get_value_for_count(&self, count: i32) -> i32 {
        for i in (1..7).rev() {
            if self.get_count_for_value(i) >= count {
                return i;
            }
        }
        0
    }
}

fn roll_dice() -> i32 {
    rand::thread_rng().gen_range(1..7)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_ones() {
        let roll = DiceRoll {
            dice: [1, 1, 2, 3, 4],
        };
        assert_eq!(roll.ones(), 2);
    }

    #[test]
    fn test_twos() {
        let roll = DiceRoll {
            dice: [2, 2, 3, 4, 5],
        };
        assert_eq!(roll.twos(), 4);
    }

    #[test]
    fn test_threes() {
        let roll = DiceRoll {
            dice: [3, 3, 3, 4, 5],
        };
        assert_eq!(roll.threes(), 9);
    }

    #[test]
    fn test_fours() {
        let roll = DiceRoll {
            dice: [4, 4, 4, 4, 5],
        };
        assert_eq!(roll.fours(), 16);
    }

    #[test]
    fn test_fives() {
        let roll = DiceRoll {
            dice: [5, 5, 5, 5, 5],
        };
        assert_eq!(roll.fives(), 25);
    }

    #[test]
    fn test_sixes() {
        let roll = DiceRoll {
            dice: [6, 6, 6, 6, 6],
        };
        assert_eq!(roll.sixes(), 30);
    }

    #[test]
    fn test_pair() {
        let roll = DiceRoll {
            dice: [6, 6, 6, 6, 6],
        };
        assert_eq!(roll.pair(), 12);

        let roll = DiceRoll {
            dice: [1, 2, 3, 4, 5],
        };
        assert_eq!(roll.pair(), 0);

        let roll = DiceRoll {
            dice: [1, 2, 3, 4, 4],
        };
        assert_eq!(roll.pair(), 8);
    }

    #[test]
    fn test_two_pairs() {
        let roll = DiceRoll {
            dice: [6, 6, 6, 6, 6],
        };
        assert_eq!(roll.two_pairs(), 0);

        let roll = DiceRoll {
            dice: [1, 3, 3, 4, 1],
        };
        assert_eq!(roll.two_pairs(), 8);

        let roll = DiceRoll {
            dice: [2, 2, 4, 4, 4],
        };
        assert_eq!(roll.two_pairs(), 12);
    }

    #[test]
    fn test_three_of_a_kind() {
        let roll = DiceRoll {
            dice: [2, 2, 4, 2, 2],
        };
        assert_eq!(roll.three_of_a_kind(), 6);

        let roll = DiceRoll {
            dice: [1, 3, 3, 4, 1],
        };
        assert_eq!(roll.three_of_a_kind(), 0);

        let roll = DiceRoll {
            dice: [5, 2, 5, 5, 4],
        };
        assert_eq!(roll.three_of_a_kind(), 15);
    }

    #[test]
    fn test_four_of_a_kind() {
        let roll = DiceRoll {
            dice: [2, 2, 4, 2, 2],
        };
        assert_eq!(roll.four_of_a_kind(), 8);

        let roll = DiceRoll {
            dice: [3, 3, 3, 4, 1],
        };
        assert_eq!(roll.four_of_a_kind(), 0);

        let roll = DiceRoll {
            dice: [5, 5, 5, 5, 5],
        };
        assert_eq!(roll.four_of_a_kind(), 20);
    }

    #[test]
    fn test_small_straight() {
        let roll = DiceRoll {
            dice: [1, 4, 5, 2, 3],
        };
        assert_eq!(roll.small_straight(), 15);

        let roll = DiceRoll {
            dice: [1, 5, 5, 2, 3],
        };
        assert_eq!(roll.small_straight(), 0);
    }

    #[test]
    fn test_large_straight() {
        let roll = DiceRoll {
            dice: [6, 4, 5, 3, 2],
        };
        assert_eq!(roll.large_straight(), 20);

        let roll = DiceRoll {
            dice: [1, 5, 5, 2, 3],
        };
        assert_eq!(roll.large_straight(), 0);
    }

    #[test]
    fn test_full_house() {
        let roll = DiceRoll {
            dice: [6, 6, 3, 3, 3],
        };
        assert_eq!(roll.full_house(), 21);

        let roll = DiceRoll {
            dice: [1, 5, 5, 2, 3],
        };
        assert_eq!(roll.full_house(), 0);
    }

    #[test]
    fn test_chance() {
        let roll = DiceRoll {
            dice: [1, 2, 3, 4, 5],
        };
        assert_eq!(roll.chance(), 15);
    }

    #[test]
    fn test_yatsy() {
        let roll = DiceRoll {
            dice: [1, 1, 1, 1, 1],
        };
        assert_eq!(roll.yatsy(), 50);

        let roll = DiceRoll {
            dice: [6, 4, 5, 3, 2],
        };
        assert_eq!(roll.yatsy(), 0);
    }
}
