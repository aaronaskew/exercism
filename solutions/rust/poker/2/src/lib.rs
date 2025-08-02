/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 1 {
        return hands.to_vec();
    }

    let mut hands: Vec<Hand> = hands.iter().map(|hand| Hand::new(hand)).collect();

    hands.sort();
    hands.reverse();

    let winning_hand = &hands[0];

    hands
        .iter()
        .filter_map(|hand| {
            if hand.rank == winning_hand.rank {
                Some(hand.orig)
            } else {
                None
            }
        })
        .collect()
}

#[derive(Debug)]
pub struct Hand<'a> {
    pub orig: &'a str,
    pub rank: HandRank,
}

impl<'a> Hand<'a> {
    pub fn new(orig: &'a str) -> Self {
        let mut cards = orig
            .split(" ")
            .map(|card| {
                let card_chars = card.chars().collect::<Vec<_>>();
                let rank = match card_chars[0] {
                    '2' => CardRank::Two,
                    '3' => CardRank::Three,
                    '4' => CardRank::Four,
                    '5' => CardRank::Five,
                    '6' => CardRank::Six,
                    '7' => CardRank::Seven,
                    '8' => CardRank::Eight,
                    '9' => CardRank::Nine,
                    '1' => CardRank::Ten,
                    'J' => CardRank::Jack,
                    'Q' => CardRank::Queen,
                    'K' => CardRank::King,
                    'A' => CardRank::AceHigh,
                    _ => panic!(),
                };
                let suit = match card_chars[card_chars.len() - 1] {
                    'H' => Suit::Hearts,
                    'S' => Suit::Spades,
                    'D' => Suit::Diamonds,
                    'C' => Suit::Clubs,
                    _ => panic!(),
                };

                Card { rank, suit }
            })
            .collect::<Vec<_>>();

        cards.sort();
        cards.reverse();

        // Straight-Flush
        if is_flush(&cards) && is_straight(&mut cards) {
            return Self {
                orig,

                rank: HandRank::StraightFlush {
                    high_card: cards[0].rank,
                },
            };
        }

        // Four-of-a-Kind
        let ranks = card_ranks(&cards);
        if ranks[0] == ranks[1] && ranks[1] == ranks[2] && ranks[2] == ranks[3] {
            return Self {
                orig,

                rank: HandRank::FourOfAKind {
                    four_of_a_kind_rank: ranks[0],
                    high_card: ranks[4],
                },
            };
        } else if ranks[1] == ranks[2] && ranks[2] == ranks[3] && ranks[3] == ranks[4] {
            return Self {
                orig,

                rank: HandRank::FourOfAKind {
                    four_of_a_kind_rank: ranks[4],
                    high_card: ranks[0],
                },
            };
        }

        // Full House
        let ranks = card_ranks(&cards);
        if ranks[0] == ranks[1] && ranks[1] == ranks[2] && ranks[3] == ranks[4] {
            return Self {
                orig,

                rank: HandRank::FullHouse {
                    three_of_a_kind_rank: ranks[0],
                    pair_rank: ranks[3],
                },
            };
        } else if ranks[0] == ranks[1] && ranks[2] == ranks[3] && ranks[3] == ranks[4] {
            return Self {
                orig,

                rank: HandRank::FullHouse {
                    three_of_a_kind_rank: ranks[2],
                    pair_rank: ranks[0],
                },
            };
        }

        // Flush
        if is_flush(&cards) {
            return Self {
                orig,

                rank: HandRank::Flush {
                    cards: card_ranks(&cards),
                },
            };
        }

        // Straight
        if is_straight(&mut cards) {
            return Self {
                orig,

                rank: HandRank::Straight {
                    high_card: cards[0].rank,
                },
            };
        }

        // Three-of-a-Kind
        let ranks = card_ranks(&cards);
        if ranks[0] == ranks[1] && ranks[1] == ranks[2] {
            return Self {
                orig,

                rank: HandRank::ThreeOfAKind {
                    three_of_a_kind_rank: ranks[0],
                    remainder: ranks[3..].to_vec(),
                },
            };
        } else if ranks[1] == ranks[2] && ranks[2] == ranks[3] {
            return Self {
                orig,

                rank: HandRank::ThreeOfAKind {
                    three_of_a_kind_rank: ranks[1],
                    remainder: vec![ranks[0], ranks[4]],
                },
            };
        } else if ranks[2] == ranks[3] && ranks[3] == ranks[4] {
            return Self {
                orig,

                rank: HandRank::ThreeOfAKind {
                    three_of_a_kind_rank: ranks[2],
                    remainder: ranks[..=1].to_vec(),
                },
            };
        }

        // Two Pair
        let ranks = card_ranks(&cards);
        if ranks[0] == ranks[1] && ranks[2] == ranks[3] {
            return Self {
                orig,

                rank: HandRank::TwoPair {
                    high_pair_rank: ranks[0],
                    low_pair_rank: ranks[2],
                    high_card: ranks[4],
                },
            };
        } else if ranks[1] == ranks[2] && ranks[3] == ranks[4] {
            return Self {
                orig,

                rank: HandRank::TwoPair {
                    high_pair_rank: ranks[1],
                    low_pair_rank: ranks[3],
                    high_card: ranks[0],
                },
            };
        } else if ranks[0] == ranks[1] && ranks[3] == ranks[4] {
            return Self {
                orig,

                rank: HandRank::TwoPair {
                    high_pair_rank: ranks[0],
                    low_pair_rank: ranks[3],
                    high_card: ranks[2],
                },
            };
        }

        // Pair
        let ranks = card_ranks(&cards);
        if ranks[0] == ranks[1] {
            return Self {
                orig,

                rank: HandRank::Pair {
                    pair_rank: ranks[0],
                    remainder: ranks[1..].to_vec(),
                },
            };
        } else if ranks[1] == ranks[2] {
            return Self {
                orig,

                rank: HandRank::Pair {
                    pair_rank: ranks[1],
                    remainder: vec![ranks[0], ranks[3], ranks[4]],
                },
            };
        } else if ranks[2] == ranks[3] {
            return Self {
                orig,

                rank: HandRank::Pair {
                    pair_rank: ranks[2],
                    remainder: vec![ranks[0], ranks[1], ranks[4]],
                },
            };
        } else if ranks[3] == ranks[4] {
            return Self {
                orig,

                rank: HandRank::Pair {
                    pair_rank: ranks[3],
                    remainder: ranks[..=2].to_vec(),
                },
            };
        }

        // High Card
        Self {
            orig,

            rank: HandRank::HighCard {
                cards: card_ranks(&cards),
            },
        }
    }
}

