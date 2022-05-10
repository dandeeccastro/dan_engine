// Array of bitboards goes from least to most valuable piece, so 
// it goes Pawn, Knight, Bishop, Rook, Queen and King.
struct ChessBoard {
    white: [u64; 6],
    black: [u64; 6],
    fen: String,
}

fn generate_starter_board() -> ChessBoard {
    let mut board: ChessBoard = ChessBoard { 
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

fn main() {
    let _board: ChessBoard = generate_starter_board(); 
    
    for bitboard in _board.white {
        println!("{:064b}", bitboard);
    }
    println!();

    for bitboard in _board.black {
        println!("{:064b}", bitboard);
    }
}
