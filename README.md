# ternary-epoch

**Detect regime changes in ternary signals. When did the era change?**

History isn't smooth — it has *epochs*. Long stretches of stability punctuated by sudden transitions. A stock market has bull and bear epochs. A conversation has topics. A system has operational modes. This crate detects those boundaries in ternary signals: runs of `+1`, stretches of `0`, periods of `-1`, and the exact tick where one era ends and another begins.

## What's Inside

- **`Epoch`** — a time period with `start_tick`, `dominant_state`, and `stability` measure
- **`detect_epochs(history, min_length)`** — find stable runs in a ternary signal. Minimum length filters noise
- **`epoch_boundary(history)`** — find the exact indices where state changes
- **`current_epoch(epochs, tick)`** — given a tick, which epoch are we in?
- **`epoch_duration(epochs)`** — how long did each era last?
- **`transition_matrix(epochs)`** — build a 3×3 Markov transition matrix from epoch sequence. How often does `-1` → `+1`? Does `0` always follow `+1`?

## Quick Example

```rust
use ternary_epoch::*;

// A signal with clear regime changes
let history: Vec<i8> = vec![1,1,1,1, -1,-1,-1, 0,0,0,0,0, 1,1];

let epochs = detect_epochs(&history, 2);
// Four epochs: [+1], [-1], [0], [+1]

let boundaries = epoch_boundary(&history);
// [4, 7, 12] — where transitions happen

// Which epoch is tick 5 in?
let current = current_epoch(&epochs, 5);
assert_eq!(current.unwrap().dominant_state, -1);

// How often does each transition happen?
let matrix = transition_matrix(&epochs);
// matrix[i][j] = P(state_j | state_i)
// Discover hidden dynamics in your signal
```

## The Insight

**Epochs reveal timescale structure.** A ternary signal might look random tick-by-tick, but at the epoch level, it tells a story: first expansion (+1), then contraction (-1), then stagnation (0), then expansion again. The transition matrix captures the *grammar* of that story — which regimes follow which, and how predictable the sequence is.

**Use cases:**
- **Time series analysis** — detect regime changes in discretized signals
- **System monitoring** — when did the server enter error mode? When did it recover?
- **Market microstructure** — bull/bear/flat regime detection
- **Conversation analysis** — topic boundaries in dialogue systems
- **Game AI** — detect when the player's strategy changes epoch

## See Also

- **ternary-markov** — Markov chain prediction on ternary sequences
- **ternary-dynamics** — full dynamical systems analysis
- **ternary-entropy** — information content at epoch boundaries
- **ternary-loop** — periodic epoch patterns

## Install

```bash
cargo add ternary-epoch
```

## License

MIT
