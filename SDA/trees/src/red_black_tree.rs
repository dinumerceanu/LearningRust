use std::{cell::{Ref, RefCell}, collections::VecDeque, rc::Rc};

use colored::Colorize;

pub struct Node {
    val: i32,
    red: bool,
    parent: Option<Rc<RefCell<Node>>>, 
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            red: true,
            parent: None,
            left: None,
            right: None,
        }
    }
}

pub struct RBTree {
    root: Option<Rc<RefCell<Node>>>,
}

impl RBTree {
    pub fn new() -> Self {
        RBTree {
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
                    // if current.borrow().parent.is_some() {
                    //     println!("Parent of {} is {}", current.borrow().val, current.borrow().parent.as_ref().clone().unwrap().borrow().val);
                    // } else {
                    //     println!("{} is root", current.borrow().val);
                    // }
                    if current.borrow().red == true {
                        print!("{} ", current.borrow().val.to_string().red());
                    } else {
                        print!("{} ", current.borrow().val.to_string().black());
                    }
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

    // pub fn test_rotations(&mut self) {
    //     let node_to_rotate = self.root.as_ref().unwrap().borrow().right.clone();
    //     if let Some(node) = node_to_rotate {
    //         self.RR(node);
    //     }
    // }

    fn LL(&mut self, A: Rc<RefCell<Node>>) {
        let A_parent_opt = A.borrow().parent.clone();
        let B = A.borrow().left.as_ref().unwrap().clone();

        {
            let mut a_mut = A.borrow_mut();
            let B_R_opt = B.borrow().right.clone();
            a_mut.left = B_R_opt.clone();
            drop(a_mut);

            if let Some(B_R) = B_R_opt {
                B_R.borrow_mut().parent = Some(A.clone());
            }
        }

        {
            let mut b_mut = B.borrow_mut();
            b_mut.right = Some(A.clone());
        }
        A.borrow_mut().parent = Some(B.clone());

        match A_parent_opt {
            Some(A_parent) => {
                let mut parent_mut = A_parent.borrow_mut();
                if let Some(left_child) = &parent_mut.left {
                    if Rc::ptr_eq(left_child, &A) {
                        parent_mut.left = Some(B.clone());
                    } else {
                        parent_mut.right = Some(B.clone());
                    }
                } else {
                    parent_mut.right = Some(B.clone());
                }
                drop(parent_mut);
                B.borrow_mut().parent = Some(A_parent);
            }
            None => {
                B.borrow_mut().parent = None;
                self.root = Some(B.clone());
            }
        }
    }

    fn RR(&mut self, A: Rc<RefCell<Node>>) {
        let A_parent_opt = A.borrow().parent.clone();
        let B = A.borrow().right.as_ref().unwrap().clone();

        {
            let mut a_mut = A.borrow_mut();
            let B_L_opt = B.borrow().left.clone();
            a_mut.right = B_L_opt.clone();
            drop(a_mut);

            if let Some(B_L) = B_L_opt {
                B_L.borrow_mut().parent = Some(A.clone());
            }
        }

        {
            let mut b_mut = B.borrow_mut();
            b_mut.left = Some(A.clone());
        }
        A.borrow_mut().parent = Some(B.clone());

        match A_parent_opt {
            Some(A_parent) => {
                let mut parent_mut = A_parent.borrow_mut();
                if let Some(left_child) = &parent_mut.left {
                    if Rc::ptr_eq(left_child, &A) {
                        parent_mut.left = Some(B.clone());
                    } else {
                        parent_mut.right = Some(B.clone());
                    }
                } else {
                    parent_mut.right = Some(B.clone());
                }
                drop(parent_mut);
                B.borrow_mut().parent = Some(A_parent);
            }
            None => {
                B.borrow_mut().parent = None;
                self.root = Some(B.clone());
            }
        }
    }

    fn rebalance(&mut self, mut node: Rc<RefCell<Node>>) {
        loop {
            let parent = {
                let node_borrow = node.borrow();
                if let Some(p) = &node_borrow.parent {
                    if !p.borrow().red {
                        break;
                    }
                    p.clone()
                } else {
                    break;
                }
            };

            let grandparent = parent.borrow().parent.as_ref().unwrap().clone();
            let parent_is_left_child = grandparent.borrow().left.as_ref().map_or(false, |left| Rc::ptr_eq(left, &parent));

            if parent_is_left_child {
                let uncle = grandparent.borrow().right.clone();
                if uncle.is_some() && uncle.as_ref().unwrap().borrow().red {
                    parent.borrow_mut().red = false;
                    uncle.unwrap().borrow_mut().red = false;
                    grandparent.borrow_mut().red = true;
                    node = grandparent;
                    continue;
                } else {
                    let node_is_right_child = parent.borrow().right.as_ref().map_or(false, |right| Rc::ptr_eq(right, &node));
                    if node_is_right_child {
                        let old_parent = parent.clone();
                        self.RR(old_parent.clone());
                        node = old_parent;
                    }

                    let current_parent = node.borrow().parent.as_ref().unwrap().clone();

                    current_parent.borrow_mut().red = false;
                    grandparent.borrow_mut().red = true;
                    self.LL(grandparent);
                    
                    break;
                }
            } else {
                let uncle = grandparent.borrow().left.clone();
                if uncle.is_some() && uncle.as_ref().unwrap().borrow().red {
                    parent.borrow_mut().red = false;
                    uncle.unwrap().borrow_mut().red = false;
                    grandparent.borrow_mut().red = true;
                    node = grandparent;
                    continue;
                } else {
                    let node_is_left_child = parent.borrow().left.as_ref().map_or(false, |left| Rc::ptr_eq(left, &node));
                    if node_is_left_child {
                        let old_parent = parent.clone();
                        self.LL(old_parent.clone());
                        node = old_parent;
                    }
                    
                    let current_parent = node.borrow().parent.as_ref().unwrap().clone();

                    current_parent.borrow_mut().red = false;
                    grandparent.borrow_mut().red = true;
                    self.RR(grandparent);
                    
                    break;
                }
            }
        }

        if let Some(root_node) = &self.root {
            root_node.borrow_mut().red = false;
        }
    }

    pub fn insert(&mut self, val: i32) {
        let new_node_rc = Rc::new(RefCell::new(Node::new(val)));
        
        if self.root.is_none() {
            self.root = Some(new_node_rc);
            self.root.as_ref().unwrap().borrow_mut().red = false;
            return;
        }

        let mut parent: Rc<RefCell<Node>>;
        let mut current = self.root.as_ref().unwrap().clone();
        let mut next_node: Rc<RefCell<Node>>;

        loop {
            let mut borrowed_current = current.borrow_mut();
            parent = current.clone();
            
            if val == borrowed_current.val {
                return;
            } else if val < borrowed_current.val {
                if borrowed_current.left.is_none() {
                    new_node_rc.borrow_mut().parent = Some(current.clone());
                    borrowed_current.left = Some(new_node_rc);

                    let rbnode = borrowed_current.left.clone();
                    drop(borrowed_current);
                    self.rebalance(rbnode.unwrap());

                    return;
                } else {
                    next_node = borrowed_current.left.as_ref().unwrap().clone();
                    drop(borrowed_current);
                    current = next_node;
                }
            } else {
                if borrowed_current.right.is_none() {
                    new_node_rc.borrow_mut().parent = Some(current.clone());
                    borrowed_current.right = Some(new_node_rc);

                    let rbnode = borrowed_current.right.clone();
                    drop(borrowed_current);
                    self.rebalance(rbnode.unwrap());

                    return;
                } else {
                    next_node = borrowed_current.right.as_ref().unwrap().clone();
                    drop(borrowed_current);
                    current = next_node;
                }
            }
        }
    }
}
