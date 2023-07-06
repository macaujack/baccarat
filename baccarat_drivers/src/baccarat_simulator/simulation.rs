use baccarat::calculation::Solution;
use baccarat::card::{Card, Shoe};
use baccarat::game::{DealerProvider, GamblerProvider, Game, Hand, HandsBet, RoundResult};
use baccarat::Rule;
use baccarat_drivers_lib::{ConfigBaccaratSimulator, MoneyStat};
use std::cell::RefCell;
use std::collections::HashMap;

pub fn start_simulation(rule: &Rule, config: &ConfigBaccaratSimulator) {
    let firsts = RefCell::new(None);
    let dealer = SimulatorDealer::new(rule, &firsts);
    let gambler = SimulatorGambler::new(rule, config, &firsts);
    let mut game = Game::new(rule, dealer, gambler);
    game.start_game_loop();
}

#[derive(Debug, Clone)]
struct SimulatorGambler<'a> {
    _rule: &'a Rule,
    config: &'a ConfigBaccaratSimulator,
    firsts: &'a RefCell<Option<Vec<Card>>>,

    bets: HashMap<HandsBet, i64>,

    rounds: u32,

    // Records for log
    max_bets: ((HandsBet, f64), (HandsBet, f64)),
    money_stat: MoneyStat,
}

impl<'a> SimulatorGambler<'a> {
    fn new(
        _rule: &'a Rule,
        config: &'a ConfigBaccaratSimulator,
        firsts: &'a RefCell<Option<Vec<Card>>>,
    ) -> Self {
        Self {
            _rule,
            config,
            firsts,

            bets: HashMap::new(),

            rounds: 0,

            max_bets: (
                (HandsBet::BankerWin, -f64::INFINITY),
                (HandsBet::PlayerBonus, -f64::INFINITY),
            ),
            money_stat: Default::default(),
        }
    }

    fn print_log(&self, delta_money: i64, final_player: &Hand, final_banker: &Hand) {
        println!(
            "Day #{}, Round #{}",
            self.rounds / self.config.rounds_per_day,
            self.rounds % self.config.rounds_per_day
        );

        println!(
            "Max main bet: {:#?}({:.5}). Max side bet: {:#?}({:.5})",
            self.max_bets.0 .0, self.max_bets.0 .1, self.max_bets.1 .0, self.max_bets.1 .1
        );

        print!("Bet:");
        for (bet, money) in &self.bets {
            print!(" ({:#?}, {})", bet, money);
        }
        println!();

        let mut firsts = self.firsts.borrow_mut();
        let mut tem = None;
        std::mem::swap(&mut *firsts, &mut tem);
        let firsts = tem.unwrap();
        print!("First {} cards in shoe:", firsts.len());
        for card in firsts {
            print!(" {:#?}", card);
        }
        println!();

        println!("Final hands: {:#?} {:#?}", final_player, final_banker);

        println!(
            "Money: {}({}). Min: {}. Max: {}.",
            self.money_stat.cur_money(),
            delta_money,
            self.money_stat.min_money(),
            self.money_stat.max_money()
        );

        println!("----------------------------------");
    }
}

impl<'a> GamblerProvider for SimulatorGambler<'a> {
    fn on_new_shoe(&mut self) {
        println!("NEW SHOE!!!!!!!");
        println!("++++++++++++++++++++++++++++++++++");
        let mut firsts = self.firsts.borrow_mut();
        *firsts = None;
    }
    fn on_cut_card_reached(&mut self, _cards_before_cut: u32) {}
    fn on_discard(&mut self, card: Card) {
        println!("Discard: {:#?}", card);
        println!("++++++++++++++++++++++++++++++++++");
    }
    fn on_round_start(&mut self) {
        let mut firsts = self.firsts.borrow_mut();
        *firsts = None;
    }
    fn place_bet(&mut self, solution: &Solution) -> &HashMap<HandsBet, i64> {
        self.bets.clear();

        let max_bets = solution.get_max_main_side_bets();
        self.max_bets = max_bets;

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

        &self.bets
    }
    fn on_round_end(&mut self, player: &Hand, banker: &Hand, round_result: &RoundResult) {
        self.money_stat.add(round_result.total_profit);
        if self.bets.len() != 0 {
            self.print_log(round_result.total_profit, player, banker);
        }
        self.rounds += 1;
    }
}

#[derive(Debug, Clone)]
struct SimulatorDealer<'a> {
    _rule: &'a Rule,
    shoe: Shoe,
    firsts: &'a RefCell<Option<Vec<Card>>>,
}

impl<'a> SimulatorDealer<'a> {
    fn new(rule: &'a Rule, firsts: &'a RefCell<Option<Vec<Card>>>) -> Self {
        Self {
            _rule: rule,
            shoe: Shoe::new(rule.number_of_decks, rule.cut_card_proportion),
            firsts,
        }
    }
}

impl<'a> DealerProvider for SimulatorDealer<'a> {
    fn deal_card(&mut self) -> Card {
        let mut firsts = self.firsts.borrow_mut();
        if firsts.is_none() {
            let nexts = self.shoe.get_next_cards();
            const NUMBER: usize = 10;
            let len = std::cmp::min(nexts.len(), NUMBER);
            let mut v = vec![Default::default(); len];
            v.clone_from_slice(&nexts[..len]);
            *firsts = Some(v);
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
