use std::io::{self, Write};
use rand::Rng;
use log::info;

use crate::player::Player;
use crate::scoring::average_score;
use crate::turn::play_objectif;

/// Génère un vecteur d'objectifs aléatoires entre 0 et 100
pub fn generate_objectifs(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(0..=100)).collect()
}

/// Joue le tour d'un joueur, retourne son score moyen
pub fn play_turn(player: &Player, objectifs: &[u32]) -> i32 {
    println!("\nAu tour de {} (Vitality={}, Speed={}, Strength={})",
        player.name, player.vitality, player.speed, player.strength);

    println!("→ Objectifs : {:?}", objectifs);
    println!("→ Appuie sur ENTREE pour démarrer le tour..");

    // attend ENTREE pour démarrer
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    let mut scores = Vec::new();

    for &target in objectifs {
        let result = play_objectif(target, player.strength, player.speed);
        let score = result.score;
        println!("→ Objectif {:>3} : Miss = {} | Compteur = {:>3}  // Score = {}",
            target, result.miss, result.counter, score);
        scores.push(score);
    }

    let avg = average_score(&scores);
    println!("# Fin du tour #");
    println!("→ Score moyen : {}", avg);
    avg
}

/// Demande au gagnant de choisir un poison à appliquer au perdant
pub fn choose_poison(winner: &Player, loser: &mut Player) {
    println!("\n{}, choisis un poison à appliquer à {} :",
        winner.name, loser.name);
    println!("→ 1: -5 speed");
    println!("→ 2: -5 strength");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => { loser.apply_poison(1); break; }
            "2" => { loser.apply_poison(2); break; }
            _   => println!("Entre 1 ou 2."),
        }
    }
}

/// Joue une manche complète entre deux joueurs
pub fn play_round(p1: &mut Player, p2: &mut Player, n_objectifs: usize) {
    let objectifs1 = generate_objectifs(n_objectifs);
    let objectifs2 = generate_objectifs(n_objectifs);

    let score1 = play_turn(p1, &objectifs1);
    let score2 = play_turn(p2, &objectifs2);

    if score1 > score2 {
        let diff = score1 - score2;
        println!("\n{} gagne la manche. {} perd {} points de vitalité.",
            p1.name, p2.name, diff);
        p2.take_damage(diff);
        choose_poison(p1, p2);
    } else if score2 > score1 {
        let diff = score2 - score1;
        println!("\n{} gagne la manche. {} perd {} points de vitalité.",
            p2.name, p1.name, diff);
        p1.take_damage(diff);
        choose_poison(p2, p1);
    } else {
        println!("\nÉgalité, personne ne perd de vitalité.");
    }
}

/// Boucle principale : joue jusqu'à ce qu'un joueur soit mort
pub fn run_game(mut p1: Player, mut p2: Player, n_objectifs: usize) {
    println!("\n##### Démarrage de la partie #####");
    let mut round = 1;

    loop {
        println!("\n## Manche {} ##", round);
        play_round(&mut p1, &mut p2, n_objectifs);
        println!("## FIN Manche {} ##", round);

        info!("fin manche {} — {}: vit={} | {}: vit={}",
            round, p1.name, p1.vitality, p2.name, p2.vitality);

        if p1.is_dead() || p2.is_dead() {
            break;
        }
        round += 1;
    }

    println!("\n##### Partie terminée #####");
    if p1.is_dead() {
        println!("{} gagne !", p2.name);
    } else {
        println!("{} gagne !", p1.name);
    }
}

/// Demande si les joueurs veulent relancer une partie
pub fn ask_replay() -> bool {
    loop {
        print!("\nRelancer une partie ? [Y/N] > ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "y" => return true,
            "n" => return false,
            _   => println!("Entre Y ou N."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_objectifs_len() {
        let obj = generate_objectifs(5);
        assert_eq!(obj.len(), 5);
    }

    #[test]
    fn test_generate_objectifs_range() {
        let obj = generate_objectifs(100);
        assert!(obj.iter().all(|&x| x <= 100));
    }
}