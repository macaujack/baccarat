use crate::{card::Card, Rule};

pub struct Solver<'a> {
    rule: &'a Rule,
    solution: Solution,
    counter: Counter,
}

impl<'a> Solver<'a> {
    pub fn new(rule: &'a Rule) -> Self {
        Self {
            rule,
            solution: Default::default(),
            counter: Counter::new(0),
        }
    }

    /// Note that this is NOT thread-safe.
    pub fn solve(&mut self, counter: &Counter) -> &Solution {
        self.counter = counter.clone();
        functional::calculate(self.rule, &mut self.counter, &mut self.solution);
        &self.solution
    }
}

#[derive(Debug, Clone)]
pub struct Counter {
    value_count: [u32; 13],
    card_count: [u32; 52],
}

impl Counter {
    pub fn new(number_of_decks: u32) -> Self {
        Counter {
            value_count: [4 * number_of_decks; 13],
            card_count: [number_of_decks; 52],
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.value_count[card.to_value_index()] += 1;
        self.card_count[card.to_index()] += 1;
    }

    pub fn remove_card(&mut self, card: Card) {
        self.value_count[card.to_value_index()] -= 1;
        self.card_count[card.to_index()] -= 1;
    }

    pub fn get_value_count(&self) -> &[u32; 13] {
        &self.value_count
    }

    pub fn get_card_count(&self) -> &[u32; 52] {
        &self.card_count
    }
}

#[derive(Debug, Clone, Default)]
pub struct Solution {
    pub sol_main: SolutionMain,
    pub sol_pair: SolutionPair,
    pub sol_bonus: SolutionBonus,
}

#[derive(Debug, Clone, Default)]
pub struct SolutionMain {
    pub p_player_win: f64,
    pub ex_player_win: f64,
    pub p_banker_win: f64,
    pub ex_banker_win: f64,
    pub p_tie: f64,
    pub ex_tie: f64,
}

#[derive(Debug, Clone, Default)]
pub struct SolutionPair {
    pub p_unsuit_pair: f64,
    pub ex_unsuit_pair: f64,

    pub p_suit_pair: [f64; 2],
    pub ex_suit_pair: f64,
}

#[derive(Debug, Clone, Default)]
pub struct SolutionBonus {
    pub p_player_bonus_unnatural: [f64; 6],
    pub p_player_bonus_natural_win: f64,
    pub p_banker_bonus_unnatural: [f64; 6],
    pub p_banker_bonus_natural_win: f64,
    pub p_bonus_natural_tie: f64,
    pub ex_player_bonus: f64,
    pub ex_banker_bonus: f64,
}

mod functional {
    use super::*;

    pub fn calculate(
        // Input
        rule: &Rule,
        counter: &mut Counter,

        // Output
        solution: &mut Solution,
    ) {
    }
}
