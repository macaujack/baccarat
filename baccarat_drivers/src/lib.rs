use baccarat::{Payouts, Rule};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(with = "ConfigRule")]
    pub rule: Rule,
    pub baccarat_simulator: ConfigBaccaratSimulator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(remote = "Rule")]
pub struct ConfigRule {
    pub number_of_decks: u32,
    pub cut_card_proportion: f64,
    pub discard_at_start: bool,

    #[serde(with = "ConfigRulePayouts")]
    pub payouts: Payouts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(remote = "Payouts")]
pub struct ConfigRulePayouts {
    pub player_win: f64,
    pub banker_win: f64,
    pub tie: f64,

    pub unsuit_pair: f64,
    pub perfect_pair: [f64; 2],

    pub bonus_unnatural: [f64; 6], // bonus[0] stands for "win by 4".
    pub bonus_natural_win: f64,
    pub bonus_natural_tie: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigBaccaratSimulator {
    pub rounds_per_day: u32,
    pub p_threshold: f64,
}

/// Reads the content of a given config file and parses it to a Config.
///
/// Panics if any error occurs.
pub fn parse_config_from_file(filename: &str) -> Config {
    let file_content = std::fs::read_to_string(filename).unwrap();
    serde_yaml::from_str(&file_content).unwrap()
}

#[derive(Debug, Clone)]
pub struct MoneyStat {
    cur: i64,
    min: i64,
    max: i64,
}

impl Default for MoneyStat {
    fn default() -> Self {
        Self {
            cur: 0,
            min: i64::MAX,
            max: i64::MIN,
        }
    }
}

impl MoneyStat {
    pub fn add(&mut self, delta: i64) {
        self.cur += delta;
        if self.min > self.cur {
            self.min = self.cur;
        }
        if self.max < self.cur {
            self.max = self.cur;
        }
    }

    pub fn cur_money(&self) -> i64 {
        self.cur
    }

    pub fn min_money(&self) -> i64 {
        self.min
    }

    pub fn max_money(&self) -> i64 {
        self.max
    }
}
