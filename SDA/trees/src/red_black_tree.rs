use std::{cell::RefCell, char::REPLACEMENT_CHARACTER, collections::VecDeque, rc::Rc};

use colored::Colorize;

pub struct Node {
    val: i32,
    red: bool,
    parent: Option<Rc<RefCell<Node>>>, 
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
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
    pub root: Option<Rc<RefCell<Node>>>,
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

    fn rebalance_delete(&mut self, node_to_delete: Rc<RefCell<Node>>) {
        let mut parent_opt = node_to_delete.clone().borrow().parent.clone();
        if parent_opt.is_none() {
            node_to_delete.borrow_mut().red = false;
            return;
        }

        let node_is_left_child = parent_opt.clone().unwrap().borrow().right.as_ref().map_or(true, |right| !Rc::ptr_eq(right, &node_to_delete));
        
        let mut parent = parent_opt.clone().unwrap();
        let mut sibling = if node_is_left_child {
            parent.borrow().right.clone()
        } else {
            parent.borrow().left.clone()
        };

        let mut sibling_exists = false;
        let mut sibling_is_red = false;
        let mut sibling_left_exists = false;
        let mut sibling_left_is_red = false;
        let mut sibling_right_exists = false;
        let mut sibling_right_is_red = false;

        if let Some(sibling_rc) = sibling.clone() {
            sibling_exists = true;
            sibling_is_red = (sibling_rc.borrow().red == true);
            if let Some(_) = sibling_rc.borrow().left.clone() {
                sibling_left_exists = true;
                sibling_left_is_red = (sibling_rc.borrow().left.clone().unwrap().borrow().red == true);
            } else {
                sibling_left_exists = false;
                sibling_left_is_red = false;
            }
            if let Some(_) = sibling_rc.borrow().right.clone() {
                sibling_right_exists = true;
                sibling_right_is_red = (sibling_rc.borrow().right.clone().unwrap().borrow().red == true);
            } else {
                sibling_right_is_red = false;
                sibling_right_exists = false;
            }
        } else {
            sibling_exists = false;
            sibling_is_red = false;
            sibling_left_is_red = false;
            sibling_right_is_red = false;
        }

        if node_is_left_child == true {
            if sibling_exists && sibling_is_red {
                sibling.clone().unwrap().borrow_mut().red = false;
                parent.borrow_mut().red = true;
                self.RR(parent.clone());
            }

            if !sibling_exists {
                // nu exista sibling
                if parent.borrow().red == true {
                    parent.borrow_mut().red = false;
                    return;
                } else {
                    self.rebalance_delete(parent.clone());
                }
            } else {
                // exista sibling
                if !sibling_is_red && !sibling_left_is_red && !sibling_right_is_red {
                    sibling.clone().unwrap().borrow_mut().red = true;
                    self.rebalance_delete(parent.clone());
                } else {
                    if !sibling_is_red && sibling_left_is_red && !sibling_right_is_red {
                        sibling.clone().unwrap().borrow_mut().left.clone().unwrap().borrow_mut().red = false;
                        sibling.clone().unwrap().borrow_mut().red = true;
                        self.LL(sibling.clone().unwrap());
                        self.rebalance_delete(node_to_delete);

                    } else if sibling_right_exists && sibling_right_is_red {
                        sibling.clone().unwrap().borrow_mut().red = parent.borrow().red;
                        parent.borrow_mut().red = false;
                        sibling.clone().unwrap().borrow_mut().right.clone().unwrap().borrow_mut().red = false;
                        self.RR(parent.clone());
                    }
                }

            }
        } else {
            if sibling_exists && sibling_is_red {
                sibling.clone().unwrap().borrow_mut().red = false;
                parent.borrow_mut().red = true;
                self.LL(parent.clone());
            }

            if !sibling_exists {
                // nu exista sibling
                if parent.borrow().red == true {
                    parent.borrow_mut().red = false;
                    return;
                } else {
                    self.rebalance_delete(parent.clone());
                }
            } else {
                // exista sibling
                if !sibling_is_red && !sibling_left_is_red && !sibling_right_is_red {
                    sibling.clone().unwrap().borrow_mut().red = true;
                    self.rebalance_delete(parent.clone());
                } else {
                    if !sibling_is_red && !sibling_left_is_red && sibling_right_is_red {
                        sibling.clone().unwrap().borrow_mut().right.clone().unwrap().borrow_mut().red = false;
                        sibling.clone().unwrap().borrow_mut().red = true;
                        self.RR(sibling.clone().unwrap());
                    } else if sibling_right_exists && sibling_right_is_red {
                        sibling.clone().unwrap().borrow_mut().red = parent.borrow().red;
                        parent.borrow_mut().red = false;
                        sibling.clone().unwrap().borrow_mut().left.clone().unwrap().borrow_mut().red = false;
                        self.LL(parent.clone());
                    }
                }

            }
        }
        // ensore final node is black
    }

