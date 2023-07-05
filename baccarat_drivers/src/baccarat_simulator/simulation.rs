use baccarat::calculation::Solution;
use baccarat::card::{Card, Shoe};
use baccarat::game::{DealerProvider, GamblerProvider, Game, Hand, HandsBet, RoundResult};
use baccarat::Rule;
use baccarat_drivers_lib::ConfigBaccaratSimulator;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn start_simulation(rule: &Rule, config: &ConfigBaccaratSimulator) {
    let print_firsts = RefCell::new(false);
    let dealer = SimulatorDealer::new(rule, &print_firsts);
    let gambler = SimulatorGambler::new(rule, config, &print_firsts);
    let mut game = Game::new(rule, dealer, gambler);
    game.start_game_loop();
}

#[derive(Debug, Clone)]
struct SimulatorGambler<'a> {
    _rule: &'a Rule,
    config: &'a ConfigBaccaratSimulator,
    print_firsts: &'a RefCell<bool>,

    bets: HashMap<HandsBet, i64>,

    rounds: u32,
    money: i64,
    min_money: i64,
}

impl<'a> SimulatorGambler<'a> {
    fn new(
        _rule: &'a Rule,
        config: &'a ConfigBaccaratSimulator,
        print_firsts: &'a RefCell<bool>,
    ) -> Self {
        Self {
            _rule,
            config,
            print_firsts,

            bets: HashMap::new(),

            rounds: 0,
            money: 0,
            min_money: 0,
        }
    }
}

impl<'a> GamblerProvider for SimulatorGambler<'a> {
    fn on_new_shoe(&mut self) {
        println!("NEW SHOE!!!!!!!");
        println!("++++++++++++++++++++++++++++++++++");
        let mut print_firsts = self.print_firsts.borrow_mut();
        *print_firsts = true;
    }
    fn on_cut_card_reached(&mut self, _cards_before_cut: u32) {}
    fn on_discard(&mut self, card: Card) {
        println!("Discard: {:#?}", card);
        println!("++++++++++++++++++++++++++++++++++");
    }
    fn on_round_start(&mut self) {
        println!(
            "Day #{}, Round #{}",
            self.rounds / self.config.rounds_per_day,
            self.rounds % self.config.rounds_per_day
        );
        let mut print_firsts = self.print_firsts.borrow_mut();
        *print_firsts = true;
    }
    fn place_bet(&mut self, solution: &Solution) -> &HashMap<HandsBet, i64> {
        self.bets.clear();

        let max_bets = solution.get_max_main_side_bets();
        println!(
            "Max main bet: {:#?}({:.5}). Max side bet: {:#?}({:.5})",
            max_bets.0 .0, max_bets.0 .1, max_bets.1 .0, max_bets.1 .1
        );

        // If side bet's ex < main bet's ex, we don't consider side bets.
        if max_bets.1 .1 < max_bets.0 .1 {
            if max_bets.0 .1 > 0.0 {
                self.bets.insert(max_bets.0 .0, 100);
            }
        }
        // If side bet's ex is greater, we may consider it.
        else {
            if 2.0 * max_bets.0 .1 + max_bets.1 .1 > 0.0 {
                self.bets.insert(max_bets.0 .0, 200);
                self.bets.insert(max_bets.1 .0, 100);
            }
        }

        print!("Bet:");
        for (bet, money) in &self.bets {
            print!(" ({:#?}, {})", bet, money);
        }
        println!();

        &self.bets
    }
    fn on_round_end(&mut self, player: &Hand, banker: &Hand, round_result: &RoundResult) {
        println!("Final hands: {:#?} {:#?}", player, banker);
        self.money += round_result.total_profit;
        let mut delta_min_money = 0;
        if self.money < self.min_money {
            delta_min_money = self.money - self.min_money;
            self.min_money = self.money;
        }
        println!(
            "Money: {}({}). Min: {}({})",
            self.money, round_result.total_profit, self.min_money, delta_min_money
        );
        println!("----------------------------------");
        self.rounds += 1;
    }
}

#[derive(Debug, Clone)]
struct SimulatorDealer<'a> {
    _rule: &'a Rule,
    shoe: Shoe,
    print_firsts: &'a RefCell<bool>,
}

impl<'a> SimulatorDealer<'a> {
    fn new(rule: &'a Rule, print_firsts: &'a RefCell<bool>) -> Self {
        Self {
            _rule: rule,
            shoe: Shoe::new(rule.number_of_decks, rule.cut_card_proportion),
            print_firsts,
        }
    }
}

impl<'a> DealerProvider for SimulatorDealer<'a> {
    fn deal_card(&mut self) -> Card {
        let mut print_firsts = self.print_firsts.borrow_mut();
        if *print_firsts {
            *print_firsts = false;
            let nexts = self.shoe.get_next_cards();
            const NUMBER: usize = 10;
            print!("First {} cards in shoe:", NUMBER);
            for (i, card) in nexts.iter().enumerate() {
                if i == NUMBER {
                    break;
                }
                print!(" {:#?}", card);
            }
            println!();
        }

        self.shoe.deal_card()
    }
    fn discard_cards(&mut self, cards: u32) {
        for _ in 0..cards {
            self.shoe.deal_card();
        }
    }
    fn is_cut_card_reached(&self) -> bool {
        self.shoe.is_cut_card_reached()
    }
    fn start_new_shoe(&mut self) {
        self.shoe.shuffle();
    }
}
