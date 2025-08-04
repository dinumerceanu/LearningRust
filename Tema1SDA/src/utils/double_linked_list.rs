use std::io::StderrLock;
use std::rc::{self, Rc};
use std::cell::{Ref, RefCell};

type Link = Option<Rc<RefCell<ListNode>>>;
pub struct ListNode {
    pub val: char,
    pub next: Link,
    pub prev: Link,
}

impl ListNode {
    pub fn new(val: char) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListNode{
            val,
            next: None,
            prev: None,
        }))
    }
}

pub struct List {
    pub santinel: Link,
    pub mec_pos: Link,
}

impl List {
    pub fn new() -> Self {
        let santinel_node = ListNode::new(' ');
        {
            let mut santinel_borrowed = santinel_node.borrow_mut();
            santinel_borrowed.next = Some(Rc::clone(&santinel_node));
            santinel_borrowed.prev = Some(Rc::clone(&santinel_node));
        }

        let mut list: List = List{
            santinel: Some(santinel_node),
            mec_pos: None,
        };

        list.add_cart('#');
        list.mec_pos = list.santinel.as_ref().unwrap().borrow().next.clone();

        list
    }

    pub fn add_cart(&mut self, val: char) {
        let mut new_node = ListNode::new(val);
        
        let santinel_node = self.santinel.as_ref().unwrap();

        let old_last = santinel_node.borrow().prev.clone().unwrap();
        {
            let mut new_node_borrowed = new_node.borrow_mut();
            new_node_borrowed.prev = Some(Rc::clone(&old_last));
            new_node_borrowed.next = Some(Rc::clone(santinel_node));
        }
        santinel_node.borrow_mut().prev = Some(Rc::clone(&new_node));
        old_last.borrow_mut().next = Some(Rc::clone(&new_node));
    }
}

pub trait Showcase {
    fn print_list(&mut self);
}

impl Showcase for List {
    fn print_list(&mut self) {
        let santinel_node = self.santinel.as_ref().unwrap();
        let mecanic_node = self.mec_pos.as_ref().unwrap();
        
        let mut iter = santinel_node.borrow().next.clone();

        while let Some(node) = iter {
            if Rc::ptr_eq(&node, santinel_node) {
                break;
            }
            if Rc::ptr_eq(&node, mecanic_node) {
                print!("|{}| ", node.borrow().val);
            } else {
                print!("{} ", node.borrow().val);
            }
            iter = node.borrow().next.clone();
        }

        println!();
    }
}

pub trait Update {
    fn move_left(&mut self);

    fn move_right(&mut self);

    fn move_right_circular(&mut self);

    fn write(&mut self, new_val: char);

    fn insert_left(&mut self, symbol: char);

    fn insert_right(&mut self, symbol: char);

    fn clear_cell(&mut self);

    fn clear_all(&mut self);
}

impl Update for List {
    fn move_left(&mut self) {
        let mecanic_rc = self.mec_pos.clone().unwrap();

        let before_mec_rc = mecanic_rc.borrow().prev.clone().unwrap();

        let santinel_node = self.santinel.as_ref().unwrap();

        if !Rc::ptr_eq(&before_mec_rc, santinel_node) {
            self.mec_pos = Some(before_mec_rc);
        } else {
            let last_node = self.santinel.clone().unwrap().borrow().prev.clone().unwrap();
            self.mec_pos = Some(last_node);
        }
    }

    fn move_right(&mut self) {
        let mecanic_rc = self.mec_pos.clone().unwrap();
        let after_mec_rc = mecanic_rc.borrow().next.clone().unwrap();
        let santinel_node = self.santinel.as_ref().unwrap();

        if Rc::ptr_eq(&after_mec_rc, santinel_node) {
            self.add_cart('#');
            let last_node = self.santinel.clone().unwrap().borrow().prev.clone().unwrap();
            self.mec_pos = Some(last_node);
        } else {
            self.mec_pos = Some(after_mec_rc);
        }
    }

    fn move_right_circular(&mut self) {
        let mecanic_rc = self.mec_pos.clone().unwrap();
        let after_mec_rc = mecanic_rc.borrow().next.clone().unwrap();
        let santinel_node = self.santinel.as_ref().unwrap();

        if Rc::ptr_eq(&after_mec_rc, santinel_node) {
            let first_node = self.santinel.clone().unwrap().borrow().next.clone().unwrap();
            self.mec_pos = Some(first_node);
        } else {
            self.mec_pos = Some(after_mec_rc);
        }
    }

    fn write(&mut self, new_val: char) {
        self.mec_pos.as_ref().unwrap().borrow_mut().val = new_val;
    }

    fn insert_left(&mut self, symbol: char) {
        let mecanic_rc = self.mec_pos.clone().unwrap();
        let prev_mecanic_rc = mecanic_rc.borrow().prev.clone().unwrap();
        let santinel_node = self.santinel.as_ref().unwrap();

        if Rc::ptr_eq(&prev_mecanic_rc, santinel_node) {
            println!("ERROR");
        } else {
            let mut new_node = ListNode::new(symbol);
            let mecanic_rc = self.mec_pos.clone().unwrap();
            let prev_mecanic_rc = mecanic_rc.borrow_mut().prev.clone().unwrap();
            {
                let mut new_node_borrowed = new_node.borrow_mut();
                new_node_borrowed.next = Some(Rc::clone(&mecanic_rc));
                new_node_borrowed.prev = Some(Rc::clone(&prev_mecanic_rc));
            }
            prev_mecanic_rc.borrow_mut().next = Some(Rc::clone(&new_node));
            mecanic_rc.borrow_mut().prev = Some(Rc::clone(&new_node));
            self.move_left();
        }
    }

