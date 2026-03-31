use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::io::{self, BufRead};
use log::debug;

use crate::scoring::{circular_diff, compute_score};

/// Résultat d'un objectif : compteur figé, miss, score obtenu
pub struct ObjectifResult {
    pub counter: u32,
    pub miss: u32,
    pub score: i32,
}

/// Joue un objectif complet : lance le compteur, attend ENTREE, retourne le résultat
pub fn play_objectif(target: u32, strength: i32, speed: u64) -> ObjectifResult {
    // variables partagées entre les deux threads
    let counter = Arc::new(Mutex::new(0u32));
    let miss = Arc::new(Mutex::new(0u32));
    let running = Arc::new(Mutex::new(true));

    // clones pour le thread compteur
    let counter_t = Arc::clone(&counter);
    let miss_t = Arc::clone(&miss);
    let running_t = Arc::clone(&running);

    // thread compteur : tourne jusqu'à ce que running passe à false
    let handle = thread::spawn(move || {
        loop {
            {
                let run = running_t.lock().unwrap();
                if !*run {
                    break;
                }
            }
            thread::sleep(Duration::from_millis(speed));
            {
                let mut c = counter_t.lock().unwrap();
                let mut m = miss_t.lock().unwrap();
                *c += 1;
                if *c > 100 {
                    *c = 0;
                    *m += 1;
                }
            }
            // affiche le compteur sur la même ligne
            let c = counter_t.lock().unwrap();
            print!("\r  cible {:>3}  |  compteur [ {:>3} ]", target, *c);
            use std::io::Write;
            io::stdout().flush().unwrap();
        }
    });

    // thread principal : attend ENTREE
    let stdin = io::stdin();
    stdin.lock().lines().next();

    // stoppe le thread compteur
    {
        let mut run = running.lock().unwrap();
        *run = false;
    }
    handle.join().unwrap();

    // lit les valeurs finales
    let final_counter = *counter.lock().unwrap();
    let final_miss = *miss.lock().unwrap();

    let diff = circular_diff(final_counter, target);
    let score = compute_score(diff, strength, final_miss);

    debug!("objectif={} compteur={} miss={} diff={} score={}", target, final_counter, final_miss, diff, score);

    println!();
    ObjectifResult {
        counter: final_counter,
        miss: final_miss,
        score,
    }
}