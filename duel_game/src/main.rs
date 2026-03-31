mod player;
mod scoring;
mod turn;
mod game;

use clap::Parser;
use player::Player;
use game::{run_game, ask_replay};

/// Jeu de duel en terminal, deux joueurs, tour par tour
#[derive(Parser)]
#[command(name = "duel_game", about = "Mini jeu de duel en Rust")]
struct Args {
    /// Nom du joueur 1
    #[arg(long, default_value = "Player1")]
    name1: String,

    /// Nom du joueur 2
    #[arg(long, default_value = "Player2")]
    name2: String,

    /// Points de vie de départ
    #[arg(long, default_value_t = 50)]
    vitality: i32,

    /// Nombre d'objectifs par tour
    #[arg(long, default_value_t = 5)]
    objectifs: usize,
}

fn main() {
    env_logger::init();

    let args = Args::parse();

    loop {
        let p1 = Player::new(args.name1.clone(), args.vitality);
        let p2 = Player::new(args.name2.clone(), args.vitality);

        run_game(p1, p2, args.objectifs);

        if !ask_replay() {
            break;
        }
    }

    println!("À bientôt !");
}