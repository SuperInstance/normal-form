//! # normal-form
//!
//! A library for normal-form (strategic) games with payoff matrices,
//! dominant strategies, Pareto optimality, iterated elimination of
//! strictly dominated strategies, and Nash equilibrium computation.

pub mod game;
pub mod strategy;
pub mod dominance;
pub mod pareto;
pub mod nash;
