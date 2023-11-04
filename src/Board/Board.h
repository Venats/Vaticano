#pragma once

#include "Pieces/Piece.h"
#include <vector>

class Board
{
public:
    Board();

private:
    std::vector<Piece> black_pieces;
    std::vector<Piece> white_pieces;
};
