use super::board::BoardState;
use super::pieces::{Color, GetState, MoveList, Piece};

/// check if the square we are looking at contains an enemy piece
fn check_enemy(color: &Color, piece: Piece) -> bool {
    (piece.get_colour() == Some(Color::Black) && *color == Color::White)
        || (piece.get_colour() == Some(Color::White) && *color == Color::Black)
}

pub fn pawn_move(
    sq: (usize, usize),
    color: &Color,
    first_move: &bool,
    board: &BoardState,
) -> MoveList {
    // fill an array of possible move vectors
    let mut moves: MoveList = Vec::new(); // start with empty movelist
    match color {
        Color::White => {
            // need to make sure that we don't request squares outside of the array bounds
            let (mut row, mut col) = (sq.0, sq.1 + 1);
            //* 1. move forward one if square is empty
            if board[row][col] == Piece::None {
                moves.push(((row, col), false));
                col = sq.1 + 2;
                //* 2. move forward two if hasn't moved and squares are empty
                if board[row][col] == Piece::None && *first_move {
                    moves.push(((row, col), false));
                }
            }
            //* 3. potential attacks if target square contains an enemy piece
            if sq.0 < 7 && sq.1 < 7 {
                (row, col) = (sq.0 + 1, sq.1 + 1);
                if board[row][col].get_colour() == Some(Color::Black) {
                    moves.push(((row, col), true))
                }
            }
            if sq.0 > 0 && sq.1 < 7 {
                (row, col) = (sq.0 - 1, sq.1 + 1);
                if board[row][col].get_colour() == Some(Color::Black) {
                    moves.push(((row, col), true))
                }
            }
        }
        Color::Black => {
            let (mut row, mut col) = (sq.0, sq.1 - 1);
            //* 1. move forward one if square is empty
            if board[row][col] == Piece::None {
                moves.push(((row, col), false));
                col = sq.1 - 2;
                //* 2. move forward two if hasn't moved and squares are empty
                if board[row][col] == Piece::None && *first_move {
                    moves.push(((row, col), false));
                }
            }
            //* 3. potential attacks if target square contains an enemy piece
            if sq.0 < 7 && sq.1 > 0 {
                (row, col) = (sq.0 + 1, sq.1 - 1);
                if board[row][col].get_colour() == Some(Color::White) {
                    moves.push(((row, col), true))
                }
            }
            if sq.0 > 0 && sq.1 > 0 {
                (row, col) = (sq.0 - 1, sq.1 - 1);
                if board[row][col].get_colour() == Some(Color::White) {
                    moves.push(((row, col), true))
                }
            }
        }
    }
    moves
}

pub fn rook_move(sq: (usize, usize), color: &Color, board: &BoardState) -> MoveList {
    let mut moves: MoveList = Vec::new(); // start with empty movelist
    for add in 1..8 {
        //* right
        let (row, col) = (sq.0 + add, sq.1);
        if row > 7 {
            break; // out of bounds, stop
        }
        if board[row][col] != Piece::None {
            if check_enemy(color, board[row][col]) {
                moves.push(((row, col), true));
            }
            break; // stop
        }
        moves.push(((row, col), false));
    }
    for sub in 1..8 {
        //* left
        let (row, col) = (sq.0 as i8 - sub, sq.1);
        if row < 0 {
            break; // out of bounds
        }
        let row = row as usize; // recast back to usize now we know it's >= 0
        if board[row][col] != Piece::None {
            if check_enemy(color, board[row][col]) {
                moves.push(((row, col), true));
            }
            break;
        }
        moves.push(((row, col), false));
    }
    for add in 1..8 {
        //* up
        let (row, col) = (sq.0, sq.1 + add);
        if col > 7 {
            break; // out of bounds, stop
        }
        if board[row][col] != Piece::None {
            if check_enemy(color, board[row][col]) {
                moves.push(((row, col), true));
            }
            break; // stop
        }
        moves.push(((row, col), false));
    }
    for sub in 1..8 {
        //* down
        let (row, col) = (sq.0, sq.1 as i8 - sub);
        if col < 0 {
            break; // out of bounds
        }
        let col = col as usize; // recast back to usize now we know it's >= 0
        if board[row][col] != Piece::None {
            if check_enemy(color, board[row][col]) {
                moves.push(((row, col), true));
            }
            break;
        }
        moves.push(((row, col), false));
    }

    moves
}

pub fn bish_move(sq: (usize, usize), color: &Color, board: &BoardState) -> MoveList {
    let mut moves: MoveList = Vec::new(); // start with empty movelist
    for add in 1..8 {
        //* right up
        let (row, col) = (sq.0 + add, sq.1 + add);
        if row > 7 || col > 7 {
            break; // out of bounds, stop
        }
        if board[row][col] != Piece::None {
            if check_enemy(color, board[row][col]) {
                moves.push(((row, col), true));
            }
            break; // stop
        }
        moves.push(((row, col), false));
    }
    for sub in 1..8 {
        //* left down
        let (row, col) = (sq.0 as i8 - sub, sq.1 as i8 - sub);
        if row < 0 || col < 0 {
            break; // out of bounds
        }
        let (row, col) = (row as usize, col as usize); // recast back to usize
        if board[row][col] != Piece::None {
            if check_enemy(color, board[row][col]) {
                moves.push(((row, col), true));
            }
            break;
        }
        moves.push(((row, col), false));
    }
    for add in 1..8 {
        //* left up
        let (row, col) = (sq.0 as i8 - add, sq.1 as i8 + add);
        if col > 7 || row < 0 {
            break; // out of bounds, stop
        }
        let (row, col) = (row as usize, col as usize); // recast back to usize
        if board[row][col] != Piece::None {
            if check_enemy(color, board[row][col]) {
                moves.push(((row, col), true));
            }
            break; // stop
        }
        moves.push(((row, col), false));
    }
    for sub in 1..8 {
        //* right down
        let (row, col) = (sq.0 as i8 + sub, sq.1 as i8 - sub);
        if col < 0 || row > 7 {
            break; // out of bounds
        }
        let (row, col) = (row as usize, col as usize); // recast back to usize
        if board[row][col] != Piece::None {
            if check_enemy(color, board[row][col]) {
                moves.push(((row, col), true));
            }
            break;
        }
        moves.push(((row, col), false));
    }

    moves
}

pub fn knight_move(sq: (usize, usize), color: &Color, board: &BoardState) -> MoveList {
    let mut moves: MoveList = Vec::new(); // start with empty movelist
                                          //* list all possible vectors for a knight to move
    const VECTORS: [(i8, i8); 8] = [
        (2, 1),
        (2, -1),
        (1, 2),
        (1, -2),
        (-2, 1),
        (-2, -1),
        (-1, 2),
        (-1, -2),
    ];
    for vector in VECTORS {
        let row = sq.0 as i8 + vector.0;
        let col = sq.1 as i8 + vector.1;
        if row >= 0 && row <= 7 && col >= 0 && col <= 7 {
            // valid square
            let (row, col) = (row as usize, col as usize);
            // let target_colour = board[row][col].get_colour();
            if board[row][col] == Piece::None {
                moves.push(((row, col), false));
            } else if check_enemy(color, board[row][col]) {
                moves.push(((row, col), true));
            }
        }
    }

    moves
}
