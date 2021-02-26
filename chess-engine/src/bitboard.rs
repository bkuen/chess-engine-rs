//! Types to represent a bitboard for the engine

/// A `BitBoard` represents the position of one piece type
#[derive(Copy, Clone, Default)]
pub struct Bitboard(u64);

impl Into<Bitboard> for u64 {
    fn into(self) -> Bitboard {
        Bitboard(self)
    }
}

impl Bitboard {
    /// Prints a bitboard to the console by iterating
    /// over all ranks and files
    pub fn print(&self) {
        // Print empty line at the beginning
        println!();

        // Loop over board ranks
        for rank in 0..8 {
            // Loop over board files
            for file in 0..8 {
                // Calculate the particular square
                let square = rank * 8 + file;

                // Print rank numbers
                if file == 0 {
                    print!(" {}  ", 8 - rank);
                }

                // Print bit state of the square to the console
                print!(" {} ", if self.bit(square) == 0 { 0 } else { 1 });
            }

            // Print a line break for every new rank
            println!();
        }

        // Print the files characters
        print!("\n    ");
        for c in "abcdefgh".chars() {
            print!(" {} ", c);
        }

        // Print the bitboard representation
        println!("\n\n===========================");
        println!("Bitboard: {}", self.0);
    }

    /// Sets a bit of a square to `1`
    ///
    /// # Arguments
    ///
    /// * `square` - The given square
    pub fn set_bit<T: Into<i32>>(&mut self, square: T) {
        self.0 |= 1 << square.into();
    }

    /// Pops a bit of a square
    ///
    /// # Arguments
    ///
    /// * `square` - The given square
    pub fn pop_bit<T: Into<i32>>(&mut self, square: T) {
        self.0 |= 0 << square.into();
    }

    /// Returns a bit at a given square
    ///
    /// # Arguments
    ///
    /// * `square` - The given square
    pub fn bit<T: Into<i32>>(&self, square: T) -> u64 {
        self.0 & 1 << square.into()
    }
}

