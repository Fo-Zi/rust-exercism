#[derive(Debug)]
pub struct HighScores{
    scores_: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores{ scores_: scores.to_vec() }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores_
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores_.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores_.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores_local: Vec<u32> = self.scores().to_vec().clone();
        scores_local.sort_by(|a, b| b.cmp(a));
        match scores_local.len() {
            0..=3 => scores_local,      // 1 to 3 elements, return the whole vector
            _ => scores_local[0..=2].to_vec(), // More than 3 elements, return the first 3
        }
    }
}
