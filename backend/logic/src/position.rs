pub const WIDTH: u32 = 7;
pub const HEIGHT: u32 = 6;

const BOTTOM_MASK: u64 = bottom(WIDTH, HEIGHT);
const BOARD_MASK: u64 = BOTTOM_MASK * ((1 << 6) - 1);


pub trait Position {
    fn can_win_next(&self) -> bool;

    fn get_moves_played(&self) -> u32;

    fn possible_non_losing_move(&self) -> u64;

    fn get_key(&self) -> u64;


    fn play_bit_move(&mut self, new_move: u64);

    fn play(&mut self, seq: &str) -> Result<(), &'static str>;

    fn can_play(&self, col: u32) -> bool;

    fn is_winning_move(&self, col: u32) -> bool;

    fn play_col(&mut self, col: u32);
}

#[derive(Clone, Copy, Debug)]
pub struct BoardPosition {
    current_position: u64,
    mask: u64,
    moves: u32,
}

impl BoardPosition {
    pub fn new() -> BoardPosition {
        BoardPosition {
            current_position: 0,
            mask: 0,
            moves: 0,
        }
    }

    pub fn from_str(seq: &str) -> Result<BoardPosition, &'static str> {
        let mut pos = BoardPosition::new();
        pos.play(seq)?;
        Ok(pos)
    }

    pub fn move_score(&self, move_played: u64) -> u32 {
        pop_count(compute_winning_position(self.current_position | move_played, self.mask))
    }

    fn winning_position(&self) -> u64 {
        compute_winning_position(self.current_position, self.mask)
    }

    fn opponent_winning_position(&self) -> u64 {
        compute_winning_position(self.current_position ^ self.mask, self.mask)
    }

    pub fn mirror(&self) -> u64 {
        let mut mirrored_position = 0;
        let mut mirrored_mask = 0;

        for col in 0..WIDTH as i32 {
            let mirror_col = WIDTH as i32 - col - 1;

            // Create a mask to extract the entire column
            let col_mask = ((1 << HEIGHT) - 1) << (col * (HEIGHT + 1) as i32);

            // Extract the column from current_position and mask
            let position_column = self.current_position & col_mask;
            let mask_column = self.mask & col_mask;

            // Shift the extracted columns to their mirrored positions
            let rotation_amount = ((mirror_col - col) * (HEIGHT + 1) as i32) as u32;
            let mirrored_position_column = position_column.rotate_left(rotation_amount);
            let mirrored_mask_column = mask_column.rotate_left(rotation_amount);

            mirrored_position |= mirrored_position_column;
            mirrored_mask |= mirrored_mask_column;
        }
        get_key(mirrored_position, mirrored_mask)
    }

    pub fn canonical_key(&self) -> u64 {
        std::cmp::min(get_key(self.current_position, self.mask), self.mirror())
    }

    #[inline]
    pub fn possible(&self) -> u64 {
        return (self.mask + BOTTOM_MASK) & BOARD_MASK;
    }
}

impl Position for BoardPosition {
    fn can_win_next(&self) -> bool {
        self.winning_position() & self.possible() != 0
    }

    fn get_moves_played(&self) -> u32 {
        self.moves
    }

    fn possible_non_losing_move(&self) -> u64 {
        let mut possible_mask = self.possible();
        let opponent_win = self.opponent_winning_position();
        let forced_moves = possible_mask & opponent_win;
        if forced_moves != 0 {
            if (forced_moves & (forced_moves - 1)) != 0 {
                // check if there is more than one forced move
                return 0;
            } else {
                possible_mask = forced_moves;
            }
        }
        return possible_mask & !(opponent_win >> 1);
    }

    fn get_key(&self) -> u64 {
        get_key(self.current_position, self.mask)
    }


    fn play_bit_move(&mut self, new_move: u64) {
        self.current_position ^= self.mask;
        self.mask |= new_move;
        self.moves += 1;
    }

    fn play(&mut self, seq: &str) -> Result<(), &'static str> {
        for c in seq.chars() {
            if let Some(digit) = c.to_digit(10) {
                let col = digit - 1;
                if (col >= WIDTH) || !self.can_play(col) || self.is_winning_move(col) {
                    return Err("Invalid move");
                }
                self.play_col(col);
            }
        }
        Ok(())
    }

    fn can_play(&self, col: u32) -> bool {
        return (self.mask & top_mask(col)) == 0;
    }

    fn is_winning_move(&self, col: u32) -> bool {
        self.winning_position() & self.possible() & column_mask(col) != 0
    }

    fn play_col(&mut self, col: u32) {
        self.current_position ^= self.mask;
        self.mask |= self.mask + bottom_mask(col);
        self.moves += 1;
    }
}

#[inline]
const fn compute_winning_position(position: u64, mask: u64) -> u64 {
    let mut r = (position << 1) & (position << 2) & (position << 3);

    let temp1 = position << HEIGHT;
    let temp2 = position << 2 * HEIGHT;

    // horizontal
    let mut p = (position << (HEIGHT + 1)) & (position << 2 * (HEIGHT + 1));
    r |= p & (position << 3 * (HEIGHT + 1));
    r |= p & (position >> (HEIGHT + 1));
    p >>= 3 * (HEIGHT + 1);
    r |= p & (position << (HEIGHT + 1));
    r |= p & (position >> 3 * (HEIGHT + 1));

    // diagonal 1
    p = (temp1) & (temp2);
    r |= p & (position << 3 * HEIGHT);
    r |= p & (position >> HEIGHT);
    p >>= 3 * HEIGHT;
    r |= p & (temp1);
    r |= p & (position >> 3 * HEIGHT);

    // diagonal 2
    let temp3 = position << (HEIGHT + 2);
    let temp4 = position << 2 * (HEIGHT + 2);
    p = temp3 & temp4;
    r |= p & (position << 3 * (HEIGHT + 2));
    r |= p & (position >> (HEIGHT + 2));
    p >>= 3 * (HEIGHT + 2);
    r |= p & (temp3);
    r |= p & (position >> 3 * (HEIGHT + 2));

    return r & (BOARD_MASK ^ mask);
}


#[inline]
const fn bottom(width: u32, height: u32) -> u64 {
    match (width, height) {
        (7, 6) => 4432676798593,
        _ => if width == 0 { 0 } else { bottom(width - 1, height) | 1 << (width - 1) * (height + 1) }
    }
}

#[inline]
const fn top_mask(col: u32) -> u64 {
    return (1 << (HEIGHT - 1)) << col * (HEIGHT + 1);
}

#[inline]
const fn bottom_mask(col: u32) -> u64 {
    return 1 << col * (HEIGHT + 1);
}

pub const fn column_mask(col: u32) -> u64 {
    return ((1 << HEIGHT) - 1) << col * (HEIGHT + 1);
}

#[inline]
const fn pop_count(mut m: u64) -> u32 {
    let mut c = 0;
    while m != 0 {
        c += 1;
        m &= m - 1;
    }
    return c;
}

const fn get_key(current_position: u64, mask: u64) -> u64 {
    current_position + mask
}
