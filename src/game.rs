//! Core game types and construction.

use std::fmt;

/// A payoff for a player in a game outcome.
pub type Payoff = f64;

/// A strategy index for a player.
pub type StrategyIdx = usize;

/// A player index.
pub type PlayerIdx = usize;

/// A strategy profile: one strategy per player.
pub type Profile = Vec<StrategyIdx>;

/// A payoff vector: one payoff per player.
pub type PayoffVector = Vec<Payoff>;

/// A 2-player normal-form game represented as a payoff matrix.
///
/// The matrix is indexed as `[row_player_strategy][col_player_strategy]`
/// where each entry is `(row_payoff, col_payoff)`.
#[derive(Clone, Debug)]
pub struct NormalFormGame {
    /// Number of strategies for player 0 (row player).
    pub n_strategies: [usize; 2],
    /// Payoff matrix: `payoffs[s0][s1] = (payoff_0, payoff_1)`.
    pub payoffs: Vec<Vec<(Payoff, Payoff)>>,
    /// Strategy labels for player 0.
    pub labels_0: Vec<String>,
    /// Strategy labels for player 1.
    pub labels_1: Vec<String>,
}

impl NormalFormGame {
    /// Create a new 2-player normal-form game from a payoff matrix.
    pub fn new_2x2(
        payoffs: [[(Payoff, Payoff); 2]; 2],
    ) -> Self {
        Self {
            n_strategies: [2, 2],
            payoffs: payoffs.iter().map(|row| row.to_vec()).collect(),
            labels_0: vec!["S0".into(), "S1".into()],
            labels_1: vec!["S0".into(), "S1".into()],
        }
    }

    /// Create a game from an arbitrary-sized payoff matrix.
    pub fn new(
        payoffs: Vec<Vec<(Payoff, Payoff)>>,
    ) -> Self {
        let n0 = payoffs.len();
        let n1 = if n0 > 0 { payoffs[0].len() } else { 0 };
        Self {
            n_strategies: [n0, n1],
            payoffs,
            labels_0: (0..n0).map(|i| format!("S{i}")).collect(),
            labels_1: (0..n1).map(|j| format!("S{j}")).collect(),
        }
    }

