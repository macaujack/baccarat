use serde::{Deserialize, Serialize};

use crate::{card::Card, game::HandsBet, Rule};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
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
    total: u32,
    bcr_value_count: [u32; 10],
    value_count: [u32; 13],
    card_count: [u32; 52],
}

impl Counter {
    pub fn new(number_of_decks: u32) -> Self {
        let mut bcr_value_count = [4 * number_of_decks; 10];
        bcr_value_count[0] = 16 * number_of_decks;
        Counter {
            total: 52 * number_of_decks,
            bcr_value_count,
            value_count: [4 * number_of_decks; 13],
            card_count: [number_of_decks; 52],
        }
    }

    pub fn with_card_count(card_count: &[u32; 52]) -> Self {
        let mut counter = Self {
            total: 0,
            bcr_value_count: Default::default(),
            value_count: Default::default(),
            card_count: card_count.clone(),
        };
        for (i, value) in card_count.iter().enumerate() {
            counter.total += *value;
            let rem = i % 13;
            counter.value_count[rem] += *value;
            counter.bcr_value_count[(rem + 1) % 10] += *value;
        }
        counter
    }

    pub fn add_card(&mut self, card: Card) {
        self.total += 1;
        self.bcr_value_count[card.to_bcr_value_index()] += 1;
        self.value_count[card.to_value_index()] += 1;
        self.card_count[card.to_index()] += 1;
    }

    pub fn remove_card(&mut self, card: Card) {
        self.total -= 1;
        self.bcr_value_count[card.to_bcr_value_index()] -= 1;
        self.value_count[card.to_value_index()] -= 1;
        self.card_count[card.to_index()] -= 1;
    }

    pub fn get_total(&self) -> u32 {
        self.total
    }

    pub fn get_bcr_value_count(&self) -> &[u32; 10] {
        &self.bcr_value_count
    }

    pub fn get_value_count(&self) -> &[u32; 13] {
        &self.value_count
    }

