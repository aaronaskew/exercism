#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    let mut dice = dice;
    dice.sort();
    match category {
        Category::Ones => dice
            .iter()
            .fold(0, |acc, &d| acc + if d == 1 { d } else { 0 }),
        Category::Twos => dice
            .iter()
            .fold(0, |acc, &d| acc + if d == 2 { d } else { 0 }),
        Category::Threes => dice
            .iter()
            .fold(0, |acc, &d| acc + if d == 3 { d } else { 0 }),
        Category::Fours => dice
            .iter()
            .fold(0, |acc, &d| acc + if d == 4 { d } else { 0 }),
        Category::Fives => dice
            .iter()
            .fold(0, |acc, &d| acc + if d == 5 { d } else { 0 }),
        Category::Sixes => dice
            .iter()
            .fold(0, |acc, &d| acc + if d == 6 { d } else { 0 }),
        Category::FullHouse => {
            if (dice[2] == dice[3] || dice[1] == dice[2])
                && dice[0] != dice[4]
                && dice[3] == dice[4]
                && dice[0] == dice[1]
            {
                dice.iter().sum::<u8>()
            } else {
                0
            }
        }
        Category::FourOfAKind => {
            if (dice[3] == dice[4] || dice[0] == dice[1])
                && dice[2] == dice[3]
                && dice[1] == dice[2]
            {
                4 * dice[2]
            } else {
                0
            }
        }
        Category::LittleStraight => {
            if dice == [1, 2, 3, 4, 5] {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            if dice == [2, 3, 4, 5, 6] {
                30
            } else {
                0
            }
        }
        Category::Choice => dice.iter().sum::<u8>(),
        Category::Yacht => {
            if dice.iter().all(|&x| x == dice[0]) {
                50
            } else {
                0
            }
        }
    }
}
