use std::{cell::{Ref, RefCell}, collections::VecDeque, rc::Rc};

pub struct Node {
    val: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct AVLTree {
    pub root: Option<Rc<RefCell<Node>>>,
}

impl AVLTree {
    pub fn new() -> Self {
        AVLTree {
            root: None,
        }
    }

    pub fn bfs_print_on_levels(&self) {
        let mut queue: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
        let mut n = 1;
        let mut m = 0;

        queue.push_back(self.root.as_ref().unwrap().clone());

        while !queue.is_empty() {
            m = 0;
            for _ in 0..n {
                if let Some(current) = queue.pop_front() {
                    print!("{} ", current.borrow().val);
                    if let Some(kid) = &current.borrow().left {
                        queue.push_back(kid.clone());
                        m += 1;
                    }
                    if let Some(kid) = &current.borrow().right {
                        queue.push_back(kid.clone());
                        m += 1;
                    }
                }
            }
            n = m;
            println!();
        }
    }

    pub fn insert(&mut self, val: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(val)));

        if self.root.is_none() {
            self.root = Some(new_node);
            return;
        }

        let mut current = self.root.as_ref().unwrap().clone();
        let mut next_node: Rc<RefCell<Node>>;

        loop {
            {
                let mut borrowed_current = current.borrow_mut();
                if val == borrowed_current.val {
                    return;
                } else if val < borrowed_current.val {
                    if borrowed_current.left.is_none() {
                        borrowed_current.left = Some(new_node);
                        return;
                    } else {
                        next_node = borrowed_current.left.as_ref().unwrap().clone();
                    }
                } else {
                    if borrowed_current.right.is_none() {
                        borrowed_current.right = Some(new_node);
                        return;
                    } else {
                        next_node = borrowed_current.right.as_ref().unwrap().clone();
                    }
                }
            }
            current = next_node;
        }
        
    }



    pub fn insert_recursive_wrapper(&mut self, val: i32) {
        let new_root = Self::insert_recursive(self.root.clone(), val);
        self.root = new_root;
    }

    fn insert_recursive(node_opt: Option<Rc<RefCell<Node>>>, val: i32) -> Option<Rc<RefCell<Node>>> {
        match node_opt {
            None => Some(Rc::new(RefCell::new(Node::new(val)))),
            
            Some(node) => {
                let mut borrowed_node = node.borrow_mut();
                
                if val < borrowed_node.val {
                    let left_subtree = borrowed_node.left.clone();
                    let new_left_subtree = Self::insert_recursive(left_subtree, val);
                    borrowed_node.left = new_left_subtree;
                } else if val > borrowed_node.val {
                    let right_subtree = borrowed_node.right.clone();
                    let new_right_subtree = Self::insert_recursive(right_subtree, val);
                    borrowed_node.right = new_right_subtree;
                }

                Some(node.clone())
            }
        }
    }

    pub fn LL(&mut self, A: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let B = A.borrow().left.as_ref().unwrap().clone();
        let B_R = B.borrow().right.clone();

        A.borrow_mut().left = B_R;

        B.borrow_mut().right = Some(A);
        
        B
    }

    pub fn RR(&mut self, A: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let B = A.borrow().right.as_ref().unwrap().clone();
        let B_L = B.borrow().left.clone();

        A.borrow_mut().right = B_L;

        B.borrow_mut().left = Some(A);
        
        B
    }

    pub fn LR(&mut self, A: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let B = A.borrow().left.as_ref().unwrap().clone();
        
        A.borrow_mut().left = Some(self.RR(B));

        self.LL(A)
    }

    pub fn RL(&mut self, A: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let B = A.borrow().right.as_ref().unwrap().clone();

        A.borrow_mut().right = Some(self.LL(B));

        self.RR(A)
    }
}