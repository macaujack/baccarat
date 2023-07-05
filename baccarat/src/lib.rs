pub mod calculation;
pub mod card;
pub mod game;

#[derive(Debug, Clone)]
pub struct Rule {
    pub number_of_decks: u32,
    pub cut_card_proportion: f64,
    pub discard_at_start: bool,

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
