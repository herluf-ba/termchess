use std::fmt::Display;

use crate::{board::Board, Move, Piece, Square};

/// A bit board representation of a game of chess.
/// A chess board has 64 squares, so it can be represented as a 64-bit unsigned integer.
/// a1 is the 1st bit a2 the 2nd bit. The a2 is the 9th bit. h8 is the very last 64th bit.
/// The position of each piece type is saved as 1-bits within these u64's.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct BitBoard {
    pawns_white: u64,
    bishops_white: u64,
    knights_white: u64,
    rooks_white: u64,
    king_white: u64,
    queen_white: u64,

    pawns_black: u64,
    bishops_black: u64,
    knights_black: u64,
    rooks_black: u64,
    king_black: u64,
    queen_black: u64,

    // TODO: Add castling rights and en pessant representations
    white_to_move: bool,
}

impl Board for BitBoard {
    /// Create a new chess board with no pieces placed.
    fn blank() -> Self {
        Self {
            pawns_white: 0,
            bishops_white: 0,
            knights_white: 0,
            rooks_white: 0,
            king_white: 0,
            queen_white: 0,
            pawns_black: 0,
            bishops_black: 0,
            knights_black: 0,
            rooks_black: 0,
            king_black: 0,
            queen_black: 0,
            white_to_move: true,
        }
    }

    /// Create a bit-board from a Forsyth-Edwards-Notation (FEN) string.
    fn from_fen_string(fen: &str) -> Self {
        // TODO: Make this return Result and don't panic

        // FEN contains 6 fields separated by space.
        // They are:
        // 1. Piece placement.
        // 2. Side to move (w/b)
        // 3. Castling ability
        // 4. En pessant target square
        // 5. Halfmove clock
        // 6. Fullmove counter
        // Fields 5. and 6. may be left out.
        let fields: Vec<_> = fen.split_whitespace().collect();
        if fields.len() > 6 || fields.len() < 4 {
            panic!("Not a valid FEN string: '{fen}'");
        }

        // Read piece placement and place onto blank board.
        // Placement is presented from rank 8 to 1, each rank separated by '/'.
        // Each rank lists the pieces (pnbrqk) going from file 1 to 8. White is uppercase.
        // N consequtive blank squares are listed as the number N.
        // For example here is the standard setup:
        // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR
        let mut board = Self::blank();
        let placement = fields[0];
        for (rank_idx, rank_str) in placement.split('/').enumerate() {
            let rank: u8 = 7 - (rank_idx as u8);
            let mut file: u8 = 0;
            for piece in rank_str.chars() {
                match piece {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                        // Skip this amount of squares
                        file += piece.to_string().parse::<u8>().unwrap();
                    }
                    p => {
                        if let Some(valid) = Piece::from_char(&p) {
                            board.place(valid, rank * 8 + file);
                            file += 1
                        } else {
                            panic!("Not a valid FEN string: '{fen}'")
                        }
                    }
                }
            }
        }

        // Read whose turn it is.
        match fields[1] {
            "w" => board.white_to_move = true,
            "b" => board.white_to_move = false,
            _ => panic!("Not a valid FEN string: '{fen}'"),
        }

        // TODO: Read the rest of the FEN string.

        board
    }

    fn clear(&mut self, square: Square) {
        let mask = !(1u64 << square);
        self.pawns_black &= mask;
        self.bishops_black &= mask;
        self.knights_black &= mask;
        self.rooks_black &= mask;
        self.king_black &= mask;
        self.queen_black &= mask;
        self.pawns_white &= mask;
        self.bishops_white &= mask;
        self.knights_white &= mask;
        self.rooks_white &= mask;
        self.king_white &= mask;
        self.queen_white &= mask;
    }

    fn place(&mut self, piece: Piece, at: Square) {
        self.clear(at); // Clear any pieces off the target square.

        let mask = 1u64 << at;
        match piece {
            Piece::PawnWhite => self.pawns_white |= mask,
            Piece::KnightWhite => self.knights_white |= mask,
            Piece::BishopWhite => self.bishops_white |= mask,
            Piece::RookWhite => self.rooks_white |= mask,
            Piece::QueenWhite => self.queen_white |= mask,
            Piece::KingWhite => self.king_white |= mask,
            Piece::PawnBlack => self.pawns_black |= mask,
            Piece::KnightBlack => self.knights_black |= mask,
            Piece::BishopBlack => self.bishops_black |= mask,
            Piece::RookBlack => self.rooks_black |= mask,
            Piece::QueenBlack => self.queen_black |= mask,
            Piece::KingBlack => self.king_black |= mask,
        }
    }

    /// Applies a move to the board with _no_ validation.
    fn apply(&mut self, r#move: Move) {
        // Look up what piece is moving.
        if let Some(from_piece) = self.at(r#move.from) {
            // Clear square that piece is moving from and to.
            self.clear(r#move.from);
            self.place(from_piece, r#move.to);
        }
    }

    /// Lookup what piece is at a particular square in the board.
    fn at(&self, square: Square) -> Option<Piece> {
        let mask = 1u64 << square;
        if self.pawns_white & mask >= 1 {
            Some(Piece::PawnWhite)
        } else if self.pawns_black & mask >= 1 {
            Some(Piece::PawnBlack)
        } else if self.knights_white & mask >= 1 {
            Some(Piece::KnightWhite)
        } else if self.knights_black & mask >= 1 {
            Some(Piece::KnightBlack)
        } else if self.bishops_white & mask >= 1 {
            Some(Piece::BishopWhite)
        } else if self.bishops_black & mask >= 1 {
            Some(Piece::BishopBlack)
        } else if self.rooks_white & mask >= 1 {
            Some(Piece::RookWhite)
        } else if self.rooks_black & mask >= 1 {
            Some(Piece::RookBlack)
        } else if self.queen_white & mask >= 1 {
            Some(Piece::QueenWhite)
        } else if self.queen_black & mask >= 1 {
            Some(Piece::QueenBlack)
        } else if self.king_white & mask >= 1 {
            Some(Piece::KingWhite)
        } else if self.king_black & mask >= 1 {
            Some(Piece::KingBlack)
        } else {
            None
        }
    }

    fn white_to_move(&self) -> bool {
        self.white_to_move
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::with_capacity(1028); // make sure the string has capacity for the board string.
        out.push_str("   -----------------\n");

        for r in (0..8).rev() {
            out.push_str(format!("{} | ", r + 1).as_str());
            for f in 0..8 {
                let square = r * 8 + f;
                match self.at(square) {
                    None => out.push_str("  "),
                    Some(piece) => out.push_str(format!("{} ", piece).as_str()),
                }
            }
            out.push_str("|\n");
        }
        out.push_str("   -----------------\n    a b c d e f g h");
        write!(f, "{}", out)
    }
}
