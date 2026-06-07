//! Strategy profiles and payoff extraction.

use crate::game::{NormalFormGame, Payoff, PlayerIdx, StrategyIdx};

/// A pure strategy profile for a 2-player game.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StrategyProfile {
    pub strategies: [StrategyIdx; 2],
}

impl StrategyProfile {
    /// Create a new strategy profile.
    pub fn new(s0: StrategyIdx, s1: StrategyIdx) -> Self {
        Self { strategies: [s0, s1] }
    }

    /// Get the strategy for a player.
    pub fn strategy(&self, player: PlayerIdx) -> StrategyIdx {
        self.strategies[player]
    }
}

/// Compute the payoff for each player under a given strategy profile.
pub fn profile_payoffs(game: &NormalFormGame, profile: &StrategyProfile) -> [Payoff; 2] {
    let (p0, p1) = game.get_payoff(profile.strategies[0], profile.strategies[1]);
    [p0, p1]
}

/// Compute the payoff for a single player under a given profile.
pub fn player_payoff_in_profile(
    game: &NormalFormGame,
    player: PlayerIdx,
    profile: &StrategyProfile,
) -> Payoff {
    let payoffs = profile_payoffs(game, profile);
    payoffs[player]
}

/// Find the best response for a player given the opponent's strategy.
///
/// Returns all strategies that maximize the player's payoff (may be multiple if tied).
pub fn best_responses(
    game: &NormalFormGame,
    player: PlayerIdx,
    opponent_strategy: StrategyIdx,
) -> Vec<StrategyIdx> {
    let _opponent: PlayerIdx = 1 - player;
    let n = game.num_strategies(player);
    let mut best = Vec::new();
    let mut best_val = f64::NEG_INFINITY;

    for s in 0..n {
        let (s0, s1) = if player == 0 {
            (s, opponent_strategy)
        } else {
            (opponent_strategy, s)
        };
        let val = game.player_payoff(player, s0, s1);
        if val > best_val {
            best_val = val;
            best.clear();
            best.push(s);
        } else if (val - best_val).abs() < 1e-10 {
            best.push(s);
        }
    }
    best
}

/// Check if a strategy is a best response to the opponent's play.
pub fn is_best_response(
    game: &NormalFormGame,
    player: PlayerIdx,
    strategy: StrategyIdx,
    opponent_strategy: StrategyIdx,
) -> bool {
    best_responses(game, player, opponent_strategy).contains(&strategy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::prisoners_dilemma;

    #[test]
    fn test_strategy_profile_creation() {
        let p = StrategyProfile::new(0, 1);
        assert_eq!(p.strategy(0), 0);
        assert_eq!(p.strategy(1), 1);
    }

    #[test]
    fn test_profile_payoffs_pd() {
        let g = prisoners_dilemma();
        let p = StrategyProfile::new(1, 1); // (Defect, Defect)
        let payoffs = profile_payoffs(&g, &p);
        assert!((payoffs[0] - (-2.0)).abs() < 1e-10);
        assert!((payoffs[1] - (-2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_best_response_pd_defect() {
        let g = prisoners_dilemma();
        // In PD, Defect is always a best response
        let br0 = best_responses(&g, 0, 0); // P0's BR to P1 playing C
        assert_eq!(br0, vec![1]); // Defect
        let br1 = best_responses(&g, 1, 0); // P1's BR to P0 playing C
        assert_eq!(br1, vec![1]); // Defect
    }

    #[test]
    fn test_best_response_pd_always_defect() {
        let g = prisoners_dilemma();
        let br0 = best_responses(&g, 0, 1); // P0's BR to P1 playing D
        assert_eq!(br0, vec![1]); // Defect
        let br1 = best_responses(&g, 1, 1); // P1's BR to P0 playing D
        assert_eq!(br1, vec![1]); // Defect
    }
}
