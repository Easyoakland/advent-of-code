use crate::cord::Cord;

#[derive(Debug)]
pub struct Pair<T> {
    pub sensor: Cord<T>,
    pub beacon: Cord<T>,
}
