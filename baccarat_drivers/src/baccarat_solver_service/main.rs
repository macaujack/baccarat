use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use baccarat::{
    calculation::{self, Counter, Solution},
    Rule,
};
use baccarat_drivers_lib::parse_config_from_file;
use clap::Parser;
use std::{mem::MaybeUninit, sync::RwLock};

const DEFAULT_CONFIG_PATH: &str = "~/.baccarat.yml";

#[derive(Debug, Parser)]
#[command(author, about, long_about = None)]
struct CommandLineArgs {
    /// The path of the config file
    #[arg(short, long, default_value_t = String::from(DEFAULT_CONFIG_PATH))]
    config: String,

    /// The listening IP address
    #[arg(long)]
    ip: Option<String>,

    /// The listening port
    #[arg(long)]
    port: Option<u16>,
}

#[derive(Debug)]
struct ServiceState {
    rule: RwLock<Rule>,
}

#[post("/solve")]
async fn solve(card_count: web::Json<Vec<u32>>, state: web::Data<ServiceState>) -> impl Responder {
    let rule = state.rule.read().unwrap();
    let card_count = card_count.into_inner();
    if card_count.len() != 52 {
        return HttpResponse::BadRequest().body("Array length must be 52");
    }
    let card_count: [u32; 52] = unsafe {
        let mut arr: [MaybeUninit<u32>; 52] = MaybeUninit::uninit().assume_init();
        for (i, e) in card_count.iter().enumerate() {
            arr[i] = MaybeUninit::new(*e);
        }
        std::mem::transmute_copy(&arr)
    };

    let mut counter = Counter::with_card_count(&card_count);
    let mut solution = Solution::default();
    calculation::functional::calculate(&rule, &mut counter, &mut solution);
    HttpResponse::Ok().json(solution)
}

#[post("/change_rule")]
async fn change_rule(new_rule: web::Json<Rule>, state: web::Data<ServiceState>) -> impl Responder {
    let mut rule = state.rule.write().unwrap();
    let new_rule: Rule = new_rule.into_inner().into();
    *rule = new_rule;
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut args = CommandLineArgs::parse();
    if args.config == DEFAULT_CONFIG_PATH {
        let home_dir = home::home_dir().expect("Cannot find home directory");
        let config_file_path = home_dir.join(".baccarat.yml");
        if !config_file_path.exists() {
            panic!("Config file not exists");
        }
        if config_file_path.is_dir() {
            panic!("This should be a path rather than a directory");
        }
        args.config = String::from(config_file_path.to_str().unwrap());
    }
    let args = args;
    let mut config = parse_config_from_file(&args.config);

    if let Some(listening_ip) = args.ip {
        config.baccarat_solver_service.listening_ip = listening_ip;
    }
    if let Some(listening_port) = args.port {
        config.baccarat_solver_service.listening_port = listening_port;
    }

    let c = &config.baccarat_solver_service;
    println!(
        "Baccarat Solver Service running at {}:{}",
        c.listening_ip, c.listening_port
    );

    let rule: Rule = config.rule;
    let state = web::Data::new(ServiceState {
        rule: RwLock::new(rule.clone()),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(solve)
            .service(change_rule)
    })
    .bind((&c.listening_ip as &str, c.listening_port))?
    .run()
    .await
}
