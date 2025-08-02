#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        if let Some(&latest) = self.scores.last() {
            Some(latest)
        } else {
            None
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut scores= Vec::from(self.scores);
        scores.sort_unstable();
        if let Some(&highest) = scores.last() {
            Some(highest)
        } else {
            None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores= Vec::from(self.scores);
        scores.sort_unstable();
        scores.iter().rev().take(3).copied().collect()

    }
}
