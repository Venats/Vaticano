#pragma once

#include <stdbool.h>

typedef enum
{
    EMPTY = 0,
    WHITE_PAWN = 0x1,
    WHITE_KNIGHT = 0x2,
    WHITE_BISHOP = 0x3,
    WHITE_ROOK = 0x4,
    WHITE_QUEEN = 0x5,
    WHITE_KING = 0x6,
    BLACK_PAWN = 0x9,
    BLACK_KNIGHT = 0xA,
    BLACK_BISHOP = 0xB,
    BLACK_ROOK = 0xC,
    BLACK_QUEEN = 0xD,
    BLACK_KING = 0xE,
}piece;

typedef struct 
{
    int x;
    int y;
}position;
//TODO:
bool is_white(piece p);
bool is_black(piece p);

// void get_moves(position* moves_buf, piece p, position pos);