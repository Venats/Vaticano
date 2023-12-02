pub const KNIGHT_MOVES: [(isize, isize); 8] = [
    (1, 8),
    (1, -8),
    (-1, 8),
    (-1, -8),
    (2, 4),
    (2, -4),
    (-2, 4),
    (-2, -4),
];

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Attacks {
    Pawn = 0x8,
    KnightBishop = 0x4,
    Rook = 0x2,
    KingQueen = 0x1,
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Piece {
    Empty = 0,
    WhitePawn = 0x1,
    WhiteKnight = 0x2,
    WhiteBishop = 0x3,
    WhiteRook = 0x4,
    WhiteQueen = 0x5,
    WhiteKing = 0x6,
    BlackPawn = 0x9,
    BlackKnight = 0xA,
    BlackBishop = 0xB,
    BlackRook = 0xC,
    BlackQueen = 0xD,
    BlackKing = 0xE,
}

impl Piece {
    pub fn from_u8(value: u8) -> Option<Piece> {
        match value {
            0 => Some(Piece::Empty),
            0x1 => Some(Piece::WhitePawn),
            0x2 => Some(Piece::WhiteKnight),
            0x3 => Some(Piece::WhiteBishop),
            0x4 => Some(Piece::WhiteRook),
            0x5 => Some(Piece::WhiteQueen),
            0x6 => Some(Piece::WhiteKing),
            0x9 => Some(Piece::BlackPawn),
            0xA => Some(Piece::BlackKnight),
            0xB => Some(Piece::BlackBishop),
            0xC => Some(Piece::BlackRook),
            0xD => Some(Piece::BlackQueen),
            0xE => Some(Piece::BlackKing),
            _ => None,
        }
    }

    pub fn is_black(&self) -> bool {
        return (*self as u8) & 0x8 != 0;
    }
    pub fn is_white(&self) -> bool {
        return !self.is_black();
    }
}
