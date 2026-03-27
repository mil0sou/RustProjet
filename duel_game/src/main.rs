use rand::Rng;
use log::{info, debug};
use clap::Parser;

/// Arguments du programme
#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "Player1")]
    name1: String,

    #[arg(long, default_value = "Player2")]
    name2: String,
}

fn main() {
    // init du logger
    env_logger::init();

    // parse les args CLI
    let args = Args::parse();

    info!("Démarrage du programme");

    println!("Joueur 1 : {}", args.name1);
    println!("Joueur 2 : {}", args.name2);

    // test rand : génère 5 objectifs aléatoires
    let mut rng = rand::thread_rng();
    let objectifs: Vec<u32> = (0..5).map(|_| rng.gen_range(0..=100)).collect();

    debug!("Objectifs générés : {:?}", objectifs);
    println!("Objectifs : {:?}", objectifs);
}