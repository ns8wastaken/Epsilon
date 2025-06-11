pub type Bitboard = u64;

pub trait BitboardExt {
    fn print(&self);
}

impl BitboardExt for Bitboard {
    fn print(&self) {
        println!("  +-----------------+");

        for rank in (0..8).rev() {
            print!("{} |", rank + 1);
            for file in 0..8 {
                let square = rank * 8 + file;
                let occupied = (self >> square) & 1 != 0;
                if file == 0 { print!(" "); }
                print!("{} ", if occupied { 'X' } else { '.' });
            }
            println!("|");
        }

        println!("  +-----------------+");
        println!("    a b c d e f g h");
    }
}
