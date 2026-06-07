//! Nash equilibrium computation for normal-form games.

use crate::game::{NormalFormGame, PlayerIdx, StrategyIdx};
use crate::strategy::StrategyProfile;

/// A pure-strategy Nash equilibrium.
#[derive(Clone, Debug)]
pub struct NashEquilibrium {
    pub profile: StrategyProfile,
    pub payoffs: [f64; 2],
}

/// Find all pure-strategy Nash equilibria of a 2-player normal-form game.
///
/// A strategy profile is a Nash equilibrium if each player's strategy
/// is a best response to the other player's strategy.
pub fn find_pure_nash(game: &NormalFormGame) -> Vec<NashEquilibrium> {
    let mut equilibria = Vec::new();

    for (s0, s1) in game.all_profiles() {
        let br0 = best_responses_for(game, 0, s1);
        let br1 = best_responses_for(game, 1, s0);

        if br0.contains(&s0) && br1.contains(&s1) {
            let (p0, p1) = game.get_payoff(s0, s1);
            equilibria.push(NashEquilibrium {
                profile: StrategyProfile::new(s0, s1),
                payoffs: [p0, p1],
            });
        }
    }

    equilibria
}

/// Find best responses for a player given the opponent's strategy.
fn best_responses_for(
    game: &NormalFormGame,
    player: PlayerIdx,
    opponent_strategy: StrategyIdx,
) -> Vec<StrategyIdx> {
    let n = game.num_strategies(player);
    let mut best = Vec::new();
    let mut best_val = f64::NEG_INFINITY;

    for s in 0..n {
        let val = if player == 0 {
            game.player_payoff(player, s, opponent_strategy)
        } else {
            game.player_payoff(player, opponent_strategy, s)
        };
        if val > best_val + 1e-10 {
            best_val = val;
            best.clear();
            best.push(s);
        } else if (val - best_val).abs() < 1e-10 {
            best.push(s);
        }
    }
    best
}

/// Check if a given strategy profile is a Nash equilibrium.
pub fn is_nash_equilibrium(game: &NormalFormGame, s0: StrategyIdx, s1: StrategyIdx) -> bool {
    let br0 = best_responses_for(game, 0, s1);
    let br1 = best_responses_for(game, 1, s0);
    br0.contains(&s0) && br1.contains(&s1)
}

/// Find Nash equilibria that survive IESDS.
///
/// These are the equilibria whose strategies survive iterated elimination
/// of strictly dominated strategies.
pub fn nash_surviving_iesds(game: &NormalFormGame) -> Vec<NashEquilibrium> {
    let surviving = crate::dominance::iesds(game);
    let all_nash = find_pure_nash(game);

    all_nash
        .into_iter()
        .filter(|ne| {
            surviving[0].contains(&ne.profile.strategies[0])
                && surviving[1].contains(&ne.profile.strategies[1])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{prisoners_dilemma, battle_of_sexes, matching_pennies, stag_hunt, chicken};

    #[test]
    fn test_pd_nash_equilibrium() {
        let g = prisoners_dilemma();
        let ne = find_pure_nash(&g);
        assert_eq!(ne.len(), 1);
        assert_eq!(ne[0].profile.strategies, [1, 1]); // (D, D)
    }

    #[test]
    fn test_battle_of_sexes_two_nash() {
        let g = battle_of_sexes();
        let ne = find_pure_nash(&g);
        assert_eq!(ne.len(), 2);
    }

    #[test]
    fn test_matching_pennies_no_pure_nash() {
        let g = matching_pennies();
        let ne = find_pure_nash(&g);
        assert_eq!(ne.len(), 0); // No pure NE in zero-sum matching pennies
    }

    #[test]
    fn test_stag_hunt_two_nash() {
        let g = stag_hunt();
        let ne = find_pure_nash(&g);
        assert_eq!(ne.len(), 2); // (Stag,Stag) and (Hare,Hare)
    }

    #[test]
    fn test_chicken_two_nash() {
        let g = chicken();
        let ne = find_pure_nash(&g);
        assert_eq!(ne.len(), 2); // (Swerve,Straight) and (Straight,Swerve)
    }

    #[test]
    fn test_is_nash_equilibrium() {
        let g = prisoners_dilemma();
        assert!(is_nash_equilibrium(&g, 1, 1)); // (D,D) is NE
        assert!(!is_nash_equilibrium(&g, 0, 0)); // (C,C) is not NE
    }

    #[test]
    fn test_nash_surviving_iesds() {
        let g = prisoners_dilemma();
        let ne = nash_surviving_iesds(&g);
        assert_eq!(ne.len(), 1);
        assert_eq!(ne[0].profile.strategies, [1, 1]);
    }

    #[test]
    fn test_pd_nash_payoffs() {
        let g = prisoners_dilemma();
        let ne = find_pure_nash(&g);
        assert!((ne[0].payoffs[0] - (-2.0)).abs() < 1e-10);
        assert!((ne[0].payoffs[1] - (-2.0)).abs() < 1e-10);
    }
}
