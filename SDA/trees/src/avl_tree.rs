use core::borrow;
use std::{cell::{Ref, RefCell}, cmp::max, collections::VecDeque, io::Cursor, rc::Rc};

pub struct Node {
    val: i32,
    height: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            height: 1,
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
        self.root = self.insert_recursive(self.root.clone(), val);
    }

    fn insert_recursive(&mut self, node_opt: Option<Rc<RefCell<Node>>>, val: i32) -> Option<Rc<RefCell<Node>>> {
        match node_opt {
            None => Some(Rc::new(RefCell::new(Node::new(val)))),
            
            Some(node) => {
                {
                    let mut borrowed_node = node.borrow_mut();
                    if val < borrowed_node.val {
                        let new_left = self.insert_recursive(borrowed_node.left.clone(), val);
                        borrowed_node.left = new_left;
                    } else if val > borrowed_node.val {
                        let new_right = self.insert_recursive(borrowed_node.right.clone(), val);
                        borrowed_node.right = new_right;
                    } else {
                        drop(borrowed_node);
                        return Some(node);
                    }
                }

                self.update_height(&node);

                let balance = self.balance_factor(&node);

                if balance > 1 {
                    let left_child = node.borrow().left.as_ref().unwrap().clone();
                    if val < left_child.borrow().val {
                        return Some(self.LL(node));
                    } 
                    else {
                        return Some(self.LR(node));
                    }
                }

                if balance < -1 {
                    let right_child = node.borrow().right.as_ref().unwrap().clone();
                    if val > right_child.borrow().val {
                        return Some(self.RR(node));
                    } 
                    else {
                        return Some(self.RL(node));
                    }
                }

                Some(node)
            }
        }
    }

    pub fn delete(&mut self, val: i32) {
        self.root = self.delete_node(self.root.clone(), val);
    }

    fn delete_node(&mut self, node_opt: Option<Rc<RefCell<Node>>>, val: i32) -> Option<Rc<RefCell<Node>>> {
        match node_opt {
            None => None,

            Some(node) => {
                {
                    let mut borrowed_node = node.borrow_mut();
                    if val < borrowed_node.val {
                        let new_left = self.delete_node(borrowed_node.left.clone(), val);
                        borrowed_node.left = new_left;
                    } else if val > borrowed_node.val {
                        let new_right = self.delete_node(borrowed_node.right.clone(), val);
                        borrowed_node.right = new_right;
                    } else {
                        if borrowed_node.left.is_none() {
                            return borrowed_node.right.clone();
                        } else if borrowed_node.right.is_none() {
                            return borrowed_node.left.clone();
                        }
                        if let Some(smallest_bigger) = Self::find_smallest_node(borrowed_node.right.clone()) {
                            borrowed_node.val = smallest_bigger;
                            let new_right = self.delete_node(borrowed_node.right.clone(), smallest_bigger);
                            borrowed_node.right = new_right;
                        }
                    }
                }

                self.update_height(&node);

                let balance = self.balance_factor(&node);

                if balance > 1 {
                    let left_child = node.borrow().left.as_ref().unwrap().clone();
                    if self.balance_factor(&left_child) >= 0 {
                        return Some(self.LL(node));
                    } 
                    else {
                        return Some(self.LR(node));
                    }
                }

                if balance < -1 {
                    let right_child = node.borrow().right.as_ref().unwrap().clone();
                    if self.balance_factor(&right_child) <=0 {
                        return Some(self.RR(node));
                    } 
                    else {
                        return Some(self.RL(node));
                    }
                }

                Some(node)
            },
        }
    }

    pub fn search(&self, val: i32) -> bool {
        Self::search_recursive(self.root.clone(), val)
    }
    
    fn search_recursive(node_opt: Option<Rc<RefCell<Node>>>, val: i32) -> bool {
        match node_opt {
            None => false,

            Some(node) => {
                let mut borrowed_node = node.borrow_mut();
                if val == borrowed_node.val {
                    return true;
                } else if val < borrowed_node.val {
                    Self::search_recursive(borrowed_node.left.clone(), val)
                } else {
                    Self::search_recursive(borrowed_node.right.clone(), val)
                }
            },
        }
    }

    fn find_smallest_node(node_opt: Option<Rc<RefCell<Node>>>) -> Option<i32> {
        let mut current = node_opt.clone();
        
        while let Some(node) = current {
            if node.borrow().left.is_some() {
                current = node.borrow().left.clone();
            } else {
                return Some(node.borrow().val);
            }
        }

        None
    } 

    fn LL(&mut self, A: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let B = A.borrow().left.as_ref().unwrap().clone();
        let B_R = B.borrow().right.clone();

        A.borrow_mut().left = B_R;
        B.borrow_mut().right = Some(A.clone());

        self.update_height(&A);
        self.update_height(&B);
        
        B
    }

    fn RR(&mut self, A: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let B = A.borrow().right.as_ref().unwrap().clone();
        let B_L = B.borrow().left.clone();

        A.borrow_mut().right = B_L;
        B.borrow_mut().left = Some(A.clone());

        self.update_height(&A);
        self.update_height(&B);
        
        B
    }

    fn LR(&mut self, A: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let B = A.borrow().left.as_ref().unwrap().clone();
        
        A.borrow_mut().left = Some(self.RR(B));

        self.LL(A)
    }

    fn RL(&mut self, A: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let B = A.borrow().right.as_ref().unwrap().clone();

        A.borrow_mut().right = Some(self.LL(B));

        self.RR(A)
    }

    fn height(node: &Option<Rc<RefCell<Node>>>) -> i32 {
        node.as_ref().map_or(0, |n| n.borrow().height)
    }

    fn update_height(&mut self, node: &Rc<RefCell<Node>>) {
        let left_height = Self::height(&node.borrow().left);
        let right_height = Self::height(&node.borrow().right);

        node.borrow_mut().height = max(left_height, right_height) + 1;
    }

    fn balance_factor(&self, node: &Rc<RefCell<Node>>) -> i32 {
        Self::height(&node.borrow().left) - Self::height(&node.borrow().right)
    }
}