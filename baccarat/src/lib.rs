pub mod calculation;
pub mod card;
pub mod game;

#[derive(Debug, Clone)]
pub struct Rule {
    pub number_of_decks: u32,
    pub cut_card_proportion: f64,

    pub payouts: Payouts,
}

#[derive(Debug, Clone)]
pub struct Payouts {
    pub player_win: f64,
    pub banker_win: f64,
    pub tie: f64,

    pub unsuit_pair: f64,
    pub perfect_pair: [f64; 2],

    pub bonus_unnatural: [f64; 6], // bonus[0] stands for "win by 4".
    pub bonus_natural_win: f64,
    pub bonus_natural_tie: f64,
}

fn get_typical_rule() -> Rule {
    Rule {
        number_of_decks: 8,
        cut_card_proportion: 0.9,

        payouts: Payouts {
            player_win: 1.0,
            banker_win: 0.95,
            tie: 8.0,

            unsuit_pair: 11.0,
            perfect_pair: [25.0, 200.0],

            bonus_unnatural: [1.0, 2.0, 4.0, 6.0, 10.0, 30.0],
            bonus_natural_win: 1.0,
            bonus_natural_tie: 0.0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
