/// Calcule la différence circulaire entre le compteur et l'objectif
/// Ex : objectif=5, compteur=95 -> différence de 10, pas 90
pub fn circular_diff(counter: u32, target: u32) -> u32 {
    let diff = (counter as i32 - target as i32).abs() as u32;
    diff.min(100 - diff)
}

/// Calcule le score pour un objectif donné
pub fn compute_score(diff: u32, strength: i32, miss: u32) -> i32 {
    let base = match diff {
        0       => 100,
        1..=5   => 80,
        6..=10  => 60,
        11..=20 => 40,
        21..=40 => 20,
        _       => 0,
    };
    (base + strength) / (miss as i32 + 1)
}

/// Calcule le score moyen d'un tour, arrondi à l'entier supérieur
pub fn average_score(scores: &[i32]) -> i32 {
    if scores.is_empty() {
        return 0;
    }
    let sum: i32 = scores.iter().sum();
    let len = scores.len() as i32;
    (sum + len - 1) / len  // arrondi supérieur sans float
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_diff_normal() {
        assert_eq!(circular_diff(28, 30), 2);
    }

    #[test]
    fn test_circular_diff_wraparound() {
        assert_eq!(circular_diff(95, 5), 10);
    }

    #[test]
    fn test_circular_diff_zero() {
        assert_eq!(circular_diff(50, 50), 0);
    }

    #[test]
    fn test_compute_score_perfect() {
        assert_eq!(compute_score(0, 50, 0), 150);
    }

    #[test]
    fn test_compute_score_with_miss() {
        assert_eq!(compute_score(0, 50, 1), 75);
    }

    #[test]
    fn test_compute_score_far() {
        assert_eq!(compute_score(50, 50, 0), 50);
    }

    #[test]
    fn test_average_score_round_up() {
        assert_eq!(average_score(&[45, 130, 130, 55, 65]), 85);
    }

    #[test]
    fn test_average_score_empty() {
        assert_eq!(average_score(&[]), 0);
    }
}