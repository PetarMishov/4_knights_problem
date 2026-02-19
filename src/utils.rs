use crate::utils;

pub fn iter_bits(mut mask: u64) -> impl Iterator<Item = u8> {
    std::iter::from_fn(move || {
        if mask == 0 {
            return None;
        }
        let lsb = mask & mask.wrapping_neg();
        let idx = lsb.trailing_zeros() as u8;
        mask &= mask - 1;
        Some(idx)
    })
}

pub fn add_edges(
    edges: &mut Vec<(u8, u8)>,
    knight_mask: u64,
    valid_position_mask: u64,
    bounds: (u8, u8), // (rows, cols)
) -> u64 {
    let rows = bounds.0 as i16;
    let cols = bounds.1 as i16;
    let deltas: [(i16, i16); 8] = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];

    let mut seen: u64 = knight_mask & valid_position_mask;
    let mut jump_from: u64 = seen;

    while jump_from != 0 {
        let mut next_jump_from: u64 = 0;

        for from in utils::iter_bits(jump_from) {
            let from_i16 = from as i16;
            let r = from_i16 / cols;
            let c = from_i16 % cols;

            for &(dr, dc) in &deltas {
                let nr = r + dr;
                let nc = c + dc;
                if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
                    continue;
                }

                let to_i16 = nr * cols + nc;
                let to = to_i16 as u8;
                let to_bit = 1u64 << (to as u32);
                if (valid_position_mask & to_bit) == 0 {
                    continue;
                }
                edges.push((from, to));
                edges.push((to, from));
                if (seen & to_bit) == 0 {
                    next_jump_from |= to_bit;
                }
            }
        }

        seen |= next_jump_from;
        jump_from = next_jump_from;
    }

    seen
}

pub fn is_win(
    white_mask: u64,
    black_mask: u64,
    white_mask_goal: u64,
    black_mask_goal: u64,
) -> bool {
    white_mask == white_mask_goal && black_mask == black_mask_goal
}

pub fn make_adj_matrix(graph: &crate::graph::Graph) -> Vec<Vec<u8>> {
    let mut adj: Vec<Vec<u8>> = vec![Vec::new(); 64];

    for &(u, v) in &graph.edges {
        let ui = u as usize;
        if ui < 64 {
            adj[ui].push(v);
        }
    }

    for neigh in &mut adj {
        neigh.sort_unstable();
        neigh.dedup();
    }

    adj
}

pub fn search_state_key(white: u64, black: u64) -> u128 {
    ((white as u128) << 64) | (black as u128)
}
