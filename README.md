# normal-form

A pure-Rust library for **normal-form (strategic) games** with zero external dependencies.

## Features

- **Payoff matrices** — Define arbitrary N-player normal-form games with integer or float payoffs
- **Dominant strategies** — Detect strictly and weakly dominant strategies
- **Iterated Elimination of Strictly Dominated Strategies (IESDS)** — Reduce games by removing dominated strategies
- **Pareto optimality** — Find Pareto-efficient outcomes
- **Nash equilibrium** — Compute pure-strategy Nash equilibria via best-response analysis

## Modules

| Module | Description |
|---|---|
| `game` | Core `NormalFormGame` type and construction |
| `strategy` | Strategy profiles and payoff extraction |
| `dominance` | Dominant strategy detection and IESDS |
| `pareto` | Pareto optimality analysis |
| `nash` | Nash equilibrium computation |

## Quick Start

```rust
use normal_form::game::NormalFormGame;

// Prisoner's Dilemma
let pd = NormalFormGame::new_2x2(
    [[(-1, -1), (-3,  0)],   // Cooperate
     [( 0, -3), (-2, -2)]],  // Defect
);

// Find Nash equilibria
let eq = normal_form::nash::find_pure_nash(&pd);
assert_eq!(eq.len(), 1); // (Defect, Defect)
```

## License

MIT OR Apache-2.0