    pub fn get_card_count(&self) -> &[u32; 52] {
        &self.card_count
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Solution {
    pub sol_main: SolutionMain,
    pub sol_pair: SolutionPair,
    pub sol_bonus: SolutionBonus,
}

impl Solution {
    /// This function get the best main bet and side bet based on each bet's ex.
    /// Note that not all bet is taken into consideration. We only care those
    /// bets whose probabilities are greater than p_threshold.
    pub fn get_best_main_side_bet(&self, p_threshold: f64) -> ((HandsBet, f64), (HandsBet, f64)) {
        let (mut max_main_bet, mut max_main_ex) = (HandsBet::PlaceHolder, f64::MIN);
        let (mut max_side_bet, mut max_side_ex) = (HandsBet::PlaceHolder, f64::MIN);
        // Check main bets.
        let s = &self.sol_main;
        if s.p_player_win + s.p_tie > p_threshold && max_main_ex < s.ex_player_win {
            (max_main_bet, max_main_ex) = (HandsBet::PlayerWin, s.ex_player_win);
        }
        if s.p_banker_win + s.p_tie > p_threshold && max_main_ex < s.ex_banker_win {
            (max_main_bet, max_main_ex) = (HandsBet::BankerWin, s.ex_banker_win);
        }
        if s.p_tie > p_threshold && max_main_ex < s.ex_tie {
            (max_main_bet, max_main_ex) = (HandsBet::Tie, s.ex_tie);
        }

        // Check side bets (pair).
        let s = &self.sol_pair;
        if s.p_unsuit_pair > p_threshold && max_side_ex < s.ex_unsuit_pair {
            (max_side_bet, max_side_ex) = (HandsBet::PlayerUnsuitPair, s.ex_unsuit_pair);
        }
        if s.p_either_pair > p_threshold && max_side_ex < s.ex_either_pair {
            (max_side_bet, max_side_ex) = (HandsBet::EitherPair, s.ex_either_pair);
        }
        if s.p_suit_pair[0] + s.p_suit_pair[1] > p_threshold && max_side_ex < s.ex_suit_pair {
            (max_side_bet, max_side_ex) = (HandsBet::PerfectPair, s.ex_suit_pair);
        }

        // Check side bets (bonus).
        let s = &self.sol_bonus;
        let p_player_unnatural: f64 = s.p_player_bonus_unnatural.iter().sum();
        if s.p_bonus_natural_tie + s.p_player_bonus_natural_win + p_player_unnatural > p_threshold
            && max_side_ex < s.ex_player_bonus
        {
            (max_side_bet, max_side_ex) = (HandsBet::PlayerBonus, s.ex_player_bonus);
        }
        let p_banker_unnatural: f64 = s.p_banker_bonus_unnatural.iter().sum();
        if s.p_bonus_natural_tie + s.p_banker_bonus_natural_win + p_banker_unnatural > p_threshold
            && max_side_ex < s.ex_banker_bonus
        {
            (max_side_bet, max_side_ex) = (HandsBet::BankerBonus, s.ex_banker_bonus);
        }

        ((max_main_bet, max_main_ex), (max_side_bet, max_side_ex))
    }

    fn calculate_ex_based_on_p(&mut self, rule: &Rule) {
        let payouts = &rule.payouts;

        // Calculate solution for main bets.
        let s = &mut self.sol_main;
        s.ex_player_win = s.p_player_win * payouts.player_win - s.p_banker_win;
        s.ex_banker_win = s.p_banker_win * payouts.banker_win - s.p_player_win;
        s.ex_tie = s.p_tie * payouts.tie - (1.0 - s.p_tie);

        // Calculate solution for pair bets.
        let s = &mut self.sol_pair;
        s.ex_unsuit_pair = s.p_unsuit_pair * payouts.unsuit_pair - (1.0 - s.p_unsuit_pair);
        s.ex_either_pair = s.p_either_pair * payouts.either_pair - (1.0 - s.p_either_pair);
        s.ex_suit_pair = s.p_suit_pair[0] * payouts.perfect_pair[0]
            + s.p_suit_pair[1] * payouts.perfect_pair[1]
            - (1.0 - s.p_suit_pair[0] - s.p_suit_pair[1]);

        // Calculate solution for bonus bets.
        let s = &mut self.sol_bonus;
        s.ex_player_bonus = s.p_player_bonus_natural_win * payouts.bonus_natural_win
            + s.p_bonus_natural_tie * payouts.bonus_natural_tie;
        s.ex_banker_bonus = s.p_banker_bonus_natural_win as f64 * payouts.bonus_natural_win
            + s.p_bonus_natural_tie * payouts.bonus_natural_tie;
        let mut p_player_lose = 1.0 - s.p_player_bonus_natural_win - s.p_bonus_natural_tie;
        let mut p_banker_lose = 1.0 - s.p_banker_bonus_natural_win - s.p_bonus_natural_tie;
        for i in 0..payouts.bonus_unnatural.len() {
            s.ex_player_bonus += s.p_player_bonus_unnatural[i] * payouts.bonus_unnatural[i];
            s.ex_banker_bonus += s.p_banker_bonus_unnatural[i] * payouts.bonus_unnatural[i];
            p_player_lose -= s.p_player_bonus_unnatural[i];
            p_banker_lose -= s.p_banker_bonus_unnatural[i];
        }
        s.ex_player_bonus -= p_player_lose;
        s.ex_banker_bonus -= p_banker_lose;
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SolutionMain {
    pub p_player_win: f64,
    pub ex_player_win: f64,

    pub p_banker_win: f64,
    pub ex_banker_win: f64,

    pub p_tie: f64,
    pub ex_tie: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SolutionPair {
    pub p_unsuit_pair: f64,
    pub ex_unsuit_pair: f64,

    pub p_either_pair: f64,
    pub ex_either_pair: f64,

    pub p_suit_pair: [f64; 2],
    pub ex_suit_pair: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct SolutionBonus {
    pub p_player_bonus_unnatural: [f64; 6],
    pub p_player_bonus_natural_win: f64,
    pub p_banker_bonus_unnatural: [f64; 6],
    pub p_banker_bonus_natural_win: f64,
    pub p_bonus_natural_tie: f64,

    pub ex_player_bonus: f64,
    pub ex_banker_bonus: f64,
}

const S: bool = false;
const D: bool = true;
pub static RULE_CHART: [[bool; 10]; 10] = [
    [D, D, D, D, D, D, D, D, D, D],
    [D, D, D, D, D, D, D, D, D, D],
    [D, D, D, D, D, D, D, D, D, D],
    [D, D, D, D, D, D, D, D, S, D],
    [S, S, D, D, D, D, D, D, S, S],
    [S, S, S, S, D, D, D, D, S, S],
    [S, S, S, S, S, S, D, D, S, S],
    [S, S, S, S, S, S, S, S, S, S],
    [S, S, S, S, S, S, S, S, S, S],
    [S, S, S, S, S, S, S, S, S, S],
];

pub mod functional {
    use super::*;

    pub fn calculate(
        // Input
        rule: &Rule,
        counter: &mut Counter,

        // Output
        solution: &mut Solution,
    ) {
        *solution = Default::default();

        // Step 1: Calculate probabilities of main bets and bonus bets.
        let bcr_counter = &mut counter.bcr_value_count;
        let total_count = counter.total;
        for p0 in 0..=9 {
            if bcr_counter[p0] == 0 {
                continue;
            }
            let p = bcr_counter[p0] as f64 / total_count as f64;
            bcr_counter[p0] -= 1;
            let total_count = total_count - 1;

            for p1 in 0..=p0 {
                if bcr_counter[p1] == 0 {
                    continue;
                }
                let p = p * (bcr_counter[p1] * if p0 == p1 { 1 } else { 2 }) as f64
                    / total_count as f64;
                bcr_counter[p1] -= 1;
                let total_count = total_count - 1;
                let player_sum = (p0 + p1) % 10;

                for b0 in 0..=9 {
                    if bcr_counter[b0] == 0 {
                        continue;
                    }
                    let p = p * bcr_counter[b0] as f64 / total_count as f64;
                    bcr_counter[b0] -= 1;
                    let total_count = total_count - 1;

                    for b1 in 0..=b0 {
                        if bcr_counter[b1] == 0 {
                            continue;
                        }
                        let p = p * (bcr_counter[b1] * if b0 == b1 { 1 } else { 2 }) as f64
                            / total_count as f64;
                        bcr_counter[b1] -= 1;
                        let total_count = total_count - 1;
                        let banker_sum = (b0 + b1) % 10;

                        // Core logic 1: Check if player or/and banker get(s) a natural.
                        if player_sum >= 8 || banker_sum >= 8 {
                            match player_sum.cmp(&banker_sum) {
                                Ordering::Less => {
                                    solution.sol_main.p_banker_win += p;
                                    solution.sol_bonus.p_banker_bonus_natural_win += p;
                                }
                                Ordering::Equal => {
                                    solution.sol_main.p_tie += p;
                                    solution.sol_bonus.p_bonus_natural_tie += p;
                                }
                                Ordering::Greater => {
                                    solution.sol_main.p_player_win += p;
                                    solution.sol_bonus.p_player_bonus_natural_win += p;
                                }
                            }
                        }
                        // Core logic 2: If player's initial sum is 6 or 7, then no extra
                        // card is dealt to player.
                        else if player_sum >= 6 {
                            // When player doesn't draw a card, at the same time banker's
                            // initial sum is also 6 or 7, then banker doesn't draw a
                            // card either. Otherwise banker draws a card.
                            if banker_sum >= 6 {
                                add_p_of_unnatural_to_solution(player_sum, banker_sum, solution, p);
                            } else {
                                deal_final_banker_card_loop(
                                    bcr_counter,
                                    player_sum,
                                    banker_sum,
                                    p,
                                    total_count,
                                    solution,
                                );
                            }
                        }
                        // Core logic 3: If player's initial sum <= 5, then player draws
                        // an extra card. Then whether banker draws or stands depends on
                        // her initial sum and the extra card drawn by player.
                        else {
                            for player_extra_card in 0..=9 {
                                if bcr_counter[player_extra_card] == 0 {
                                    continue;
                                }
                                let p =
                                    p * bcr_counter[player_extra_card] as f64 / total_count as f64;
                                let total_count = total_count - 1;
                                bcr_counter[player_extra_card] -= 1;
                                let player_sum = (player_sum + player_extra_card) % 10;

                                // Core logic 4: Check if banker should draw an extra card.
                                if RULE_CHART[banker_sum][player_extra_card] {
                                    deal_final_banker_card_loop(
                                        bcr_counter,
                                        player_sum,
                                        banker_sum,
                                        p,
                                        total_count,
                                        solution,
                                    );
                                } else {
                                    add_p_of_unnatural_to_solution(
                                        player_sum, banker_sum, solution, p,
                                    );
                                }

                                bcr_counter[player_extra_card] += 1;
                            }
                        }

                        bcr_counter[b1] += 1;
                    }
                    bcr_counter[b0] += 1;
                }
                bcr_counter[p1] += 1;
            }
            bcr_counter[p0] += 1;
        }

        // Step 2: Calculate probabilities of pair bets.
        let total_pairs = (counter.total * (counter.total - 1)) as f64;
        for count in counter.value_count {
            solution.sol_pair.p_unsuit_pair += (count * count.wrapping_sub(1)) as f64 / total_pairs;
        }

        let total_quads = {
            let tot = counter.total as u128;
            tot * (tot - 1) * (tot - 2) * (tot - 3)
        } as f64;
        let either_pair = {
            let mut num1 = 0u128;
            let mut num2 = 0u128;
            for i in 0..13 {
                let count1 = counter.value_count[i] as u128;
                if count1 <= 1 {
                    continue;
                }
                let res = count1 * (count1 - 1);
                counter.value_count[i] -= 2;

                for j in 0..13 {
                    let count2 = counter.value_count[j] as u128;
                    if count2 <= 1 {
                        continue;
                    }
                    let res1 = res * count2 * (counter.total - 2 - counter.value_count[j]) as u128;
                    num1 += res1;
                    let res2 = res * count2 * (count2 - 1);
                    num2 += res2;
                }

                counter.value_count[i] += 2;
            }
            num1 * 2 + num2
        };
        solution.sol_pair.p_either_pair = either_pair as f64 / total_quads;

        let card_count = &mut counter.card_count;
        for i in 0..52 {
            if card_count[i] <= 1 {
                continue;
            }
            let p_first_pair =
                (card_count[i] * (card_count[i].wrapping_sub(1))) as f64 / total_pairs;
            card_count[i] -= 2;
            let mut p_second_pair = 0.0;
            let total_pairs = ((counter.total - 2) * (counter.total - 3)) as f64;
            for count in card_count.iter() {
                p_second_pair += (*count * (count.wrapping_sub(1))) as f64 / total_pairs;
            }
            card_count[i] += 2;

            solution.sol_pair.p_suit_pair[0] += p_first_pair * (1.0 - p_second_pair);
            solution.sol_pair.p_suit_pair[1] += p_first_pair * p_second_pair;
        }
        solution.sol_pair.p_suit_pair[0] *= 2.0;

        // Step 3: Calculate expectations.
        solution.calculate_ex_based_on_p(rule);
    }

    fn deal_final_banker_card_loop(
        bcr_counter: &[u32; 10],
        player_sum: usize,
        banker_sum: usize,
        p: f64,
        total_count: u32,
        solution: &mut Solution,
    ) {
        for card in 0..=9 {
            if bcr_counter[card] == 0 {
                continue;
            }
            let p = p * bcr_counter[card] as f64 / total_count as f64;
            let banker_sum = (banker_sum + card) % 10;
            add_p_of_unnatural_to_solution(player_sum, banker_sum, solution, p);
        }
    }

    fn add_p_of_unnatural_to_solution(
        player_sum: usize,
        banker_sum: usize,
        solution: &mut Solution,
        p: f64,
    ) {
        match player_sum.cmp(&banker_sum) {
            Ordering::Less => {
                solution.sol_main.p_banker_win += p;
                let delta = banker_sum - player_sum;
                if delta >= 4 {
                    solution.sol_bonus.p_banker_bonus_unnatural[delta - 4] += p;
                }
            }
            Ordering::Equal => {
                solution.sol_main.p_tie += p;
            }
            Ordering::Greater => {
                solution.sol_main.p_player_win += p;
                let delta = player_sum - banker_sum;
                if delta >= 4 {
                    solution.sol_bonus.p_player_bonus_unnatural[delta - 4] += p;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Payouts;

    fn get_typical_rule() -> Rule {
        Rule {
            number_of_decks: 8,
            cut_card_proportion: 0.9,
            discard_at_start: true,

            payouts: Payouts {
                player_win: 1.0,
                banker_win: 0.95,
                tie: 8.0,

                unsuit_pair: 11.0,
                either_pair: 5.0,
                perfect_pair: [25.0, 200.0],

                bonus_unnatural: [1.0, 2.0, 4.0, 6.0, 10.0, 30.0],
                bonus_natural_win: 1.0,
                bonus_natural_tie: 0.0,
            },
        }
    }

    fn assert_float_equal(mut x: f64, mut y: f64) {
        const EPS: f64 = 0.0001;
        if x > y {
            (x, y) = (y, x);
        }
        assert!((y - x) <= EPS * x.abs());
    }

    #[test]
    fn test_calculation_result() {
        // Ground truth RTP based on 8 decks.
        const RTP_BANKER_WIN: f64 = 0.9894;
        const EX_PLAYER_WIN: f64 = -0.012351;
        const EX_TIE: f64 = -0.143596;
        const RTP_UNSUIT_PAIR: f64 = 0.8964;
        const RTP_EITHER_PAIR: f64 = 0.8629;
        const RTP_PERFECT_PAIR: f64 = 0.9195;
        const RTP_PLAYER_BONUS: f64 = 0.9735;
        const RTP_BANKER_BONUS: f64 = 0.9063;

        let rule = get_typical_rule();
        let mut counter = Counter::new(8);
        let mut solution: Solution = Default::default();

        functional::calculate(&rule, &mut counter, &mut solution);

        assert_float_equal(1.0 + solution.sol_main.ex_banker_win, RTP_BANKER_WIN);
        assert_float_equal(solution.sol_main.ex_player_win, EX_PLAYER_WIN);
        assert_float_equal(solution.sol_main.ex_tie, EX_TIE);
        assert_float_equal(1.0 + solution.sol_pair.ex_unsuit_pair, RTP_UNSUIT_PAIR);
        assert_float_equal(1.0 + solution.sol_pair.ex_either_pair, RTP_EITHER_PAIR);
        assert_float_equal(1.0 + solution.sol_pair.ex_suit_pair, RTP_PERFECT_PAIR);
        assert_float_equal(1.0 + solution.sol_bonus.ex_player_bonus, RTP_PLAYER_BONUS);
        assert_float_equal(1.0 + solution.sol_bonus.ex_banker_bonus, RTP_BANKER_BONUS);
    }
}
