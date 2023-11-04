#pragma once
class Piece
{
public:
    Piece(int x, int y);
    virtual ~Piece() = 0;

private:
    int m_x;
    int m_y;
};