use std::collections::HashSet;

use crate::cord::Cord;

struct Sand {
    pos: Cord<usize>,
}

impl Sand {
    fn lower(&mut self, rocks: &HashSet<Cord<usize>>, sands: &HashSet<Cord<usize>>) -> bool {
        // Try going down first.
        let next_pos = self.pos - (0, 1).into();
        if rocks.contains(&next_pos) {
            // Try down left next.
            let next_pos = self.pos - (1, 1).into();
            if rocks.contains(&next_pos) {
                // Try down right next.
                let next_pos = self.pos + (1, -1).into();
                if rocks.contains(&next_pos) {}
            }
        }
    }
}