    fn insert_right(&mut self, symbol: char) {
        let mecanic_rc = self.mec_pos.clone().unwrap();
        let next_mecanic_rc = mecanic_rc.borrow().next.clone().unwrap();
        let santinel_node = self.santinel.as_ref().unwrap();

        if Rc::ptr_eq(&next_mecanic_rc, santinel_node) {
            self.add_cart(symbol);
        } else {
            let new_node = ListNode::new(symbol);
            let mut new_node_borrowed = new_node.borrow_mut();
            new_node_borrowed.next = Some(Rc::clone(&next_mecanic_rc));
            new_node_borrowed.prev = Some(Rc::clone(&mecanic_rc));
            next_mecanic_rc.borrow_mut().prev = Some(Rc::clone(&new_node));
            mecanic_rc.borrow_mut().next = Some(Rc::clone(&new_node));
        }
        self.move_right();
    }

    fn clear_cell(&mut self) {
        let mecanic_rc = self.mec_pos.clone().unwrap();
        let next_mecanic_rc = mecanic_rc.borrow().next.clone().unwrap();
        let prev_mecanic_rc = mecanic_rc.borrow().prev.clone().unwrap();
        let santinel_node = self.santinel.as_ref().unwrap();

        if Rc::ptr_eq(&next_mecanic_rc, santinel_node) && Rc::ptr_eq(&prev_mecanic_rc, santinel_node) {
            // este singurul
            self.write('#');
        } else if Rc::ptr_eq(&next_mecanic_rc, santinel_node) {
            // este ultimul
            prev_mecanic_rc.borrow_mut().next = Some(Rc::clone(&santinel_node));
            santinel_node.borrow_mut().prev = Some(Rc::clone(&prev_mecanic_rc));
            self.move_left();
        } else if Rc::ptr_eq(&prev_mecanic_rc, santinel_node) {
            // este primul
            next_mecanic_rc.borrow_mut().prev = Some(Rc::clone(&santinel_node));
            santinel_node.borrow_mut().next = Some(Rc::clone(&next_mecanic_rc));
            self.move_left();
        } else {
            // la mijloc
            prev_mecanic_rc.borrow_mut().next = Some(Rc::clone(&next_mecanic_rc));
            next_mecanic_rc.borrow_mut().prev = Some(Rc::clone(&prev_mecanic_rc));
            self.move_left();
        }
    }

    fn clear_all(&mut self) {
        loop {
            let mecanic_rc = self.mec_pos.clone().unwrap();
            let next_mecanic_rc = mecanic_rc.borrow().next.clone().unwrap();
            let prev_mecanic_rc = mecanic_rc.borrow().prev.clone().unwrap();
            let santinel_node = self.santinel.as_ref().unwrap();
            
            if Rc::ptr_eq(&next_mecanic_rc, santinel_node) && Rc::ptr_eq(&prev_mecanic_rc, santinel_node) {
                self.clear_cell();
                break;
            }

            self.clear_cell();
        }
    }
}

pub trait Search {
    fn search_right(&mut self, pattern: String);

    fn search(&mut self, pattern: String);
}

impl Search for List {
    fn search_right(&mut self, pattern: String) {
        let mut current = String::new();

        let mut iter =  self.mec_pos.clone();
        let santinel_node = self.santinel.as_ref().unwrap();

        while let Some(node) = iter {
            let next_node = node.borrow().next.clone().unwrap();

            if Rc::ptr_eq(&node, santinel_node) {
                break;
            }

            current.push(node.borrow().val);

            iter = Some(next_node);
        }

        if let Some(index) = current.find(pattern.as_str()) {
            let steps = index + pattern.len() - 1;
            for i in 0..steps {
                self.move_right();
            }
        } else {
            println!("ERROR");
        }

    }

    fn search(&mut self, pattern: String) {
        let mut current = String::new();

        let mecanic_node = self.mec_pos.clone().unwrap().borrow().prev.clone().unwrap();
        let mut iter = self.mec_pos.clone();
        let santinel_node = self.santinel.as_ref().unwrap();

        while let Some(node) = &iter {
            let next_node = node.borrow().next.clone().unwrap();

            if Rc::ptr_eq(node, santinel_node) {
                iter = Some(next_node);
                continue;
            }

            if Rc::ptr_eq(node, &mecanic_node) {
                let iter_value = node.borrow().val;
                current.push(iter_value);
                break;  
            }

            let iter_value = node.borrow().val;
            current.push(iter_value);

            iter = Some(next_node);
        }

        if let Some(index) = current.find(pattern.as_str()) {
            let steps = index + pattern.len() - 1;
            for _ in 0..steps {
                self.move_right_circular();
            }
        } else {
            println!("ERROR");
        }

    }
}
