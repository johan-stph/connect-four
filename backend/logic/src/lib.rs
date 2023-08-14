pub mod solver;
pub mod position;
mod trans_table;
pub mod opening_book;

use crate::position::{BoardPosition};
use crate::solver::Solve;


fn create_solver() -> solver::Solver {
    solver::Solver::new()
}


pub fn get_evaluation(position: &str) -> i32 {
    let mut solver = create_solver();
    let pos = BoardPosition::from_str(&position).unwrap();
    solver.solve(pos)
}

pub fn analyze_position(position: &str) -> Result<Vec<i32>, &'static str> {
    let mut solver = create_solver();
    let pos = BoardPosition::from_str(&position)?;
    Ok(solver.analyze(pos).to_vec())
}

