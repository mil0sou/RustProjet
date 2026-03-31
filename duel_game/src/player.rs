/// Représente un joueur avec ses caractéristiques
#[derive(Debug, Clone)]
pub struct Player {
    /// Nom du joueur
    pub name: String,
    /// Points de vie
    pub vitality: i32,
    /// Pas d'incrémentation du compteur en ms
    pub speed: u64,
    /// Bonus de score à chaque objectif
    pub strength: i32,
}

impl Player {
    /// Crée un nouveau joueur avec des valeurs par défaut
    pub fn new(name: String, vitality: i32) -> Self {
        Player {
            name,
            vitality,
            speed: 50,
            strength: 50,
        }
    }

    /// Applique des dégâts de vitalité au joueur
    pub fn take_damage(&mut self, amount: i32) {
        self.vitality -= amount;
        if self.vitality < 0 {
            self.vitality = 0;
        }
    }

    /// Applique un poison : réduit speed ou strength de 5
    pub fn apply_poison(&mut self, choice: u8) {
        match choice {
            1 => self.speed += 5,   // speed en ms donc augmenter = ralentir
            2 => self.strength -= 5,
            _ => {}
        }
    }

    /// Retourne true si le joueur est mort
    pub fn is_dead(&self) -> bool {
        self.vitality <= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_player() {
        let p = Player::new("Michel".to_string(), 50);
        assert_eq!(p.vitality, 50);
        assert_eq!(p.speed, 50);
        assert_eq!(p.strength, 50);
    }

    #[test]
    fn test_take_damage() {
        let mut p = Player::new("Michel".to_string(), 50);
        p.take_damage(20);
        assert_eq!(p.vitality, 30);
    }

    #[test]
    fn test_vitality_not_negative() {
        let mut p = Player::new("Michel".to_string(), 50);
        p.take_damage(999);
        assert_eq!(p.vitality, 0);
    }

    #[test]
    fn test_apply_poison_speed() {
        let mut p = Player::new("Michel".to_string(), 50);
        p.apply_poison(1);
        assert_eq!(p.speed, 55);
    }

    #[test]
    fn test_apply_poison_strength() {
        let mut p = Player::new("Michel".to_string(), 50);
        p.apply_poison(2);
        assert_eq!(p.strength, 45);
    }

    #[test]
    fn test_is_dead() {
        let mut p = Player::new("Michel".to_string(), 50);
        assert!(!p.is_dead());
        p.take_damage(50);
        assert!(p.is_dead());
    }
}