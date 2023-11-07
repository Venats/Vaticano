#pragma once

#include <Pieces/Piece.h>
#include <stdint.h>

#define ROW_BOUNDS(bitshift) (bitshift >= 0 && bitshift <= 28)
#define COLUMN_BOUNDS(arr_idx) (arr_idx >= 0 && arr_idx <= 7)
#define BOUNDS(bitshift, arr_idx)                                             \
  (ROW_BOUNDS (bitshift) && (COLUMN_BOUNDS (arr_idx)))

typedef struct
{
  uint32_t sqrs[8];
} board;

void board_init (board *b);
