use crate::{card::Card, Rule};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Hand {
    initial: [Card; 2],
    third: Option<Card>,
}

impl Hand {
    pub fn is_natural(&self) -> bool {
        if self.third.is_some() {
            return false;
        }
        (self.initial[0].value + self.initial[1].value) % 10 >= 8
    }

    pub fn get_sum(&self) -> u8 {
        let mut sum = self.initial[0].value + self.initial[1].value;
        if let Some(ref third_card) = self.third {
            sum += third_card.value;
        }
        sum % 10
    }

    pub fn is_initial_unsuit_pair(&self) -> bool {
        self.initial[0].value == self.initial[1].value
    }

    pub fn is_initial_suit_pair(&self) -> bool {
        self.initial[0] == self.initial[1]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HandsBet {
    PlayerWin,
    Tie,
    BankerWin,

    PlayerUnsuitPair,
    BankerUnsuitPair,
    PerfectPair,

    PlayerBonus,
    BankerBonus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HandsResult {
    Lose,

    PlayerWin,
    Tie,
    BankerWin,

    PlayerUnsuitPair,
    BankerUnsuitPair,
    PerfectPair(u8), // Param can be 1 or 2.

    PlayerBonus(HandsResultBonus),
    BankerBonus(HandsResultBonus),
}

impl Default for HandsResult {
    fn default() -> Self {
        Self::Lose
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HandsResultBonus {
    NaturalWin,
    NaturalTie,
    UnnaturalBonus(u8), // Param can be in range [4, 9].
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RoundBetResult(HandsResult, i64);

#[derive(Debug, Clone)]
pub struct RoundResult<'a> {
    /// Indicates how much you earn from casino. This will be 0 when player neither
    /// win nor lose, and will be negative when player lose.
    rule: &'a Rule,
    pub total_profit: i64,
    pub details: HashMap<HandsBet, RoundBetResult>,
}

impl<'a> RoundResult<'a> {
    pub fn new(rule: &'a Rule) -> Self {
        Self {
            rule,
            total_profit: 0,
            details: HashMap::new(),
        }
    }

    pub fn calculate_with_hands_and_bet(
        &mut self,
        player: &Hand,
        banker: &Hand,
        bets: &HashMap<HandsBet, i64>,
    ) {
        self.total_profit = 0;
        self.details.clear();
        let payouts = &self.rule.payouts;

        let player_sum = player.get_sum();
        let banker_sum = banker.get_sum();

        fn f(money: i64, payout: f64) -> i64 {
            (money as f64 * payout) as i64
        }

        for (hands_bet, money) in bets {
            let bet_result = match *hands_bet {
                // Main bet 1: Player win
                HandsBet::PlayerWin => match player_sum.cmp(&banker_sum) {
                    Ordering::Less => RoundBetResult(HandsResult::Lose, -money),
                    Ordering::Equal => RoundBetResult(HandsResult::Tie, 0),
                    Ordering::Greater => {
                        RoundBetResult(HandsResult::PlayerWin, f(*money, payouts.player_win))
                    }
                },
                // Main bet 2: Tie
                HandsBet::Tie => {
                    if player_sum == banker_sum {
                        RoundBetResult(HandsResult::Tie, f(*money, payouts.tie))
                    } else {
                        RoundBetResult(HandsResult::Lose, -money)
                    }
                }
                // Main bet 3: Banker win
                HandsBet::BankerWin => match player_sum.cmp(&banker_sum) {
                    Ordering::Less => {
                        RoundBetResult(HandsResult::BankerWin, f(*money, payouts.banker_win))
                    }
                    Ordering::Equal => RoundBetResult(HandsResult::Tie, 0),
                    Ordering::Greater => RoundBetResult(HandsResult::Lose, -money),
                },

                // Side bet 1: Player pair
                HandsBet::PlayerUnsuitPair => {
                    if player.is_initial_unsuit_pair() {
                        RoundBetResult(
                            HandsResult::PlayerUnsuitPair,
                            f(*money, payouts.unsuit_pair),
                        )
                    } else {
                        RoundBetResult(HandsResult::Lose, -money)
                    }
                }
                // Side bet 2: Banker pair
                HandsBet::BankerUnsuitPair => {
                    if banker.is_initial_unsuit_pair() {
                        RoundBetResult(
                            HandsResult::BankerUnsuitPair,
                            f(*money, payouts.unsuit_pair),
                        )
                    } else {
                        RoundBetResult(HandsResult::Lose, -money)
                    }
                }
                // Side bet 3: Perfect pair
                HandsBet::PerfectPair => {
                    let player_perfect = if player.is_initial_suit_pair() { 1 } else { 0 };
                    let banker_perfect = if banker.is_initial_suit_pair() { 1 } else { 0 };
                    match player_perfect + banker_perfect {
                        0 => RoundBetResult(HandsResult::Lose, -money),
                        pairs @ 1..=2 => RoundBetResult(
                            HandsResult::PerfectPair(pairs),
                            f(*money, payouts.perfect_pair[(pairs - 1) as usize]),
                        ),
                        _ => unreachable!(),
                    }
                }
                // Side bet 4: Player bonus
                HandsBet::PlayerBonus => {
                    if player_sum < banker_sum {
                        RoundBetResult(HandsResult::Lose, -money)
                    } else if player.is_natural() {
                        if player_sum > banker_sum {
                            RoundBetResult(
                                HandsResult::PlayerBonus(HandsResultBonus::NaturalWin),
                                f(*money, payouts.bonus_natural_win),
                            )
                        } else {
                            RoundBetResult(
                                HandsResult::PlayerBonus(HandsResultBonus::NaturalTie),
                                f(*money, payouts.bonus_natural_tie),
                            )
                        }
                    } else {
                        match player_sum - banker_sum {
                            ..=3 => RoundBetResult(HandsResult::Lose, -money),
                            delta @ 4..=9 => RoundBetResult(
                                HandsResult::PlayerBonus(HandsResultBonus::UnnaturalBonus(delta)),
                                f(*money, payouts.bonus_unnatural[(delta - 4) as usize]),
                            ),
                            _ => unreachable!(),
                        }
                    }
                }
                // Side bet 5: Banker bonus
                HandsBet::BankerBonus => {
                    if player_sum > banker_sum {
                        RoundBetResult(HandsResult::Lose, -money)
                    } else if banker.is_natural() {
                        if player_sum < banker_sum {
                            RoundBetResult(
                                HandsResult::BankerBonus(HandsResultBonus::NaturalWin),
                                f(*money, payouts.bonus_natural_win),
                            )
                        } else {
                            RoundBetResult(
                                HandsResult::BankerBonus(HandsResultBonus::NaturalTie),
                                f(*money, payouts.bonus_natural_tie),
                            )
                        }
                    } else {
                        match banker_sum - player_sum {
                            ..=3 => RoundBetResult(HandsResult::Lose, -money),
                            delta @ 4..=9 => RoundBetResult(
                                HandsResult::BankerBonus(HandsResultBonus::UnnaturalBonus(delta)),
                                f(*money, payouts.bonus_unnatural[(delta - 4) as usize]),
                            ),
                            _ => unreachable!(),
                        }
                    }
                }
            };

            self.details.insert(*hands_bet, bet_result);
            self.total_profit += bet_result.1;
        }
    }
}
