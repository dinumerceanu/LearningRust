use std::{cell::{Ref, RefCell}, rc::Rc};

struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            next: None,
        }
    }
}

pub struct Queue {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: None,
        }
    }

    pub fn push(&mut self, val: i32) {
        let new_node = Rc::new(RefCell::new(Node::new(val)));
        let head_node = self.head.clone();

        if let Some(head) = head_node {
            self.tail.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
        } else {
            self.head = Some(Rc::clone(&new_node));
            self.tail = Some(new_node);
        }
        
    }

    pub fn pop(&mut self) -> Option<i32> {
        let head_rc = self.head.take()?;

        let val = head_rc.borrow().val;

        self.head = head_rc.borrow().next.clone();

        Some(val)
    }

    pub fn delete_queue(&mut self) {
        while let Some(node) = self.head.take() {
            self.head = node.borrow_mut().next.take();
        }
        self.tail = None;
    }

    pub fn print_queue(&self) {
        let mut iter = self.head.clone();

        while let Some(current) = iter {
            print!("{} ", current.borrow().val);
            iter = current.borrow().next.clone();
        }

        println!()
    }
}
