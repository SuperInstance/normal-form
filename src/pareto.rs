//! Pareto optimality analysis.

use crate::game::{NormalFormGame, Payoff};
use crate::strategy::StrategyProfile;

/// A game outcome with its associated payoffs.
#[derive(Clone, Debug)]
pub struct Outcome {
    pub profile: StrategyProfile,
    pub payoffs: [Payoff; 2],
}

/// Check if outcome `a` Pareto dominates outcome `b`.
///
/// Outcome `a` Pareto dominates `b` if every player is at least as well off
/// in `a` as in `b`, and at least one player is strictly better off.
pub fn pareto_dominates(a: &[Payoff; 2], b: &[Payoff; 2]) -> bool {
    let mut all_geq = true;
    let mut some_gt = false;
    for i in 0..2 {
        if a[i] < b[i] - 1e-10 {
            all_geq = false;
            break;
        }
        if a[i] > b[i] + 1e-10 {
            some_gt = true;
        }
    }
    all_geq && some_gt
}

/// Find all Pareto optimal (Pareto efficient) outcomes in a game.
///
/// An outcome is Pareto optimal if no other outcome Pareto dominates it.
pub fn find_pareto_optimal(game: &NormalFormGame) -> Vec<Outcome> {
    let all = all_outcomes(game);
    let mut optimal = Vec::new();

    for outcome in &all {
        let mut dominated = false;
        for other in &all {
            if pareto_dominates(&other.payoffs, &outcome.payoffs) {
                dominated = true;
                break;
            }
        }
        if !dominated {
            optimal.push(outcome.clone());
        }
    }

    optimal
}

/// Find the outcome that maximizes the sum of payoffs (social welfare).
pub fn max_social_welfare(game: &NormalFormGame) -> Option<Outcome> {
    let all = all_outcomes(game);
    all.into_iter().max_by(|a, b| {
        let sum_a = a.payoffs[0] + a.payoffs[1];
        let sum_b = b.payoffs[0] + b.payoffs[1];
        sum_a.partial_cmp(&sum_b).unwrap()
    })
}

/// Get all outcomes of the game.
fn all_outcomes(game: &NormalFormGame) -> Vec<Outcome> {
    game.all_profiles()
        .into_iter()
        .map(|(s0, s1)| {
            let (p0, p1) = game.get_payoff(s0, s1);
            Outcome {
                profile: StrategyProfile::new(s0, s1),
                payoffs: [p0, p1],
            }
        })
        .collect()
}

/// Compute the Pareto frontier: the set of payoff vectors that are Pareto optimal.
pub fn pareto_frontier(game: &NormalFormGame) -> Vec<[Payoff; 2]> {
    find_pareto_optimal(game)
        .into_iter()
        .map(|o| o.payoffs)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::prisoners_dilemma;

    #[test]
    fn test_pareto_dominates_basic() {
        assert!(pareto_dominates(&[3.0, 3.0], &[2.0, 2.0]));
        assert!(pareto_dominates(&[3.0, 2.0], &[2.0, 2.0]));
        assert!(!pareto_dominates(&[2.0, 3.0], &[3.0, 2.0])); // incomparable
    }

    #[test]
    fn test_pd_pareto_optimal() {
        let g = prisoners_dilemma();
        let opt = find_pareto_optimal(&g);
        // In PD, (Cooperate, Cooperate) is Pareto optimal (sum -2 beats -4, -3)
        // and (Defect, Cooperate), (Cooperate, Defect) are also Pareto optimal
        assert!(opt.len() >= 1);
        let payoffs: Vec<[f64; 2]> = opt.iter().map(|o| o.payoffs).collect();
        // (C,C) = (-1,-1) should be Pareto optimal
        assert!(payoffs.iter().any(|p| (p[0] - (-1.0)).abs() < 1e-10 && (p[1] - (-1.0)).abs() < 1e-10));
    }

    #[test]
    fn test_pd_not_pareto_optimal_dd() {
        let g = prisoners_dilemma();
        let opt = find_pareto_optimal(&g);
        let payoffs: Vec<[f64; 2]> = opt.iter().map(|o| o.payoffs).collect();
        // (D,D) = (-2,-2) is NOT Pareto optimal because (C,C) = (-1,-1) dominates it
        assert!(!payoffs.iter().any(|p| (p[0] - (-2.0)).abs() < 1e-10 && (p[1] - (-2.0)).abs() < 1e-10));
    }

    #[test]
    fn test_max_social_welfare_pd() {
        let g = prisoners_dilemma();
        let best = max_social_welfare(&g).unwrap();
        // (C,C) has sum -2, the highest
        assert!((best.payoffs[0] - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_battle_of_sexes_pareto() {
        let g = crate::game::battle_of_sexes();
        let opt = find_pareto_optimal(&g);
        // Both (Opera,Opera) and (Football,Football) are Pareto optimal
        assert!(opt.len() >= 2);
    }
}
