use crate::types::Move;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TTFlag {
    Exact,
    LowerBound, // Alpha (failed low)
    UpperBound, // Beta (failed high)
}

#[derive(Clone, Copy)]
pub struct TTEntry {
    pub key: u64,
    pub score: i32,
    pub depth: u8,
    pub flag: TTFlag,
    pub best_move: Option<Move>,
}

pub struct TranspositionTable {
    entries: Vec<Option<TTEntry>>,
    size: usize,
}

impl TranspositionTable {
    pub fn new(size: usize) -> Self { // size in megabytes
        let entry_size = size_of::<Option<TTEntry>>();
        let num_entries = (size * 1024 * 1024) / entry_size;

        Self {
            entries: vec![None; num_entries],
            size: num_entries,
        }
    }

    pub fn clear(&mut self) {
        self.entries.fill(None);
    }

    /// Check for existing evaluation
    pub fn read(&self, key: u64, depth: u8, alpha: i32, beta: i32) -> Option<i32> {
        let index = (key as usize) % self.size;

        self.entries[index]
            .filter(|e| e.key == key && e.depth >= depth)
            .filter(|e| match e.flag {
                TTFlag::Exact => true,
                TTFlag::LowerBound => e.score >= beta,
                TTFlag::UpperBound => e.score <= alpha,
            })
            .map(|e| e.score)
    }

    pub fn write(&mut self, key: u64, depth: u8, score: i32, flag: TTFlag, best_move: Option<Move>) {
        if !(-48_000..=48_000).contains(&score) {
            return;
        }

        let index = (key as usize) % self.size;

        self.entries[index] = Some(TTEntry {
            key,
            depth,
            score,
            flag,
            best_move,
        });
    }
}