fn card_ranks(cards: &[Card]) -> Vec<CardRank> {
    cards.iter().map(|card| card.rank).collect()
}

fn is_straight(cards: &mut [Card]) -> bool {
    assert!(cards.len() == 5);

    // check for A->5 straight
    let ranks = cards.iter().map(|card| card.rank).collect::<Vec<_>>();
    if ranks.contains(&CardRank::AceHigh)
        && ranks.contains(&CardRank::Two)
        && ranks.contains(&CardRank::Three)
        && ranks.contains(&CardRank::Four)
        && ranks.contains(&CardRank::Five)
    {
        assert!(cards[0].rank == CardRank::AceHigh);
        cards[0] = Card {
            rank: CardRank::AceLow,
            suit: cards[0].suit.clone(),
        };
        cards.sort();
        cards.reverse();
    }

    let mut high_card = cards[0].rank;

    for card in cards.iter().skip(1) {
        if card.rank as u8 != high_card as u8 - 1 {
            return false;
        } else {
            high_card = card.rank;
        }
    }

    true
}

fn is_flush(cards: &[Card]) -> bool {
    assert!(cards.len() == 5);

    let suit = &cards[0].suit;

    cards.iter().all(|card| card.suit == *suit)
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.orig == other.orig
    }
}

impl<'a> Eq for Hand<'a> {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardRank {
    AceLow = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    AceHigh = 14,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

#[derive(Debug, Clone)]
struct Card {
    rank: CardRank,
    suit: Suit,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Eq for Card {}

#[derive(Debug)]
pub enum HandRank {
    HighCard {
        cards: Vec<CardRank>,
    },
    Pair {
        pair_rank: CardRank,
        remainder: Vec<CardRank>,
    },
    TwoPair {
        high_pair_rank: CardRank,
        low_pair_rank: CardRank,
        high_card: CardRank,
    },
    ThreeOfAKind {
        three_of_a_kind_rank: CardRank,
        remainder: Vec<CardRank>,
    },
    Straight {
        high_card: CardRank,
    },
    Flush {
        cards: Vec<CardRank>,
    },
    FullHouse {
        three_of_a_kind_rank: CardRank,
        pair_rank: CardRank,
    },
    FourOfAKind {
        four_of_a_kind_rank: CardRank,
        high_card: CardRank,
    },
    StraightFlush {
        high_card: CardRank,
    },
}

impl Ord for HandRank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (HandRank::HighCard { cards }, HandRank::HighCard { cards: other_cards }) => {
                for i in 0..cards.len() {
                    match cards[i].cmp(&other_cards[i]) {
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => continue,
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                    }
                }