    /// Create a game with labeled strategies.
    pub fn with_labels(
        payoffs: Vec<Vec<(Payoff, Payoff)>>,
        labels_0: Vec<&str>,
        labels_1: Vec<&str>,
    ) -> Self {
        let n0 = payoffs.len();
        let n1 = if n0 > 0 { payoffs[0].len() } else { 0 };
        Self {
            n_strategies: [n0, n1],
            payoffs,
            labels_0: labels_0.iter().map(|s| s.to_string()).collect(),
            labels_1: labels_1.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Get the payoff for a given strategy profile.
    pub fn get_payoff(&self, s0: StrategyIdx, s1: StrategyIdx) -> (Payoff, Payoff) {
        self.payoffs[s0][s1]
    }

    /// Get the payoff for a player given a strategy profile.
    pub fn player_payoff(&self, player: PlayerIdx, s0: StrategyIdx, s1: StrategyIdx) -> Payoff {
        let (p0, p1) = self.payoffs[s0][s1];
        if player == 0 { p0 } else { p1 }
    }

    /// Number of strategies for a given player.
    pub fn num_strategies(&self, player: PlayerIdx) -> usize {
        self.n_strategies[player]
    }

    /// Iterate over all strategy profiles.
    pub fn all_profiles(&self) -> Vec<(StrategyIdx, StrategyIdx)> {
        let mut profiles = Vec::new();
        for s0 in 0..self.n_strategies[0] {
            for s1 in 0..self.n_strategies[1] {
                profiles.push((s0, s1));
            }
        }
        profiles
    }
}

impl fmt::Display for NormalFormGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Normal-Form Game ({}x{}):", self.n_strategies[0], self.n_strategies[1])?;
        write!(f, "{:>12}", "")?;
        for l1 in &self.labels_1 {
            write!(f, "{:>12}", l1)?;
        }
        writeln!(f)?;
        for (i, row) in self.payoffs.iter().enumerate() {
            write!(f, "{:>12}", self.labels_0[i])?;
            for (p0, p1) in row {
                write!(f, " ({}, {})", p0, p1)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Prisoner's Dilemma: classic 2x2 game.
pub fn prisoners_dilemma() -> NormalFormGame {
    NormalFormGame::with_labels(
        vec![
            vec![(-1.0, -1.0), (-3.0, 0.0)],
            vec![(0.0, -3.0), (-2.0, -2.0)],
        ],
        vec!["Cooperate", "Defect"],
        vec!["Cooperate", "Defect"],
    )
}

/// Battle of the Sexes: coordination game.
pub fn battle_of_sexes() -> NormalFormGame {
    NormalFormGame::with_labels(
        vec![
            vec![(3.0, 2.0), (0.0, 0.0)],
            vec![(0.0, 0.0), (2.0, 3.0)],
        ],
        vec!["Opera", "Football"],
        vec!["Opera", "Football"],
    )
}

/// Matching Pennies: zero-sum game.
pub fn matching_pennies() -> NormalFormGame {
    NormalFormGame::with_labels(
        vec![
            vec![(1.0, -1.0), (-1.0, 1.0)],
            vec![(-1.0, 1.0), (1.0, -1.0)],
        ],
        vec!["Heads", "Tails"],
        vec!["Heads", "Tails"],
    )
}

/// Stag Hunt: assurance game.
pub fn stag_hunt() -> NormalFormGame {
    NormalFormGame::with_labels(
        vec![
            vec![(4.0, 4.0), (0.0, 3.0)],
            vec![(3.0, 0.0), (2.0, 2.0)],
        ],
        vec!["Stag", "Hare"],
        vec!["Stag", "Hare"],
    )
}

/// Chicken (Hawk-Dove): anti-coordination game.
pub fn chicken() -> NormalFormGame {
    NormalFormGame::with_labels(
        vec![
            vec![(0.0, 0.0), (-1.0, 1.0)],
            vec![(1.0, -1.0), (-10.0, -10.0)],
        ],
        vec!["Swerve", "Straight"],
        vec!["Swerve", "Straight"],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prisoners_dilemma_payoffs() {
        let g = prisoners_dilemma();
        assert_eq!(g.get_payoff(0, 0), (-1.0, -1.0)); // (C, C)
        assert_eq!(g.get_payoff(1, 1), (-2.0, -2.0)); // (D, D)
        assert_eq!(g.get_payoff(0, 1), (-3.0, 0.0));  // (C, D)
        assert_eq!(g.get_payoff(1, 0), (0.0, -3.0));  // (D, C)
    }

    #[test]
    fn test_battle_of_sexes_payoffs() {
        let g = battle_of_sexes();
        assert_eq!(g.get_payoff(0, 0), (3.0, 2.0));
        assert_eq!(g.get_payoff(1, 1), (2.0, 3.0));
    }

    #[test]
    fn test_matching_pennies_zero_sum() {
        let g = matching_pennies();
        for s0 in 0..2 {
            for s1 in 0..2 {
                let (p0, p1) = g.get_payoff(s0, s1);
                assert!((p0 + p1).abs() < 1e-10, "Not zero-sum at ({}, {})", s0, s1);
            }
        }
    }

    #[test]
    fn test_all_profiles_2x2() {
        let g = prisoners_dilemma();
        let profiles = g.all_profiles();
        assert_eq!(profiles.len(), 4);
    }

    #[test]
    fn test_game_display() {
        let g = prisoners_dilemma();
        let s = format!("{}", g);
        assert!(s.contains("Normal-Form Game"));
        assert!(s.contains("Cooperate"));
        assert!(s.contains("Defect"));
    }
}
