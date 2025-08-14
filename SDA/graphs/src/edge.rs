#[derive(Debug, Eq)]
pub struct Edge {
    pub u: usize,
    pub v: usize,
    pub weight: i32,
}

impl Edge {
    pub fn new(u: usize, v: usize, weight: i32) -> Self {
        Edge {
            u,
            v,
            weight,
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Clone for Edge {
    fn clone(&self) -> Self {
        Edge { u: self.u.clone(), v: self.v.clone(), weight: self.weight.clone() }
    }
}