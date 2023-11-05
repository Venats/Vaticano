#pragma once

#include <stdint.h>
#include <Pieces/Piece.h>

typedef struct
{
    uint32_t sqrs[8];
}board;

void board_init(board* b);
piece get_piece(position pos);