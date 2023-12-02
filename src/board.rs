use crate::piece::Attacks;
use crate::piece::Piece;
use crate::piece::KNIGHT_MOVES;

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub sqrs: [u32; 8],
}

impl Board {
    pub fn start() -> Board {
        let mut b = Board { sqrs: [0; 8] };
        b.sqrs[0] = col_a(Piece::WhiteRook as u32)
            | col_b(Piece::WhiteKnight as u32)
            | col_c(Piece::WhiteBishop as u32)
            | col_d(Piece::WhiteKing as u32)
            | col_e(Piece::WhiteQueen as u32)
            | col_f(Piece::WhiteBishop as u32)
            | col_g(Piece::WhiteKnight as u32)
            | col_h(Piece::WhiteRook as u32);
        b.sqrs[1] = col_a(Piece::WhitePawn as u32)
            | col_b(Piece::WhitePawn as u32)
            | col_c(Piece::WhitePawn as u32)
            | col_d(Piece::WhitePawn as u32)
            | col_e(Piece::WhitePawn as u32)
            | col_f(Piece::WhitePawn as u32)
            | col_g(Piece::WhitePawn as u32)
            | col_h(Piece::WhitePawn as u32);

        b.sqrs[2] = Piece::Empty as u32;
        b.sqrs[3] = Piece::Empty as u32;
        b.sqrs[4] = Piece::Empty as u32;
        b.sqrs[5] = Piece::Empty as u32;
        b.sqrs[6] = col_a(Piece::BlackPawn as u32)
            | col_b(Piece::BlackPawn as u32)
            | col_c(Piece::BlackPawn as u32)
            | col_d(Piece::BlackPawn as u32)
            | col_e(Piece::BlackPawn as u32)
            | col_f(Piece::BlackPawn as u32)
            | col_g(Piece::BlackPawn as u32)
            | col_h(Piece::BlackPawn as u32);

        b.sqrs[7] = col_a(Piece::BlackRook as u32)
            | col_b(Piece::BlackKnight as u32)
            | col_c(Piece::BlackBishop as u32)
            | col_d(Piece::BlackKing as u32)
            | col_e(Piece::BlackQueen as u32)
            | col_f(Piece::BlackBishop as u32)
            | col_g(Piece::BlackKnight as u32)
            | col_h(Piece::BlackRook as u32);
        return b;
    }

    pub fn blank() -> Board {
        let sqrs: [u32; 8] = [0; 8];
        return Board { sqrs };
    }

    pub fn knight_pseudomoves(
        // self is the attack bitboard
        &mut self,
        board: &Board,
        row: usize,
        column_number: usize,
    ) -> Board {
        let column_offset = column_number * 4;
        for knight_attacks in KNIGHT_MOVES {
            let rank_difference = knight_attacks.0;
            let file_offset_difference = knight_attacks.1;
            if let Some(new_diff) = row.checked_add_signed(rank_difference) {
                if Board::row_bounds(new_diff) {
                    if let Some(new_file) = column_offset.checked_add_signed(file_offset_difference)
                    {
                        if Board::col_bounds(new_file / 4) {
                            let shift = board.sqrs[new_diff] >> (new_file as u32);
                            let attacking_piece_square =
                                Board::get_attacking_piece(board, row, column_offset);
                            let attacking_piece = Piece::from_u8(attacking_piece_square).unwrap();
                            let is_attacker_black = Piece::is_black(&attacking_piece);
                            let target_square = Piece::from_u8((shift & 0xF) as u8).unwrap();
                            let is_target_black = Piece::is_black(&target_square);

                            if is_target_black == is_attacker_black && target_square != Piece::Empty
                            {
                                continue;
                            }
                            let attack = Attacks::KnightBishop as u32 & 0xF;
                            if let Some(attack_left) = attack.checked_shl(new_file as u32) {
                                self.sqrs[new_diff] |= attack_left
                            }
                        }
                    }
                }
            }
        }

        return *self;
    }

    const fn row_bounds(row: usize) -> bool {
        return row <= 7;
    }

    const fn col_bounds(column: usize) -> bool {
        let bit_shift = column * 4;
        return bit_shift <= 28;
    }

    const fn column_mask(column: usize) -> u32 {
        return 0xF << (column * 4);
    }

    const fn get_attacking_piece(board: &Board, row: usize, column: usize) -> u8 {
        ((board.sqrs[row] >> (column)) & 0xF) as u8
    }

    pub fn check_square(&self, row: usize, column: usize) -> bool {
        return Board::row_bounds(row)
            && Board::col_bounds(column)
            && self.sqrs[row] & Board::column_mask(column) == 0;
    }
}

pub const fn col_a(x: u32) -> u32 {
    return x << 0;
}
pub const fn col_b(x: u32) -> u32 {
    return x << 4;
}
pub const fn col_c(x: u32) -> u32 {
    return x << 8;
}
pub const fn col_d(x: u32) -> u32 {
    return x << 12;
}
pub const fn col_e(x: u32) -> u32 {
    return x << 16;
}
pub const fn col_f(x: u32) -> u32 {
    return x << 20;
}
pub const fn col_g(x: u32) -> u32 {
    return x << 24;
}
pub const fn col_h(x: u32) -> u32 {
    return x << 28;
}
