use crate::{
    calculation::{Counter, Solution},
    card::Card,
    Rule,
};

#[derive(Debug, Clone)]
pub struct Game<T: DealerProvider, U: GamblerProvider> {
    rule: Rule,
    counter: Counter,

    dealer: T,
    gambler: U,
}

impl<T: DealerProvider, U: GamblerProvider> Game<T, U> {
    pub fn new(rule: Rule, dealer: T, gambler: U) -> Self {
        let number_of_decks = rule.number_of_decks;
        Game {
            rule,
            counter: Counter::new(number_of_decks),
            dealer,
            gambler,
        }
    }

    pub fn get_value_count(&self) -> &[u32; 13] {
        self.counter.get_value_count()
    }

    pub fn get_card_count(&self) -> &[u32; 52] {
        self.counter.get_card_count()
    }
}

pub trait DealerProvider {
    fn deal_card(&mut self) -> Card;
}

pub trait GamblerProvider {
    fn place_bet(&mut self, solution: &Solution) -> i64;
    /// This is the method to call when winning money. Money here contains
    /// gambler's betting money. In another word, if you bet 100 on "Player Win"
    /// (whose payout is 1:1) and the player wins, you win 200. If banker wins,
    /// you win 0. If it's a tie, you win 100.
    fn win_money(&mut self, money: i64);
}