    pub fn delete(&mut self, val: i32) {
        let mut node_to_delete: Rc<RefCell<Node>>;
        let node_is_left_child: bool;

        if let Some(x) = Self::search_recursive(self.root.clone(), val) {
            node_to_delete = x;
        } else {
            return;
        }

        let parent_opt = node_to_delete.clone().borrow().parent.clone();
        
        match Self::nr_of_kids(Some(node_to_delete.clone())) {
            0 => {
                let borrowed_node = node_to_delete.borrow_mut();
                
                if parent_opt.is_none() {
                    self.root = None;
                } else {
                    let node_is_left_child = parent_opt.clone().unwrap().borrow().left.as_ref().map_or(false, |left| Rc::ptr_eq(&left, &node_to_delete.clone()));
                    if node_is_left_child {
                        parent_opt.clone().unwrap().borrow_mut().left = None;
                    } else {
                        parent_opt.clone().unwrap().borrow_mut().right = None;
                    }
                }
                
                if !borrowed_node.red {
                    drop(parent_opt);
                    drop(borrowed_node);
                    self.rebalance_delete(node_to_delete.clone());
                }
            },
            1 => {
                let child_is_left = node_to_delete.clone().borrow().left.is_some();

                let mut child = if child_is_left {
                    node_to_delete.clone().borrow().left.clone().unwrap()
                } else {
                    node_to_delete.clone().borrow().right.clone().unwrap()
                };

                let node_is_left_child = parent_opt.clone().unwrap().borrow().left.clone().map_or(false, |left| Rc::ptr_eq(&left, &node_to_delete.clone()));
                if node_is_left_child {
                    parent_opt.clone().unwrap().borrow_mut().left = Some(child.clone());
                    node_to_delete.borrow_mut().left = None;
                } else {
                    parent_opt.clone().unwrap().borrow_mut().right = Some(child.clone());
                    node_to_delete.borrow_mut().right = None;
                }
                child.borrow_mut().parent = parent_opt;
                child.borrow_mut().red = false;                
            },
            2 => {
                let middle_man = node_to_delete.clone().borrow().right.clone();
                let mut smallest_bigger = Self::find_smallest_node(middle_man).unwrap();
                let parent_of_new_node = smallest_bigger.clone().borrow().parent.clone();
                let new_node_is_left_child = parent_of_new_node.clone().unwrap().borrow().left.clone().map_or(false, |left| Rc::ptr_eq(&left, &smallest_bigger.clone()));
                node_to_delete.clone().borrow_mut().val = smallest_bigger.clone().borrow().val;
                if new_node_is_left_child {
                    parent_of_new_node.unwrap().borrow_mut().left = None;
                } else {
                    parent_of_new_node.unwrap().borrow_mut().right = None;
                }

                if !node_to_delete.borrow().red {
                    self.rebalance_delete(node_to_delete.clone());
                }
            }
            _ => {
                return;
            }
        }
    }

    pub fn search(&self, val: i32) -> Option<Rc<RefCell<Node>>> {
        if let Some(found) = Self::search_recursive(self.root.clone(), val) {
            println!("FOUND!");
            Some(found)
        } else {
            println!("NOT FOUND!");
            None
        }
    }

    fn search_recursive(start: Option<Rc<RefCell<Node>>>, val: i32) -> Option<Rc<RefCell<Node>>> {
        if let Some(current) = start {
            let mut borrowed_current = current.borrow();
            if val == borrowed_current.val {
                drop(borrowed_current);
                return Some(current);
            } else if val < borrowed_current.val {
                let next_node = borrowed_current.left.clone();
                drop(borrowed_current);
                Self::search_recursive(next_node, val)
            } else {
                let next_node = borrowed_current.right.clone();
                drop(borrowed_current);
                Self::search_recursive(next_node, val)
            }
        } else {
            None
        }
    }

    pub fn nr_of_kids(node: Option<Rc<RefCell<Node>>>) -> i32 {
        if node.is_some() {
            let mut kids = 0;
            if let Some(_) = node.as_ref().unwrap().borrow().left {
                kids += 1;
            }
            if let Some(_) = node.unwrap().borrow().right {
                kids += 1;
            }
            kids
        } else {
            -1
        }
    } 

    fn find_smallest_node(node_opt: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut current = node_opt.clone();
        
        while let Some(node) = current {
            if node.borrow().left.is_some() {
                current = node.borrow().left.clone();
            } else {
                return Some(node);
            }
        }

        None
    }
}
