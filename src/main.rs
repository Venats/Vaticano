const fn col_a(x: u32) -> u32 { return x << 0; }
const fn col_b(x: u32) -> u32 { return x << 4; }
const fn col_c(x: u32) -> u32 { return x << 8; }
const fn col_d(x: u32) -> u32 { return x << 12; }
const fn col_e(x: u32) -> u32 { return x << 16; }
const fn col_f(x: u32) -> u32 { return x << 20; }
const fn col_g(x: u32) -> u32 { return x << 24; }
const fn col_h(x: u32) -> u32 { return x << 28; }

#[derive(Debug)]
struct Board
{
    sqrs: [u32; 8],
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Piece
{
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

impl Piece
{
    pub fn from_u8(value: u8) -> Option<Piece>
    {
        match value
        {
            0 =>      Some(Piece::Empty),
            0x1 =>    Some(Piece::WhitePawn),
            0x2 =>    Some(Piece::WhiteKnight),
            0x3 =>    Some(Piece::WhiteBishop),
            0x4 =>    Some(Piece::WhiteRook),
            0x5 =>    Some(Piece::WhiteQueen),
            0x6 =>    Some(Piece::WhiteKing),
            0x9 =>    Some(Piece::BlackPawn),
            0xA =>    Some(Piece::BlackKnight),
            0xB =>    Some(Piece::BlackBishop),
            0xC =>    Some(Piece::BlackRook),
            0xD =>    Some(Piece::BlackQueen),
            0xE =>    Some(Piece::BlackKing),
            _ => None
        }
    }

    pub fn is_black(&self) -> bool
    {
        return (*self as u8) & 0x8 != 0
    }
    pub fn is_white(&self) -> bool
    {
        return !self.is_black();
    }
}

impl Board
{
    pub fn start() -> Board
    {
        let mut b = Board{sqrs:[0;8]};
        b.sqrs[0] = col_a(Piece::WhiteRook as u32) | col_b(Piece::WhiteKnight as u32) | col_c(Piece::WhiteBishop as u32)
            | col_d(Piece::WhiteKing as u32) | col_e(Piece::WhiteQueen as u32) | col_f(Piece::WhiteBishop as u32)
            | col_g(Piece::WhiteKnight as u32) | col_h(Piece::WhiteRook as u32);
        b.sqrs[1] = col_a(Piece::WhitePawn as u32) | col_b(Piece::WhitePawn as u32) | col_c(Piece::WhitePawn as u32)
            | col_d(Piece::WhitePawn as u32) | col_e(Piece::WhitePawn as u32) | col_f(Piece::WhitePawn as u32)
            | col_g(Piece::WhitePawn as u32) | col_h(Piece::WhitePawn as u32);
        b.sqrs[2] = Piece::Empty as u32;
        b.sqrs[3] = Piece::Empty as u32;
        b.sqrs[4] = Piece::Empty as u32;
        b.sqrs[5] = Piece::Empty as u32;
        
        b.sqrs[6] = col_a(Piece::BlackPawn as u32) | col_b(Piece::BlackPawn as u32) | col_c(Piece::BlackPawn as u32)
            | col_d(Piece::BlackPawn as u32) | col_e(Piece::BlackPawn as u32) | col_f(Piece::BlackPawn as u32)
            | col_g(Piece::BlackPawn as u32) | col_h(Piece::BlackPawn as u32);
        b.sqrs[7] = col_a(Piece::BlackRook as u32) | col_b(Piece::BlackKnight as u32) | col_c(Piece::BlackBishop as u32)
            | col_d(Piece::BlackKing as u32) | col_e(Piece::BlackQueen as u32) | col_f(Piece::BlackBishop as u32)
            | col_g(Piece::BlackKnight as u32) | col_h(Piece::BlackRook as u32);
        return b;
    }

    const fn row_bounds(row: usize) -> bool
    {
        return row >= 0 && row <= 7;
    }

    const fn col_bounds(column: usize) -> bool
    {
        let bit_shift = column * 4;
        return bit_shift >= 0 && bit_shift <= 28;
    }

    const fn column_mask(column: usize) -> u32 
    {
        return 0xF << column * 4;
    }

    pub fn check_square(&self, row: usize, column: usize) -> bool
    {
        return Board::row_bounds(row) &&
               Board::col_bounds(column) &&
               self.sqrs[row] & Board::column_mask(column) == 0;
    }
}

fn main()
{
    let b = Board::start();
    println!("{:?}",b);
}
