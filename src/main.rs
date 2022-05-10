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
        const NOT_A_FILE: u64 = 0xfefefefefefefefe;
        const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

        fn south(bitboard:u64) -> u64 { return bitboard >> 8; } 
        fn north(bitboard:u64) -> u64 { return bitboard << 8; } 
        fn east(bitboard:u64)  -> u64 { 
            return (bitboard >> 1) & NOT_H_FILE; 
        } 
        fn west(bitboard:u64)  -> u64 { 
            return (bitboard << 1) & NOT_A_FILE; 
        } 
        fn southeast(bitboard:u64) -> u64 { 
            return (bitboard >> 7) & NOT_H_FILE; 
        } 
        fn southwest(bitboard:u64) -> u64 { 
            return (bitboard >> 9) & NOT_H_FILE; 
        } 
        fn northeast(bitboard:u64) -> u64 { 
            return (bitboard << 9) & NOT_A_FILE; 
        }
        fn northwest(bitboard:u64) -> u64 { 
            return (bitboard << 7) & NOT_A_FILE; 
        }

        pub fn knights(bitboard:u64) -> u64 {
            let res:u64 = 
                south(southwest(bitboard)) | 
                west(southwest(bitboard)) | 
                south(southeast(bitboard)) | 
                east(southeast(bitboard)) | 
                north(northwest(bitboard)) | 
                west(northwest(bitboard)) | 
                north(northeast(bitboard)) | 
                east(northeast(bitboard));
            return res;
        }

        pub fn w_single_pawn_push(bitboard:u64, empty:u64) -> u64 {
            return north(bitboard) & empty;
        }

        pub fn w_double_pawn_push(bitboard:u64, empty:u64) -> u64 {
            return north(north(bitboard) & empty) & empty;
        }

        pub fn b_single_pawn_push(bitboard:u64, empty:u64) -> u64 { 
            return south(bitboard) & empty;
        }

        pub fn b_double_pawn_push(bitboard:u64, empty:u64) -> u64 {
            return south(south(bitboard) & empty) & empty;
        }

        pub fn kings(bitboard:u64, empty:u64) -> u64 {
            let res: u64 = 
                (north(bitboard) & empty) | 
                (south(bitboard) & empty) | 
                (east(bitboard) & empty) | 
                (west(bitboard) & empty) | 
                (southeast(bitboard) & empty) | 
                (southwest(bitboard) & empty) | 
                (northeast(bitboard) & empty) | 
                (northwest(bitboard) & empty);
            return res;
        }

    }
}

use chess::*; 
fn main() {
    let _board: Board = generate_starter_board(); 
}
