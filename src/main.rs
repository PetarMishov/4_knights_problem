mod graph;
mod utils;

use graph::*;

fn make_mask(
    positions: Vec<(u8, u8)>,
    mut initial_mask: u64,
    bounds: (u8, u8),
    add_1: bool,
) -> u64 {
    let (_, x) = bounds;
    positions.iter().for_each(|&(y0, x0)| {
        let bit_loc = (y0 * x + x0) as u64;
        match add_1 {
            true => {
                let mask = 1 << bit_loc;
                initial_mask = initial_mask | mask
            }
            false => {
                let mask = 1 << bit_loc;
                initial_mask = initial_mask & (!mask)
            }
        }
    });
    initial_mask
}

/*
coord system
first dim = y (top to bottom)
second dim = x (left to right)
*/
fn main() {
    let (y, x) = (4, 4);
    let invalid_squares: Vec<(u8, u8)> = vec![(0, 0),(1, 0), (2, 0), (0, 2), (0, 3), (1, 3)];
    let black_knight_positions: Vec<(u8, u8)> = vec![(3, 0), (3, 2)];
    let white_knight_positions: Vec<(u8, u8)> = vec![(0, 1), (2, 2)];

    let mut valid_position_mask: u64 = u64::MAX >> (64 - (y * x));
    valid_position_mask = make_mask(invalid_squares, valid_position_mask, (y, x), false);

    let mut black_knight_mask: u64 = 0;
    black_knight_mask = make_mask(black_knight_positions, black_knight_mask, (y, x), true);

    let mut white_knight_mask: u64 = 0;
    white_knight_mask = make_mask(white_knight_positions, white_knight_mask, (y, x), true);

    assert!(y <= 8 && x <= 8, "Board dimensions must be <= 8x8");
    assert_eq!(white_knight_mask & black_knight_mask, 0, "Overlapping knights.");
    let all_knights = white_knight_mask | black_knight_mask;
    assert_eq!(all_knights & !valid_position_mask, 0, "Knights in invalid position.");


    let white_goal = black_knight_mask;
    let black_goal = white_knight_mask;

    let initial_graph = make_initial_graph(
        white_knight_mask,
        black_knight_mask,
        valid_position_mask,
        (y, x),
    );

    // DEBUGGING
    // println!("{}", initial_graph);

    let solution_option = try_solve_graph_bfs(initial_graph, white_goal, black_goal);
    if let Some(solution) = solution_option {
        println!("Solution found in {} moves!", solution.len());
        for (i, &(from, to)) in solution.iter().enumerate() {
            println!("{:>3}. {} -> {}", i + 1, from, to);
        }
    } else {
        println!("Solution not found.");
    }
}
