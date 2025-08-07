struct Node {
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
}
