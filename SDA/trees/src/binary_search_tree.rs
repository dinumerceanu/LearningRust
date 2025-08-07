use std::collections::VecDeque;

pub struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }

    fn preorder_print(&self) {
        print!("{} ", self.val);

        if let Some(node) = &self.left {
            node.preorder_print();
        }

        if let Some(node) = &self.right {
            node.preorder_print();
        }

    }

    fn inorder_print(&self) {
        if let Some(node) = &self.left {
            node.inorder_print();
        }

        print!("{} ", self.val);

        if let Some(node) = &self.right {
            node.inorder_print();
        }
    }

    fn postorder_print(&self) {
        if let Some(node) = &self.left {
            node.postorder_print();
        }

        if let Some(node) = &self.right {
            node.postorder_print();
        }

        print!("{} ", self.val);
    }

    fn dfs_print(&self) {
        print!("{} ", self.val);

        if let Some(node) = &self.left {
            node.dfs_print();
        }

        if let Some(node) = &self.right {
            node.dfs_print();
        }
    }

    fn bfs_print(&self) {
        let mut queue = VecDeque::new();

        queue.push_back(self);

        while !queue.is_empty() {
            if let Some(current) = queue.pop_front() {
                print!("{} ", current.val);
                if let Some(kid) = &current.left {
                    queue.push_back(kid);
                }
                if let Some(kid) = &current.right {
                    queue.push_back(kid);
                }
            }
        }
    }

    fn bfs_print_on_levels(&self) {
        let mut queue = VecDeque::new();
        let mut n = 1;
        let mut m = 0;

        queue.push_back(self);

        while !queue.is_empty() {
            m = 0;
            for i in 0..n {
                if let Some(current) = queue.pop_front() {
                    print!("{} ", current.val);
                    if let Some(kid) = &current.left {
                        queue.push_back(kid);
                        m += 1;
                    }
                    if let Some(kid) = &current.right {
                        queue.push_back(kid);
                        m += 1;
                    }
                }
            }
            n = m;
            println!();
        }
    }
}

pub struct BSTree {
    root: Option<Box<Node>>,
}

impl BSTree {
    pub fn new() -> Self {
        BSTree { 
            root: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        if let Some(node) = &self.root {
            false
        } else {
            true
        }
    }

    pub fn insert(&mut self, val: i32) {
        let new_node = Box::new(Node::new(val));
        let mut current = &mut self.root;

        loop {
            match current {
                None => {
                    *current = Some(new_node);
                    return;
                },
                Some(node) => {
                    if val < node.val {
                        current = &mut node.left;
                    } else if val > node.val {
                        current = &mut node.right;
                    } else {
                        return;
                    }
                },
            }
        }
    }

    pub fn search(&self, val: i32) -> Option<&Node> {
        let mut current = &self.root;

        loop {
            match current {
                Some(node) => {
                    if node.val == val {
                        return Some(node);
                    } else if val < node.val{
                        current = &node.left;
                    } else {
                        current = &node.right;
                    }
                },
                None => return None,
            }
        }
    }

    pub fn preorder(&self) {
        if let Some(node) = &self.root {
            node.preorder_print();
            println!();
        } else {
            println!("Empty tree");
        }
    }

    pub fn inorder(&self) {
        if let Some(node) = &self.root {
            node.inorder_print();
            println!();
        } else {
            println!("Empty tree");
        }
    }

    pub fn postorder(&self) {
        if let Some(node) = &self.root {
            node.postorder_print();
            println!();
        } else {
            println!("Empty tree");
        }
    }

    pub fn dfs(&self) {
        if let Some(node) = &self.root {
            node.dfs_print();
            println!();
        } else {
            println!("Empty tree");
        }
    }

    pub fn bfs(&self) {
        if let Some(node) = &self.root {
            node.bfs_print();
            println!();
        } else {
            println!("Empty tree");
        }
    }

    pub fn bfs_on_levels(&self) {
        if let Some(node) = &self.root {
            node.bfs_print_on_levels();
            println!();
        } else {
            println!("Empty tree");
        }
    }
}
