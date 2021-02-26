use crate::bitboard::Bitboard;
use crate::game::Square;

pub mod bitboard;
pub mod game;

fn main() {
    let mut bitboard = Bitboard::default();
    bitboard.set_bit(Square::E4);
    bitboard.print();
}
