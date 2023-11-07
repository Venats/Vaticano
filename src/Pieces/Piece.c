#include "Piece.h"
#include "Board/Board.h"
#include <stdbool.h>

#define COLUMN_BOUNDS(bitshift) (bitshift >= 0 && bitshift <= 28)
#define ROW_BOUNDS(arr_idx) (arr_idx >= 0 && arr_idx <= 7)
#define BOUNDS(bitshift, arr_idx)                                             \
  (ROW_BOUNDS (arr_idx) && (COLUMN_BOUNDS (bitshift)))

bool
check_square (const uint32_t *board_rows, uint32_t row_idx, uint32_t bitshift)
{
  if (!BOUNDS (bitshift, row_idx))
    {
      return false;
    }
  uint32_t masked_shift = MASK_PIECE (bitshift);
  if ((board_rows[row_idx] & masked_shift) == 0)
    {
      return true;
    }
  return false;
}