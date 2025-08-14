pub struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>
}

impl DSU {
    pub fn new(n: usize) -> Self {
        DSU {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        self.parent[i] = self.find(self.parent[i]);
        self.parent[i]
    }

    pub fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            if self.size[root_i] < self.size[root_j] {
                self.parent[i] = root_j;
                self.size[root_j] += self.size[root_i];
            } else {
                self.parent[j] = root_i;
                self.size[root_i] += self.size[root_j];
            }
            true
        } else {
            false
        }
    }

    pub fn print(&self) {
        println!("{:?}", self.parent);
        println!("{:?}", self.size);
    }
}
