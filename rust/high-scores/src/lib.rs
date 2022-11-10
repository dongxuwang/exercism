#[derive(Debug)]
pub struct HighScores<'a>{
    scores: &'a [u32]
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a[u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut scores = Vec::from(self.scores);
        scores.sort_by(|a, b| b.cmp(a));
        scores.first().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = Vec::from(self.scores);
        scores.sort_by(|a, b| b.cmp(a));
        scores.truncate(3);
        scores
    }
}
