use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use baccarat::{
    calculation::{self, Counter, Solution},
    Rule,
};
use baccarat_drivers_lib::parse_config_from_file;
use clap::Parser;
use rust_embed::RustEmbed;
use std::{mem::MaybeUninit, sync::RwLock};

#[cfg(feature = "embed_website_assets")]
use actix_web::get;
#[cfg(feature = "embed_website_assets")]
use mime_guess;

const DEFAULT_CONFIG_PATH: &str = "~/.baccarat.yml";

#[derive(Debug, Parser)]
#[command(author = "Jack Y. <seigino.mikata@outlook.com>")]
#[command(about = "A backend service to calculate probabilities and expectations in Baccarat.", long_about = None)]
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

#[post("/api/solve")]
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

#[post("/api/change_rule")]
async fn change_rule(new_rule: web::Json<Rule>, state: web::Data<ServiceState>) -> impl Responder {
    let mut rule = state.rule.write().unwrap();
    let new_rule: Rule = new_rule.into_inner().into();
    *rule = new_rule;
    HttpResponse::Ok()
}

#[cfg(feature = "embed_website_assets")]
#[derive(RustEmbed)]
#[folder = "../baccarat_assistant/dist/"]
struct Assets;

#[cfg(feature = "embed_website_assets")]
fn handle_embedded_file(path: &str) -> HttpResponse {
    match Assets::get(path) {
        Some(content) => HttpResponse::Ok()
            .content_type(mime_guess::from_path(path).first_or_octet_stream().as_ref())
            .body(content.data.into_owned()),
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

#[cfg(feature = "embed_website_assets")]
#[get("/")]
async fn index() -> impl Responder {
    let index = Assets::get("index.html").unwrap();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(index.data.into_owned())
}

#[cfg(feature = "embed_website_assets")]
#[get("/{_:.*}")]
async fn dist(path: web::Path<String>) -> impl Responder {
    handle_embedded_file(path.as_str())
}

#[derive(RustEmbed)]
#[folder = "../"]
#[include = "sample_config.yml"]
struct SampleConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut args = CommandLineArgs::parse();
    if args.config == DEFAULT_CONFIG_PATH {
        let home_dir = home::home_dir().expect("Cannot find home directory");
        let config_file_path = home_dir.join(".baccarat.yml");
        if config_file_path.is_dir() {
            panic!("This should be a path rather than a directory");
        }
        if !config_file_path.exists() {
            println!(
                "Config file not exists. Creating a default config at {}",
                config_file_path.display()
            );
            let sample_config = SampleConfig::get("sample_config.yml").unwrap();
            let sample_config = sample_config.data.into_owned();
            if let Err(e) = std::fs::write(&config_file_path, sample_config) {
                println!("Cannot create config file: {}", e);
            }
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
        let app = App::new()
            .app_data(state.clone())
            .service(solve)
            .service(change_rule);

        #[cfg(feature = "embed_website_assets")]
        let app = app.service(index).service(dist);

        app
    })
    .bind((&c.listening_ip as &str, c.listening_port))?
    .run()
    .await
}
