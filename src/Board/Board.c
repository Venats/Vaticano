#include "Board.h"

#define A(x) x << 0
#define B(x) x << 4
#define C(x) x << 8
#define D(x) x << 12
#define E(x) x << 16
#define F(x) x << 20
#define G(x) x << 24
#define H(x) x << 28

void board_init(board* b)
{
    b->sqrs[0] = A(WHITE_ROOK) | B(WHITE_KNIGHT) | C(WHITE_BISHOP) | D(WHITE_KING) | E(WHITE_QUEEN) | F(WHITE_BISHOP) | G(WHITE_KNIGHT) | H(WHITE_ROOK);
    b->sqrs[1] = A(WHITE_PAWN) | B(WHITE_PAWN) | C(WHITE_PAWN) | D(WHITE_PAWN) | E(WHITE_PAWN) | F(WHITE_PAWN) | G(WHITE_PAWN) | H(WHITE_PAWN);
    b->sqrs[2] = 0;
    b->sqrs[3] = 0;
    b->sqrs[4] = 0;
    b->sqrs[5] = 0;
    b->sqrs[6] = A(BLACK_PAWN) | B(BLACK_PAWN) | C(BLACK_PAWN) | D(BLACK_PAWN) | E(BLACK_PAWN) | F(BLACK_PAWN) | G(BLACK_PAWN) | H(BLACK_PAWN);
    b->sqrs[7]=  A(BLACK_ROOK) | B(BLACK_KNIGHT) | C(BLACK_BISHOP) | D(BLACK_KING) | E(BLACK_QUEEN) | F(BLACK_BISHOP) | G(BLACK_KNIGHT) | H(BLACK_ROOK);
}
