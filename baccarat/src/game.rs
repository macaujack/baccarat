mod hands;

pub use hands::{Hand, HandsBet, HandsResult, HandsResultBonus, RoundResult};
use std::collections::HashMap;

use crate::{
    calculation::{Counter, Solution, Solver, RULE_CHART},
    card::Card,
    Rule,
};

#[derive(Debug, Clone)]
pub struct Game<'a, T: DealerProvider, U: GamblerProvider> {
    rule: &'a Rule,
    counter: Counter,
    player: Hand,
    banker: Hand,
    solver: Solver<'a>,
    round_result: RoundResult<'a>,

    dealer: T,
    gambler: U,

    discarded_card: Card,
    should_start_new_shoe: bool,
    cards_before_cut: u32,
}

impl<'a, T: DealerProvider, U: GamblerProvider> Game<'a, T, U> {
    pub fn new(rule: &'a Rule, dealer: T, gambler: U) -> Self {
        let number_of_decks = rule.number_of_decks;
        Self {
            rule,
            counter: Counter::new(number_of_decks),
            player: Default::default(),
            banker: Default::default(),
            solver: Solver::new(rule),
            round_result: RoundResult::new(rule),

            dealer,
            gambler,

            discarded_card: Default::default(),
            should_start_new_shoe: true,
            cards_before_cut: 0,
        }
    }

    pub fn start_game_loop(&mut self) {
        loop {
            if self.should_start_new_shoe {
                self.gambler.on_new_shoe();

                // Initialize.
                self.should_start_new_shoe = false;
                self.counter = Counter::new(self.rule.number_of_decks);
                self.cards_before_cut = 0;

                self.dealer.start_new_shoe();

                // Discard some cards.
                if self.rule.discard_at_start {
                    self.discarded_card = self.get_card_from_dealer();
                    let discarded_cards = self.discarded_card.to_bcr_value_index();
                    let discarded_cards = if discarded_cards == 0 {
                        10
                    } else {
                        discarded_cards
                    } as u32;
                    self.cards_before_cut += discarded_cards;
                    self.gambler.on_discard(self.discarded_card);
                    self.dealer.discard_cards(discarded_cards);
                }
            }

            self.gambler.on_round_start();

            let solution = self.solver.solve(&self.counter);
            let bets = self.gambler.place_bet(solution) as *const _;

            self.player.third = None;
            self.banker.third = None;
            self.player.initial[0] = self.get_card_from_dealer();
            self.banker.initial[0] = self.get_card_from_dealer();
            self.player.initial[1] = self.get_card_from_dealer();
            self.banker.initial[1] = self.get_card_from_dealer();
            if !self.player.is_natural() && !self.banker.is_natural() {
                // Check if player should draw the extra card.
                if self.player.get_sum() <= 5 {
                    self.player.third = Some(self.get_card_from_dealer());
                }

                // Check if banker should draw the extra card.
                if let Some(player_third) = self.player.third {
                    if RULE_CHART[self.banker.get_sum() as usize][player_third.to_bcr_value_index()]
                    {
                        self.banker.third = Some(self.get_card_from_dealer());
                    }
                } else if self.banker.get_sum() <= 5 {
                    self.banker.third = Some(self.get_card_from_dealer());
                }
            }

            self.round_result
                .calculate_with_hands_and_bet(&self.player, &self.banker, unsafe { &*bets });
            self.gambler
                .on_round_end(&self.player, &self.banker, &self.round_result);
        }
    }

    fn get_card_from_dealer(&mut self) -> Card {
        let card = self.dealer.deal_card();
        self.counter.remove_card(card);
        self.cards_before_cut += 1;
        if self.dealer.is_cut_card_reached() {
            self.gambler.on_cut_card_reached(self.cards_before_cut);
            self.should_start_new_shoe = true;
        }
        card
    }
}

pub trait DealerProvider {
    fn deal_card(&mut self) -> Card;
    fn discard_cards(&mut self, cards: u32);
    fn is_cut_card_reached(&self) -> bool;
    fn start_new_shoe(&mut self);
}

pub trait GamblerProvider {
    fn place_bet(&mut self, solution: &Solution) -> &HashMap<HandsBet, i64>;
    fn on_new_shoe(&mut self);
    fn on_discard(&mut self, card: Card);
    fn on_round_start(&mut self);
    fn on_round_end(&mut self, player: &Hand, banker: &Hand, round_result: &RoundResult);
    fn on_cut_card_reached(&mut self, cards_before_cut: u32);
}
