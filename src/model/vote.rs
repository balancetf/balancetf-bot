
/// A vote on a proposed change.
pub struct Vote {
    /// Vote ID.
    pub id: u64,
    /// Number of yes votes from users with the Casual role.
    pub casual_yes: u64,
    /// Number of no votes from users with the Casual role.
    pub casual_no: u64,
    /// Number of yes votes from users with the Competitive role.
    pub comp_yes: u64,
    /// Number of no votes from users with the Competitive role.
    pub comp_no: u64,
    /// The time at which voting closes and the results are final.
    pub end_time: u64,
}

impl Vote {
    /// Creates a new vote with the ID `id`, which ends at `end_time`
    pub fn new(id: u64, end_time: u64) -> Self {
        Vote {
            id,
            casual_yes: 0,
            casual_no: 0,
            comp_yes: 0,
            comp_no: 0,
            end_time,
        }
    }
    /// Gets the total yes and no votes, regardless of casual/comp.
    pub fn total(&self) -> (u64, u64) {
        (self.casual_yes + self.comp_yes, self.casual_no + self.comp_no)
    }
}
