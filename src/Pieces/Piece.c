#include "Piece.h"
#include "Board/Board.h"
#include <stdbool.h>

bool
check_square (uint32_t *board_columns, uint32_t column_idx,
              uint32_t bitshift_to_check)
{
  if (!BOUNDS (new_abs_bitshift, board_idx))
    {
      return false;
    }
  uint32_t masked_shift = MASK_PIECE (bitshift_to_check);
  if ((board_columns[column_idx] & masked_shift) == 0)
    {
      return true;
    }
  return false;
}