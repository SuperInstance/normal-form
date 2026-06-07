//! Dominant strategy detection and Iterated Elimination of Strictly Dominated Strategies.

use crate::game::{NormalFormGame, PlayerIdx, StrategyIdx};

/// Check if strategy `a` strictly dominates strategy `b` for a given player.
///
/// Strategy `a` strictly dominates `b` if `a` yields a strictly higher payoff
/// than `b` for every possible opponent strategy.
pub fn strictly_dominates(
    game: &NormalFormGame,
    player: PlayerIdx,
    a: StrategyIdx,
    b: StrategyIdx,
) -> bool {
    let opp: PlayerIdx = 1 - player;
    let n_opp = game.num_strategies(opp);
    let mut all_better = true;
    for os in 0..n_opp {
        let (s_a_0, s_a_1) = if player == 0 { (a, os) } else { (os, a) };
        let (s_b_0, s_b_1) = if player == 0 { (b, os) } else { (os, b) };
        let val_a = game.player_payoff(player, s_a_0, s_a_1);
        let val_b = game.player_payoff(player, s_b_0, s_b_1);
        if val_a <= val_b + 1e-10 {
            all_better = false;
            break;
        }
    }
    all_better
}

/// Check if strategy `a` weakly dominates strategy `b` for a given player.
///
/// Strategy `a` weakly dominates `b` if `a` yields at least as high a payoff
/// as `b` for every opponent strategy, and strictly higher for at least one.
pub fn weakly_dominates(
    game: &NormalFormGame,
    player: PlayerIdx,
    a: StrategyIdx,
    b: StrategyIdx,
) -> bool {
    let opp: PlayerIdx = 1 - player;
    let n_opp = game.num_strategies(opp);
    let mut all_geq = true;
    let mut some_gt = false;
    for os in 0..n_opp {
        let (s_a_0, s_a_1) = if player == 0 { (a, os) } else { (os, a) };
        let (s_b_0, s_b_1) = if player == 0 { (b, os) } else { (os, b) };
        let val_a = game.player_payoff(player, s_a_0, s_a_1);
        let val_b = game.player_payoff(player, s_b_0, s_b_1);
        if val_a < val_b - 1e-10 {
            all_geq = false;
            break;
        }
        if val_a > val_b + 1e-10 {
            some_gt = true;
        }
    }
    all_geq && some_gt
}

/// Find all strictly dominant strategies for a player.
///
/// A strategy is strictly dominant if it strictly dominates every other strategy.
pub fn find_strictly_dominant(
    game: &NormalFormGame,
    player: PlayerIdx,
) -> Option<StrategyIdx> {
    let n = game.num_strategies(player);
    'outer: for candidate in 0..n {
        for other in 0..n {
            if other == candidate {
                continue;
            }
            if !strictly_dominates(game, player, candidate, other) {
                continue 'outer;
            }
        }
        return Some(candidate);
    }
    None
}

/// Iterated Elimination of Strictly Dominated Strategies (IESDS).
///
/// Repeatedly removes strictly dominated strategies until none remain.
/// Returns the surviving strategy sets for each player.
pub fn iesds(game: &NormalFormGame) -> [Vec<StrategyIdx>; 2] {
    let mut surviving: [Vec<StrategyIdx>; 2] = [
        (0..game.n_strategies[0]).collect(),
        (0..game.n_strategies[1]).collect(),
    ];

    loop {
        let mut eliminated = false;

        for player in 0..2 {
            let opp: PlayerIdx = 1 - player;
            let opp_strats = &surviving[opp];
            let my_strats = &surviving[player];

            let mut to_remove = Vec::new();

            for &candidate in my_strats {
                let mut dominated = false;
                for &other in my_strats {
                    if other == candidate {
                        continue;
                    }
                    // Check strict dominance over surviving opponent strategies only
                    let dom = {
                        let mut all_better = true;
                        for &os in opp_strats.iter() {
                            let (s_a_0, s_a_1) = if player == 0 { (other, os) } else { (os, other) };
                            let (s_b_0, s_b_1) = if player == 0 { (candidate, os) } else { (os, candidate) };
                            let val_a = game.player_payoff(player, s_a_0, s_a_1);
                            let val_b = game.player_payoff(player, s_b_0, s_b_1);
                            if val_a <= val_b + 1e-10 {
                                all_better = false;
                                break;
                            }
                        }
                        all_better
                    };
                    if dom {
                        dominated = true;
                        break;
                    }
                }
                if dominated {
                    to_remove.push(candidate);
                    eliminated = true;
                }
            }

            surviving[player].retain(|s| !to_remove.contains(s));
        }

        if !eliminated {
            break;
        }
    }

    surviving
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::prisoners_dilemma;

    #[test]
    fn test_defect_strictly_dominates_cooperate_pd() {
        let g = prisoners_dilemma();
        // Defect (1) strictly dominates Cooperate (0) for player 0
        assert!(strictly_dominates(&g, 0, 1, 0));
        // Cooperate does NOT strictly dominate Defect
        assert!(!strictly_dominates(&g, 0, 0, 1));
    }

    #[test]
    fn test_strictly_dominant_strategy_pd() {
        let g = prisoners_dilemma();
        assert_eq!(find_strictly_dominant(&g, 0), Some(1)); // Defect
        assert_eq!(find_strictly_dominant(&g, 1), Some(1)); // Defect
    }

    #[test]
    fn test_no_dominant_strategy_bos() {
        let g = crate::game::battle_of_sexes();
        assert_eq!(find_strictly_dominant(&g, 0), None);
        assert_eq!(find_strictly_dominant(&g, 1), None);
    }

    #[test]
    fn test_iesds_pd() {
        let g = prisoners_dilemma();
        let surviving = iesds(&g);
        // IESDS should eliminate Cooperate, leaving only Defect
        assert_eq!(surviving[0], vec![1]);
        assert_eq!(surviving[1], vec![1]);
    }

    #[test]
    fn test_weakly_dominates() {
        let g = crate::game::stag_hunt();
        // In Stag Hunt, no strict dominance but check weak
        // Stag does not weakly dominate Hare or vice versa
        assert!(!weakly_dominates(&g, 0, 0, 1));
        assert!(!weakly_dominates(&g, 0, 1, 0));
    }
}
