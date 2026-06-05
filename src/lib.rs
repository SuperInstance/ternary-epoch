#![forbid(unsafe_code)]

#[derive(Clone, Debug, PartialEq)]
pub struct Epoch {
    pub start_tick: u64,
    pub dominant_state: i8,
    pub stability: f64,
}

pub fn new(tick: u64, state: i8) -> Epoch {
    Epoch { start_tick: tick, dominant_state: state, stability: 1.0 }
}

pub fn detect_epochs(history: &[i8], min_length: usize) -> Vec<Epoch> {
    if history.is_empty() { return vec![]; }
    let mut epochs = Vec::new();
    let mut start = 0;
    let mut current = history[0];

    for (i, &v) in history.iter().enumerate() {
        if v != current {
            if i - start >= min_length {
                let stability = 1.0; // pure run
                epochs.push(Epoch { start_tick: start as u64, dominant_state: current, stability });
            }
            current = v;
            start = i;
        }
    }
    // flush last
    if history.len() - start >= min_length {
        epochs.push(Epoch { start_tick: start as u64, dominant_state: current, stability: 1.0 });
    }
    epochs
}

pub fn epoch_boundary(history: &[i8]) -> Vec<usize> {
    if history.len() < 2 { return vec![]; }
    history.windows(2)
        .enumerate()
        .filter_map(|(i, w)| if w[0] != w[1] { Some(i + 1) } else { None })
        .collect()
}

pub fn current_epoch<'a>(epochs: &'a [Epoch], tick: u64) -> Option<&'a Epoch> {
    // Find the last epoch whose start_tick <= tick
    epochs.iter().rev().find(|e| e.start_tick <= tick)
}

pub fn epoch_duration(epochs: &[Epoch]) -> Vec<u64> {
    epochs.windows(2)
        .map(|w| w[1].start_tick - w[0].start_tick)
        .chain(epochs.last().map(|_| 0u64)) // last epoch has unknown duration, report 0
        .collect()
}

pub fn transition_matrix(epochs: &[Epoch]) -> [[f64; 3]; 3] {
    // states: -1→0, 0→1, 1→2
    let mut counts = [[0usize; 3]; 3];
    for w in epochs.windows(2) {
        let from = state_idx(w[0].dominant_state);
        let to = state_idx(w[1].dominant_state);
        counts[from][to] += 1;
    }
    let mut matrix = [[0.0f64; 3]; 3];
    for i in 0..3 {
        let row_sum: usize = counts[i].iter().sum();
        if row_sum > 0 {
            for j in 0..3 {
                matrix[i][j] = counts[i][j] as f64 / row_sum as f64;
            }
        }
    }
    matrix
}

fn state_idx(s: i8) -> usize {
    match s {
        -1 => 0,
        0 => 1,
        1 => 2,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_epoch() {
        let e = new(5, 1);
        assert_eq!(e.start_tick, 5);
        assert_eq!(e.dominant_state, 1);
    }

    #[test]
    fn test_detect_epochs_simple() {
        let epochs = detect_epochs(&[-1, -1, -1, 1, 1, 1], 2);
        assert_eq!(epochs.len(), 2);
        assert_eq!(epochs[0].dominant_state, -1);
        assert_eq!(epochs[1].dominant_state, 1);
    }

    #[test]
    fn test_detect_epochs_min_length_filters() {
        let epochs = detect_epochs(&[-1, 1, -1, -1, -1], 2);
        assert_eq!(epochs.len(), 1);
        assert_eq!(epochs[0].dominant_state, -1);
    }

    #[test]
    fn test_detect_epochs_empty() {
        assert!(detect_epochs(&[], 1).is_empty());
    }

    #[test]
    fn test_epoch_boundary() {
        let bounds = epoch_boundary(&[-1, -1, 0, 0, 1]);
        assert_eq!(bounds, vec![2, 4]);
    }

    #[test]
    fn test_epoch_boundary_flat() {
        let bounds = epoch_boundary(&[0, 0, 0]);
        assert!(bounds.is_empty());
    }

    #[test]
    fn test_current_epoch_middle() {
        let epochs = vec![new(0, -1), new(10, 0), new(20, 1)];
        let e = current_epoch(&epochs, 15).unwrap();
        assert_eq!(e.dominant_state, 0);
    }

    #[test]
    fn test_current_epoch_before() {
        let epochs = vec![new(5, 1)];
        assert!(current_epoch(&epochs, 3).is_none());
    }

    #[test]
    fn test_epoch_duration() {
        let epochs = vec![new(0, -1), new(5, 0), new(12, 1)];
        let durs = epoch_duration(&epochs);
        assert_eq!(durs, vec![5, 7, 0]);
    }

    #[test]
    fn test_transition_matrix_perfect() {
        let epochs = vec![new(0, -1), new(5, 0), new(10, 1)];
        let m = transition_matrix(&epochs);
        assert!((m[0][1] - 1.0).abs() < 1e-9); // -1 → 0 with prob 1
        assert!((m[1][2] - 1.0).abs() < 1e-9); // 0 → 1 with prob 1
        assert!((m[0][0] - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_transition_matrix_empty() {
        let m = transition_matrix(&[]);
        assert_eq!(m, [[0.0; 3]; 3]);
    }

    #[test]
    fn test_epoch_boundary_single() {
        assert!(epoch_boundary(&[1]).is_empty());
    }

    #[test]
    fn test_detect_epochs_single_run() {
        let epochs = detect_epochs(&[0, 0, 0, 0], 2);
        assert_eq!(epochs.len(), 1);
        assert_eq!(epochs[0].dominant_state, 0);
    }

    #[test]
    fn test_current_epoch_exact() {
        let epochs = vec![new(0, -1), new(10, 1)];
        let e = current_epoch(&epochs, 10).unwrap();
        assert_eq!(e.dominant_state, 1);
    }

    #[test]
    fn test_detect_epochs_all_same() {
        let epochs = detect_epochs(&[1, 1, 1, 1, 1], 1);
        assert_eq!(epochs.len(), 1);
        assert_eq!(epochs[0].start_tick, 0);
        assert_eq!(epochs[0].dominant_state, 1);
    }
}
