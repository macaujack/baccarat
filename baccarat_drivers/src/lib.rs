use baccarat::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub rule: Rule,
    pub baccarat_simulator: ConfigBaccaratSimulator,
    pub baccarat_solver_service: ConfigBaccaratSolverService,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigBaccaratSimulator {
    pub rounds_per_day: u32,
    pub p_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigBaccaratSolverService {
    #[serde(default = "default_listening_ip")]
    pub listening_ip: String,
    #[serde(default = "default_listening_port")]
    pub listening_port: u16,
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

fn default_listening_ip() -> String {
    String::from("127.0.0.1")
}

fn default_listening_port() -> u16 {
    8080
}
