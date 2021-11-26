use crate::basic::Level;

struct User {
    score: u16,
    correct: u8,
    wrong: u8,

    //TODO: Implement a ranking system
    //ranking: ??
}

/// Represents the end user - the person playing the regex game. Keeps track of all relevant data during
/// their journey.
impl User {

    /// Called when a user answers correctly
    /// qta = (Q)uestion (T)ype (A)nswered - Can be Easy, Medium, or Hard
    /// Depending on the level of difficulty of their question, this will determine how much to
    /// increment their score by.
    fn increment(&mut self, qta: Level) {
        //TODO: Current implementation is basic and not correct, see above documentation
        self.score += 1;
    }

    fn pct(&self) -> f32 {
        //TODO: Returns a pct % based on correct / wrong ratio.
        unimplemented!()
    }
}