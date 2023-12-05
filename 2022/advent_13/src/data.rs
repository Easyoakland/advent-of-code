use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Integer(x) => write!(f, "{}", x),
            Packet::List(x) => {
                write!(f, "[")?;
                if !x.is_empty() {
                    for packet_idx in 0..x.len() - 1 {
                        write!(f, "{},", x[packet_idx])?;
                    }
                    write!(f, "{}", x.last().unwrap())?;
                }
                write!(f, "]")?;
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub struct Pair {
    pub left: Packet,
    pub right: Packet,
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n{}", self.left, self.right)
    }
}

// Some elements can be equated like x == x, x!=y
impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::List(l0), Self::Integer(r0)) => l0 == &vec![Self::Integer(*r0)],
            (l, r) => r == l,
        }
    }
}

// All elements can be equated.
impl Eq for Packet {}

// Some elements can be compared like x<=y, x<y, x>y, etc...
// This implementation uses the Ord implementation.
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// All elements can be compared.
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Integer(l), Packet::Integer(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => l.cmp(r),
            (Packet::List(l), Packet::Integer(r)) => l.cmp(&vec![Packet::Integer(*r)]),
            (Packet::Integer(l), Packet::List(r)) => vec![Packet::Integer(*l)].cmp(r),
        }
    }
}
