mod chess {
    // Array of bitboards goes from least to most valuable piece, so 
    // it goes Pawn, Knight, Bishop, Rook, Queen and King.
    pub struct Board {
        white: [u64; 6],
        black: [u64; 6],
        fen: String,
    }

    pub fn generate_starter_board() -> Board {
        let mut board: Board = Board { 
            white: [0x0;6], 
            black: [0x0;6],
            fen: String::
                from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
        };

        // Pawns
        board.white[0] = 0x00ff000000000000;
        board.black[0] = 0x000000000000ff00;

        // Knights 
        board.white[1] = 0x4200000000000000; 
        board.black[1] = 0x0000000000000042; 

        // Bishops
        board.white[2] = 0x2400000000000000;
        board.black[2] = 0x0000000000000024;

        // Rooks 
        board.white[3] = 0x8100000000000000;
        board.black[3] = 0x0000000000000081;

        // Queen
        board.white[4] = 0x1000000000000000;
        board.black[4] = 0x0000000000000010; 

        // King
        board.white[5] = 0x0800000000000000;
        board.black[5] = 0x0000000000000008;

        return board;
    }

    mod movement {
        priv let not_a_file: u64 = 0xfefefefefefefefe;
        priv let not_h_file: u64 = 0x7f7f7f7f7f7f7f7f;

        pub fn south(bitboard:u64)     { return bitboard >> 8; } 
        pub fn north(bitboard:u64)     { return bitboard << 8; } 

        pub fn east(bitboard:u64)      { return (bitboard >> 1) & not_h_file; } 
        pub fn west(bitboard:u64)      { return (bitboard << 1) & not_a_file; } 
        pub fn southeast(bitboard:u64) { return (bitboard >> 7) & not_h_file; } 
        pub fn southwest(bitboard:u64) { return (bitboard >> 9) & not_h_file; } 
        pub fn northeast(bitboard:u64) { return (bitboard << 9) & not_a_file; }
        pub fn northwest(bitboard:u64) { return (bitboard << 7) & not_a_file; }
    }
}

use chess::*; 
fn main() {
    let _board: Board = generate_starter_board(); 
}
