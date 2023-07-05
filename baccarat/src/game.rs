mod hands;

pub use hands::{Hand, HandsBet, HandsResult, HandsResultBonus, RoundResult};
use std::collections::HashMap;

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

    pub fn start_game_loop(&mut self) {}

    pub fn get_bcr_value_count(&self) -> &[u32; 10] {
        self.counter.get_bcr_value_count()
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
    fn discard_cards(&mut self, cards: u32);
}

pub trait GamblerProvider {
    fn place_bet(&mut self, solution: &Solution) -> &HashMap<HandsBet, i64>;
    fn on_new_shoe(&mut self);
    fn on_discard(&mut self, cards: u32);
    fn on_round_start(&mut self);
    fn on_round_end(&mut self, round_result: &RoundResult);
    fn on_cut_card_reached(&mut self, number_of_dealt_cards: u32, cut_card_proportion: f64);
}
