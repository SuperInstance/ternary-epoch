# ternary-epoch

Epoch detection and lifecycle management for ternary state histories.

## Why This Exists

During ternary training or simulation, you observe sequences of {-1, 0, +1} values over time. These sequences naturally form epochs — periods where the state stays the same before transitioning. Detecting epoch boundaries, computing durations, and analyzing transition patterns between states is essential for understanding training dynamics, convergence behavior, and system stability. This crate provides lightweight epoch detection on raw ternary histories.

## Architecture

### Core Types

- **`Epoch`** — A time segment: `start_tick`, `end_tick`, `state` (i8).
- **`detect_epochs`** — Scans a history array and returns contiguous segments of the same value.
- **`epoch_boundary`** — Returns tick indices where state changes.
- **`transition_matrix`** — Builds a 3×3 Markov transition probability matrix from epoch sequences.

## Usage

```rust
use ternary_epoch::*;

let history = [1, 1, 1, 0, 0, -1, -1, -1, -1, 0i8];

let epochs = detect_epochs(&history, 1);
// [Epoch{1, 2, 1}, Epoch{3, 4, 0}, Epoch{5, 8, -1}, Epoch{9, 9, 0}]

let boundaries = epoch_boundary(&history);
// [3, 5, 9] — ticks where state changed

let transitions = transition_matrix(&epochs);
// P(state_j | state_i) — useful for Markov chain analysis

// Find current epoch at a given tick
let current = current_epoch(&epochs, 6);
assert_eq!(current.unwrap().state, -1);

let durations = epoch_duration(&epochs);
```

## API Reference

| Function | Returns | Description |
|----------|---------|-------------|
| `new(tick, state)` | `Epoch` | Create an epoch |
| `detect_epochs(history, min_length)` | `Vec<Epoch>` | Find contiguous state segments |
| `epoch_boundary(history)` | `Vec<usize>` | Indices where state changes |
| `current_epoch(epochs, tick)` | `Option<&Epoch>` | Epoch at given tick |
| `epoch_duration(epochs)` | `Vec<u64>` | Duration of each epoch |
| `transition_matrix(epochs)` | `[[f64; 3]; 3]` | Markov transition probabilities |

## The Deeper Idea

Epoch detection is **change-point analysis for ternary time series**. In continuous-valued time series, change-point detection is hard (you need statistical tests). In ternary time series, it's trivial: the value changed or it didn't. This means you can do real-time epoch tracking with zero computation — just compare current to previous. The transition matrix then gives you the full Markov model of your system's dynamics, which is useful for predicting how long the current state will persist and when to expect the next transition.

## Related Crates

- **ternary-accumulator** — gradient accumulation for ternary training
- **ternary-chaos** — chaos theory and dynamics in ternary systems
- **ternary-automata** — cellular automata with ternary states
