use crate::Cord;
use std::collections::HashSet;

pub type SandPosType = isize;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Sand {
    pub pos: Cord<SandPosType>,
}

impl Sand {
    pub fn fall(
        &mut self,
        rocks: &HashSet<Cord<SandPosType>>,
        sands: &HashSet<Cord<SandPosType>>,
    ) -> bool {
        // Try going down first.
        let next_pos = self.pos + [0, 1].into();
        if !rocks.contains(&next_pos) && !sands.contains(&next_pos) {
            self.pos = next_pos;
            return true;
        }
        // Try down left next.
        let next_pos = self.pos - [1, 0].into() + [0, 1].into();
        if !rocks.contains(&next_pos) && !sands.contains(&next_pos) {
            self.pos = next_pos;
            return true;
        }
        // Try down right next.
        let next_pos = self.pos + [1, 1].into();
        if !rocks.contains(&next_pos) && !sands.contains(&next_pos) {
            self.pos = next_pos;
            return true;
        }
        // If none worked then it can't go lower.
        false
    }
}
