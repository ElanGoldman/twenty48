use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LeaderboardEntry {
    pub score: u32,
    pub board_size: usize,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Leaderboard {
    pub entries: Vec<LeaderboardEntry>,
    #[serde(skip)]
    pub show: bool,
}

impl Leaderboard {
    const STORAGE_KEY: &'static str = "leaderboard_v1";
    const MAX_ENTRIES: usize = 10;

    pub fn load(storage: &dyn eframe::Storage) -> Self {
        storage
            .get_string(Self::STORAGE_KEY)
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, storage: &mut dyn eframe::Storage) {
        if let Ok(json) = serde_json::to_string(self) {
            storage.set_string(Self::STORAGE_KEY, json);
        }
    }

    pub fn add_score(&mut self, score: u32, board_size: usize) {
        if score == 0 {
            return;
        }
        self.entries.push(LeaderboardEntry { score, board_size });
        self.entries.sort_by(|a, b| b.score.cmp(&a.score));
        self.entries.truncate(Self::MAX_ENTRIES);
    }
}
