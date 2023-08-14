use crate::opening_book::OpeningBook;
use crate::position::{BoardPosition, column_mask, HEIGHT, Position, WIDTH};
use crate::trans_table::TranspositionTable;

const MIN_SCORE: i32 = -((WIDTH * HEIGHT) as i32) / 2 + 3;
const MAX_SCORE: i32 = ((WIDTH * HEIGHT) / 2 - 3) as i32;

const INVALID_MOVE: i32 = -1000000;

const MOVE_ORDER: [u32; 7] = [3, 4, 2, 5, 1, 6, 0];

pub trait Solve {
    fn solve(&mut self, pos: BoardPosition) -> i32;

    fn analyze(&mut self, pos: BoardPosition) -> [i32; 7];
}

pub struct Solver {
    transposition_table: TranspositionTable,
    opening_book: OpeningBook,
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            transposition_table: TranspositionTable::new(),
            opening_book: OpeningBook::new(),
        }
    }
}

impl Solve for Solver {
    fn solve(&mut self, position: BoardPosition) -> i32 {
        if let Some(score) = self.opening_book.get(&position) {
            return score as i32;
        }
        if position.can_win_next() {
            return ((WIDTH * HEIGHT + 1 - position.get_moves_played()) / 2) as i32;
        }
        self.transposition_table = TranspositionTable::new();
        let mut min = -((WIDTH * HEIGHT - position.get_moves_played()) as i32) / 2;
        let mut max = ((WIDTH * HEIGHT + 1 - position.get_moves_played()) / 2) as i32;

        while min < max {
            let mut med = min + (max - min) / 2;
            if med <= 0 && min / 2 < med {
                med = min / 2;
            } else if med >= 0 && max / 2 > med {
                med = max / 2;
            }

            let r = self.negamax(position, med, med + 1);   // use a null depth window to know if the actual score is greater or smaller than med
            if r <= med {
                max = r;
            } else {
                min = r;
            }
        }
        min
    }

    fn analyze(&mut self, pos: BoardPosition) -> [i32; 7] {
        let mut scores = [INVALID_MOVE; 7];
        for col in 0..WIDTH {
            if pos.can_play(col) {
                if pos.is_winning_move(col) {
                    scores[col as usize] = ((WIDTH * HEIGHT + 1 - pos.get_moves_played()) / 2) as i32;
                } else {
                    let mut pos2 = pos.clone();
                    pos2.play_col(col);
                    scores[col as usize] = -1 * self.solve(pos2);
                }
            }
        }
        scores
    }
}

impl Solver {
    fn negamax(&mut self, position: BoardPosition, mut alpha: i32, mut beta: i32) -> i32 {
        let next_possible = position.possible_non_losing_move();
        let temp = (HEIGHT * WIDTH - position.get_moves_played()) as i32 / 2;
        if next_possible == 0 {
            return -temp;
        }
        if position.get_moves_played() >= 40 { // 42 - 2
            return 0;
        }

        let mut min = -temp;
        if alpha < min {
            alpha = min;
            if alpha >= beta {
                return alpha;
            }
        }
        let mut max = (41 - position.get_moves_played()) as i32 / 2;


        if beta > max {
            beta = max;
            if alpha >= beta {
                return beta;
            }
        }
        let key = position.canonical_key();
        let val = self.transposition_table.get(key);
        match val {
            Some(val) => {
                if val as i32 > MAX_SCORE - MIN_SCORE + 1 {
                    min = val as i32 + 2 * MIN_SCORE - MAX_SCORE - 2;
                    if alpha < min {
                        alpha = min;
                        if alpha >= beta {
                            return alpha;
                        }
                    }
                } else {
                    max = val as i32 + MIN_SCORE - 1;
                    if beta > max {
                        beta = max;
                        if alpha >= beta {
                            return beta;
                        }
                    }
                }
            }
            None => (),
        }


        let mut move_sorter = MoveSorter::new();
        for i in (0..WIDTH).rev() {
            let move_mask = next_possible & column_mask(MOVE_ORDER[i as usize]);
            if move_mask != 0 {
                move_sorter.add(move_mask, position.move_score(move_mask));
            }
        }

        while let Some(next) = move_sorter.get_next() {
            let mut new_pos = position;
            new_pos.play_bit_move(next);
            let score = -self.negamax(new_pos, -beta, -alpha);
            if score >= beta {
                self.transposition_table.put(key, (score + MAX_SCORE - 2 * MIN_SCORE + 2) as u8);
                return score;
            }
            if score > alpha {
                alpha = score;
            }
        }
        self.transposition_table.put(position.canonical_key(), (alpha - MIN_SCORE + 1) as u8);
        return alpha;
    }
}

pub struct MoveSorter {
    entries: [(u64, u32); WIDTH as usize],
    size: usize,
}

impl MoveSorter {
    pub fn new() -> MoveSorter {
        MoveSorter {
            entries: [(0, u32::MIN); WIDTH as usize],
            size: 0,
        }
    }

    pub fn add(&mut self, move_: u64, score: u32) {
        let mut pos = self.size;
        self.size += 1;

        while pos > 0 && self.entries[pos - 1].1 > score {
            self.entries[pos] = self.entries[pos - 1];
            pos -= 1;
        }

        self.entries[pos] = (move_, score);
    }

    pub fn get_next(&mut self) -> Option<u64> {
        if self.size > 0 {
            self.size -= 1;
            Some(self.entries[self.size].0)
        } else {
            None
        }
    }
}
