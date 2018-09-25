
/// A vote on a proposed change.
pub struct Vote {
    /// Vote ID.
    pub id: u64,
    /// Number of yes votes.
    pub yes: u64,
    /// Number of no votes.
    pub no: u64,
    /// The time at which voting closes and the results are final.
    pub end_time: u64,
}

impl Vote {
    /// Creates a new vote with the ID `id`, which ends at `end_time`
    fn new(id: u64, end_time: u64) -> Self {
        Vote {
            id,
            yes: 0,
            no: 0,
            end_time
        }
    }
    /// Increments the counter for yes votes.
    fn add_yes(&mut self) {
        self.yes = self.yes + 1;
    }
    /// Decrements the coutner for yes votes.
    fn rm_yes(&mut self) {
        self.yes = self.yes - 1;
    }
    /// Increments the counter for no votes.
    fn add_no(&mut self) {
        self.no = self.no + 1;
    }
    /// Decrements the counter for no votes.
    fn rm_no(&mut self) {
        self.no = self.no - 1;
    }
}
