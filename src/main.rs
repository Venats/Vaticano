use components::board::Board;
use components::board::*;
use components::piece::Piece;

fn main() {}

#[test]
fn test_knight_pseudomoves_starting() {
    let default_board = Board::start();

    let mut attack_board_1 = Board::blank();
    attack_board_1.knight_pseudomoves(&default_board, 0, 1);
    attack_board_1.knight_pseudomoves(&default_board, 0, 6);
    assert_eq!(attack_board_1.sqrs[2], 0x40400404);

    let mut attack_board_2 = Board::blank();
    attack_board_2.knight_pseudomoves(&default_board, 7, 1);
    attack_board_2.knight_pseudomoves(&default_board, 7, 6);
    assert_eq!(attack_board_2.sqrs[5], 0x40400404);
}

#[test]
fn test_knight_pseudomoves_capture() {
    let mut default_board = Board::start();
    default_board.sqrs[2] = col_a(Piece::WhitePawn as u32)
        | col_c(Piece::WhitePawn as u32)
        | col_f(Piece::WhitePawn as u32)
        | col_h(Piece::WhitePawn as u32);

    let mut attack_board_1 = Board::blank();

    attack_board_1.knight_pseudomoves(&default_board, 0, 1);
    assert_eq!(attack_board_1.sqrs[2], 0);
    attack_board_1.knight_pseudomoves(&default_board, 0, 6);
    assert_eq!(attack_board_1.sqrs[2], 0);

    let mut attack_board_2 = Board::blank();

    default_board.sqrs[5] = col_a(Piece::WhitePawn as u32)
        | col_c(Piece::WhitePawn as u32)
        | col_f(Piece::WhitePawn as u32)
        | col_h(Piece::WhitePawn as u32);

    assert_eq!(default_board.sqrs[5], 0x10100101);

    attack_board_2.knight_pseudomoves(&default_board, 7, 1);

    assert_eq!(attack_board_2.sqrs[5], 0x00000404);
    attack_board_2.knight_pseudomoves(&default_board, 7, 6);
    assert_eq!(attack_board_2.sqrs[5], 0x40400404);

    let mut blank_board = Board::blank();
    let mut attack_board_3 = Board::blank();

    blank_board.sqrs[4] = col_f(Piece::BlackKnight as u32);
    blank_board.sqrs[6] = col_e(Piece::WhitePawn as u32) | col_g(Piece::WhitePawn as u32);
    blank_board.sqrs[5] = col_d(Piece::WhitePawn as u32) | col_h(Piece::WhitePawn as u32);
    blank_board.sqrs[3] = col_d(Piece::BlackPawn as u32) | col_h(Piece::BlackPawn as u32);
    blank_board.sqrs[2] = col_e(Piece::WhitePawn as u32) | col_g(Piece::WhitePawn as u32);

    attack_board_3.knight_pseudomoves(&blank_board, 4, 5);

    assert_eq!(attack_board_3.sqrs[6], 0x04040000);
    assert_eq!(attack_board_3.sqrs[5], 0x40004000);
    assert_eq!(attack_board_3.sqrs[3], 0x00000000);
    assert_eq!(attack_board_3.sqrs[2], 0x04040000);
}