                std::cmp::Ordering::Equal
            }
            (HandRank::HighCard { .. }, _) => std::cmp::Ordering::Less,
            (HandRank::Pair { .. }, HandRank::HighCard { .. }) => std::cmp::Ordering::Greater,
            (
                HandRank::Pair {
                    pair_rank,
                    remainder,
                },
                HandRank::Pair {
                    pair_rank: other_pair_rank,
                    remainder: other_remainder,
                },
            ) => match pair_rank.cmp(other_pair_rank) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => {
                    for i in 0..remainder.len() {
                        match remainder[i].cmp(&other_remainder[i]) {
                            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                            std::cmp::Ordering::Equal => continue,
                            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        }
                    }

                    std::cmp::Ordering::Equal
                }
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            },
            (HandRank::Pair { .. }, _) => std::cmp::Ordering::Less,
            (HandRank::TwoPair { .. }, HandRank::HighCard { .. } | HandRank::Pair { .. }) => {
                std::cmp::Ordering::Greater
            }
            (
                HandRank::TwoPair {
                    high_pair_rank,
                    low_pair_rank,
                    high_card,
                },
                HandRank::TwoPair {
                    high_pair_rank: other_high_pair_rank,
                    low_pair_rank: other_low_pair_rank,
                    high_card: other_high_card,
                },
            ) => match high_pair_rank.cmp(other_high_pair_rank) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => match low_pair_rank.cmp(other_low_pair_rank) {
                    std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                    std::cmp::Ordering::Equal => high_card.cmp(other_high_card),
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                },
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            },
            (HandRank::TwoPair { .. }, _) => std::cmp::Ordering::Less,
            (
                HandRank::ThreeOfAKind { .. },
                HandRank::HighCard { .. } | HandRank::Pair { .. } | HandRank::TwoPair { .. },
            ) => std::cmp::Ordering::Greater,
            (
                HandRank::ThreeOfAKind {
                    three_of_a_kind_rank,
                    remainder,
                },
                HandRank::ThreeOfAKind {
                    three_of_a_kind_rank: other_three_of_a_kind_rank,
                    remainder: other_remainder,
                },
            ) => match three_of_a_kind_rank.cmp(other_three_of_a_kind_rank) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => {
                    for i in 0..remainder.len() {
                        match remainder[i].cmp(&other_remainder[i]) {
                            std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                            std::cmp::Ordering::Equal => continue,
                            std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        }
                    }
                    std::cmp::Ordering::Equal
                }
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            },
            (HandRank::ThreeOfAKind { .. }, _) => std::cmp::Ordering::Less,
            (
                HandRank::Straight { .. },
                HandRank::HighCard { .. }
                | HandRank::Pair { .. }
                | HandRank::TwoPair { .. }
                | HandRank::ThreeOfAKind { .. },
            ) => std::cmp::Ordering::Greater,
            (
                HandRank::Straight { high_card },
                HandRank::Straight {
                    high_card: other_high_card,
                },
            ) => high_card.cmp(other_high_card),
            (HandRank::Straight { .. }, _) => std::cmp::Ordering::Less,
            (
                HandRank::Flush { .. },
                HandRank::HighCard { .. }
                | HandRank::Pair { .. }
                | HandRank::TwoPair { .. }
                | HandRank::ThreeOfAKind { .. }
                | HandRank::Straight { .. },
            ) => std::cmp::Ordering::Greater,
            (HandRank::Flush { cards }, HandRank::Flush { cards: other_cards }) => {
                for i in 0..cards.len() {
                    match cards[i].cmp(&other_cards[i]) {
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => continue,
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                    }
                }
                std::cmp::Ordering::Equal
            }
            (HandRank::Flush { .. }, _) => std::cmp::Ordering::Less,
            (
                HandRank::FullHouse { .. },
                HandRank::HighCard { .. }
                | HandRank::Pair { .. }
                | HandRank::TwoPair { .. }
                | HandRank::ThreeOfAKind { .. }
                | HandRank::Straight { .. }
                | HandRank::Flush { .. },
            ) => std::cmp::Ordering::Greater,
            (
                HandRank::FullHouse {
                    three_of_a_kind_rank,
                    pair_rank,
                },
                HandRank::FullHouse {
                    three_of_a_kind_rank: other_three_of_a_kind_rank,
                    pair_rank: other_pair_rank,
                },
            ) => match three_of_a_kind_rank.cmp(other_three_of_a_kind_rank) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => match pair_rank.cmp(other_pair_rank) {
                    std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                    std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                },
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            },
            (HandRank::FullHouse { .. }, _) => std::cmp::Ordering::Less,
            (HandRank::FourOfAKind { .. }, HandRank::StraightFlush { .. }) => {
                std::cmp::Ordering::Less
            }
            (
                HandRank::FourOfAKind {
                    four_of_a_kind_rank,
                    high_card,
                },
                HandRank::FourOfAKind {
                    four_of_a_kind_rank: other_four_of_a_kind_rank,
                    high_card: other_high_card,
                },
            ) => match four_of_a_kind_rank.cmp(other_four_of_a_kind_rank) {
                std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal => high_card.cmp(other_high_card),
                std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            },
            (HandRank::FourOfAKind { .. }, _) => std::cmp::Ordering::Greater,
            (
                HandRank::StraightFlush { high_card },
                HandRank::StraightFlush {
                    high_card: other_high_card,
                },
            ) => high_card.cmp(other_high_card),
            (HandRank::StraightFlush { .. }, _) => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandRank {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::HighCard { cards: l_cards }, Self::HighCard { cards: r_cards }) => {
                l_cards == r_cards
            }
            (
                Self::Pair {
                    pair_rank: l_pair_rank,
                    remainder: l_remainder,
                },
                Self::Pair {
                    pair_rank: r_pair_rank,
                    remainder: r_remainder,
                },
            ) => l_pair_rank == r_pair_rank && l_remainder == r_remainder,
            (
                Self::TwoPair {
                    high_pair_rank: l_high_pair_rank,
                    low_pair_rank: l_low_pair_rank,
                    high_card: l_high_card,
                },
                Self::TwoPair {
                    high_pair_rank: r_high_pair_rank,
                    low_pair_rank: r_low_pair_rank,
                    high_card: r_high_card,
                },
            ) => {
                l_high_pair_rank == r_high_pair_rank
                    && l_low_pair_rank == r_low_pair_rank
                    && l_high_card == r_high_card
            }
            (
                Self::ThreeOfAKind {
                    three_of_a_kind_rank: l_three_of_a_kind_rank,
                    remainder: l_remainder,
                },
                Self::ThreeOfAKind {
                    three_of_a_kind_rank: r_three_of_a_kind_rank,
                    remainder: r_remainder,
                },
            ) => l_three_of_a_kind_rank == r_three_of_a_kind_rank && l_remainder == r_remainder,
            (
                Self::Straight {
                    high_card: l_high_card,
                },
                Self::Straight {
                    high_card: r_high_card,
                },
            ) => l_high_card == r_high_card,
            (Self::Flush { cards: l_cards }, Self::Flush { cards: r_cards }) => l_cards == r_cards,
            (
                Self::FullHouse {
                    three_of_a_kind_rank: l_three_of_a_kind_rank,
                    pair_rank: l_pair_rank,
                },
                Self::FullHouse {
                    three_of_a_kind_rank: r_three_of_a_kind_rank,
                    pair_rank: r_pair_rank,
                },
            ) => l_three_of_a_kind_rank == r_three_of_a_kind_rank && l_pair_rank == r_pair_rank,
            (
                Self::FourOfAKind {
                    four_of_a_kind_rank: l_four_of_a_kind_rank,
                    high_card: l_high_card,
                },
                Self::FourOfAKind {
                    four_of_a_kind_rank: r_four_of_a_kind_rank,
                    high_card: r_high_card,
                },
            ) => l_four_of_a_kind_rank == r_four_of_a_kind_rank && l_high_card == r_high_card,
            (
                Self::StraightFlush {
                    high_card: l_high_card,
                },
                Self::StraightFlush {
                    high_card: r_high_card,
                },
            ) => l_high_card == r_high_card,
            _ => false,
        }
    }
}

impl Eq for HandRank {}
