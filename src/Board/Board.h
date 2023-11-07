#pragma once

#include <Pieces/Piece.h>
#include <stdint.h>

typedef struct
{
  uint32_t sqrs[8];
} board;

void board_init (board *b);
