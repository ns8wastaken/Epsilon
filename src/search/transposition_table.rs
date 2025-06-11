use crate::board::Move;

#[derive(Clone, Copy)]
pub enum TTBound {
    Exact, // All positions searched, this is the score
    Upper, // The move never improved alpha
    Lower, // The move good enough to cause a beta cutoff
}

#[derive(Clone, Copy)]
pub struct TTEntry {
    pub key: u64,   // Zobrist key
    pub depth: u8,  // Search depth
    pub bound: TTBound,
    pub score: i16,
    // pub best_move: Option<Move>,
}

pub struct TranspositionTable {
    table: Vec<Option<TTEntry>>,
    size: usize,
}

impl TranspositionTable {
    pub fn new(size: usize) -> Self {
        Self {
            table: vec![None; size],
            size,
        }
    }

    pub fn store(&mut self, key: u64, depth: u8, bound: TTBound, score: i16) {
        let index = (key as usize) % self.size;

        let new_entry = Some(TTEntry {
            key,
            depth,
            bound,
            score,
            // best_move
        });

        match &self.table[index] {
            Some(entry) => {
                if depth > entry.depth {
                    self.table[index] = new_entry;
                }
            }

            None => self.table[index] = new_entry,
        }
    }

    #[inline(always)]
    pub fn retrieve(&self, key: u64) -> Option<TTEntry> {
        match self.table[(key as usize) % self.size] {
            Some(e) if e.key == key => Some(e),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        for i in 0..self.table.len() {
            self.table[i] = None;
        }
    }
}
