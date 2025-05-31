use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LeaderboardEntry {
    pub player_name: String,
    pub score: u32,
    pub proof: String,
}

pub struct Leaderboard {
    entries: Vec<LeaderboardEntry>,
}

impl Leaderboard {
    pub fn new() -> Self {
        Leaderboard {
            entries: Vec::new(),
        }
    }
    
    pub fn add_entry(&mut self, entry: LeaderboardEntry) {
        // In a real implementation, we would verify the proof first
        self.entries.push(entry);
        self.entries.sort_by(|a, b| b.score.cmp(&a.score));
    }
    
    pub fn get_entries(&self) -> Vec<LeaderboardEntry> {
        self.entries.clone()
    }
}
