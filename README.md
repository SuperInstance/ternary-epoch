# Ternary Epoch — Epoch Detection and Transition Analysis for Ternary Time Series

**Ternary Epoch** detects epochs (periods of stable state) in ternary-valued time series, analyzes transitions between epochs, and builds Markov transition matrices. It segments history into runs where a single ternary value {-1, 0, +1} dominates, measures epoch durations, and computes the probability of transitioning between states.

## Why It Matters

Complex systems pass through phases — stable periods punctuated by transitions. Detecting these epochs in ternary time series (agent states, GPU utilization, consensus outcomes) is essential for understanding system dynamics. The transition matrix reveals whether the system is bistable (oscillating between two states), tri-stable (cycling through all three), or monostable (stuck in one state). In fleet management, epoch analysis answers questions like "how often does the fleet enter the neutral (0) state?" and "what's the probability of recovering from a -1 epoch?" — questions that directly inform fleet health assessment.

## How It Works

### Epoch Detection

Scans the history for runs of the same value. A run becomes an epoch if it lasts at least `min_length` ticks:

```
detect_epochs(history, min_length):
  for each run of identical values:
    if run.length >= min_length:
      emit Epoch { start_tick, dominant_state, stability: 1.0 }
```

This is a single-pass O(n) algorithm. The `min_length` filter suppresses noise — transient blips that don't represent true state changes.

### Boundary Detection

`epoch_boundary()` returns the indices where the value changes. This is O(n) and useful for identifying exact transition points for further analysis.

### Transition Matrix

From the sequence of epochs, builds a 3×3 Markov chain:

```
         to -1   to 0   to +1
from -1 [ p₀₀   p₀₁    p₀₂  ]
from  0 [ p₁₀   p₁₁    p₁₂  ]
from +1 [ p₂₀   p₂₁    p₂₂  ]
```

where pᵢⱼ = count(i→j) / count(i→*). This reveals the system's state dynamics: are transitions balanced (uniform) or biased (e.g., always exit 0 toward +1)?

### Epoch Duration

Measures how long each epoch lasts. Long durations indicate stability; short durations indicate turbulence. The distribution of durations follows a power law for critical systems and is exponential for chaotic systems.

### Current Epoch Lookup

`current_epoch(epochs, tick)` finds the most recent epoch whose start ≤ tick — O(e) for e epochs, or O(log e) with binary search.

## Quick Start

```rust
use ternary_epoch::{detect_epochs, transition_matrix, epoch_boundary};

let history: Vec<i8> = vec![1, 1, 1, -1, -1, 0, 0, 0, 0, 1, 1];
let epochs = detect_epochs(&history, 2); // min 2 ticks per epoch
let boundaries = epoch_boundary(&history);
let matrix = transition_matrix(&epochs);

println!("Detected {} epochs with {} transitions", epochs.len(), boundaries.len());
```

```bash
cargo add ternary-epoch
```

## API

| Type / Function | Description |
|---|---|
| `Epoch` | `{ start_tick, dominant_state, stability }` |
| `detect_epochs(history, min_length)` | Segment into stable runs (O(n)) |
| `epoch_boundary(history)` | Indices where value changes |
| `transition_matrix(epochs)` | 3×3 Markov chain (O(e)) |
| `epoch_duration(epochs)` | Duration of each epoch |
| `current_epoch(epochs, tick)` | Most recent epoch at given tick |

## Architecture Notes

Epoch analysis reveals the long-term behavior of **SuperInstance** fleets. Healthy fleets show balanced transitions across {-1, 0, +1}; degraded fleets fixate on one state. The γ + η = C conservation law predicts that epochs of high γ (growth) alternate with epochs of high η (entropy) — the transition matrix captures this rhythm. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

- Norris, J. R. *Markov Chains*, Cambridge UP, 1997 — Markov chain theory.
- Kantz, Holger & Schreiber, Thomas. *Nonlinear Time Series Analysis*, 2nd ed., Cambridge UP, 2004.
- Gershenfeld, Neil. *The Nature of Mathematical Modeling*, Cambridge UP, 1998 — epoch detection and segmentation.

## License

MIT
