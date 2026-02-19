use crate::utils;
use std::collections::{HashMap, VecDeque};
use std::fmt;

pub struct Graph {
    pub white_mask: u64,
    pub black_mask: u64,
    pub valid_position_mask: u64,
    pub vertices: Vec<u8>,
    pub edges: Vec<(u8, u8)>,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Graph {{")?;
        writeln!(f, "  white_mask:           0x{:016X}", self.white_mask)?;
        writeln!(f, "  black_mask:           0x{:016X}", self.black_mask)?;
        writeln!(
            f,
            "  valid_position_mask:  0x{:016X}",
            self.valid_position_mask
        )?;
        writeln!(f, "  vertices: {} {:?}", self.vertices.len(), self.vertices)?;
        writeln!(
            f,
            "  edges: {} (directed tuples; undirected stored as both ways)",
            self.edges.len()
        )?;

        let mut adj: [Vec<u8>; 256] = std::array::from_fn(|_| Vec::new());

        for &(u, v) in &self.edges {
            adj[u as usize].push(v);
        }

        writeln!(f, "  adjacency:")?;
        for &u in &self.vertices {
            let neigh = &mut adj[u as usize];
            neigh.sort_unstable();
            neigh.dedup();

            write!(f, "    {:>3} -> [", u)?;
            for (i, v) in neigh.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{v}")?;
            }
            writeln!(f, "]")?;
        }

        writeln!(f, "}}")
    }
}

pub fn make_initial_graph(
    white_mask: u64,
    black_mask: u64,
    valid_position_mask: u64,
    bounds: (u8, u8),
) -> Graph {
    let mut edges = Vec::new();
    let seen_white = utils::add_edges(&mut edges, white_mask, valid_position_mask, bounds);
    let seen_black = utils::add_edges(&mut edges, black_mask, valid_position_mask, bounds);
    let all_seen = seen_white | seen_black;
    let vertices: Vec<u8> = utils::iter_bits(all_seen).collect();

    Graph {
        white_mask,
        black_mask,
        valid_position_mask,
        vertices,
        edges,
    }
}
pub fn try_solve_graph_bfs(
    graph: Graph,
    white_goal: u64,
    black_goal: u64,
) -> Option<Vec<(i8, i8)>> {
    let adj = utils::make_adj_matrix(&graph);

    let start_white = graph.white_mask;
    let start_black = graph.black_mask;

    if utils::is_win(start_white, start_black, white_goal, black_goal) {
        return Some(vec![]);
    }
    let start_k = utils::search_state_key(start_white, start_black);

    // parent[state] = (prev_state, from, to)
    let mut parent: HashMap<u128, (u128, u8, u8)> = HashMap::new();
    let mut q: VecDeque<(u64, u64)> = VecDeque::new();
    parent.insert(start_k, (start_k, 0, 0));
    q.push_back((start_white, start_black));

    while let Some((white, black)) = q.pop_front() {
        let occupied = white | black;

        for from in utils::iter_bits(white) {
            for &to in &adj[from as usize] {
                let to_bit = 1u64 << (to as u32);
                if (graph.valid_position_mask & to_bit) == 0 || occupied & to_bit != 0 {
                    continue;
                }

                let next_white = (white & !(1u64 << (from as u32))) | to_bit;
                let next_black = black;
                let nk = utils::search_state_key(next_white, next_black);

                if !parent.contains_key(&nk) {
                    parent.insert(nk, (utils::search_state_key(white, black), from, to));

                    if utils::is_win(next_white, next_black, white_goal, black_goal) {
                        return Some(reconstruct_moves(parent, start_k, nk));
                    }

                    q.push_back((next_white, next_black));
                }
            }
        }

        for from in utils::iter_bits(black) {
            for &to in &adj[from as usize] {
                let to_bit = 1u64 << (to as u32);
                if (graph.valid_position_mask & to_bit) == 0 || occupied & to_bit != 0 {
                    continue;
                }

                let next_white = white;
                let next_black = (black & !(1u64 << (from as u32))) | to_bit;
                let nk = utils::search_state_key(next_white, next_black);

                if !parent.contains_key(&nk) {
                    parent.insert(nk, (utils::search_state_key(white, black), from, to));

                    if utils::is_win(next_white, next_black, white_goal, black_goal) {
                        return Some(reconstruct_moves(parent, start_k, nk));
                    }

                    q.push_back((next_white, next_black));
                }
            }
        }
    }

    None
}

fn reconstruct_moves(
    parent: HashMap<u128, (u128, u8, u8)>,
    start_k: u128,
    mut goal_k: u128,
) -> Vec<(i8, i8)> {
    let mut moves: Vec<(i8, i8)> = Vec::new();

    while goal_k != start_k {
        let (pk, from, to) = parent[&goal_k];
        moves.push((from as i8, to as i8));
        goal_k = pk;
    }

    moves.reverse();
    moves
}
