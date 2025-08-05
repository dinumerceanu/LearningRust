struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,   
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node {
            val,
            next: None,
        }
    }
}

pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Debug> Stack<T> {
    pub fn new() -> Self {
        Stack {
            head: None,
        }
    }

    pub fn push(&mut self, val: T) {
        let mut new_node = Box::new(Node::new(val));

        if let Some(head) = self.head.take() {
            new_node.next = Some(head);
            self.head = Some(new_node);
        } else {
            self.head = Some(new_node);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            self.head = head.next;
            return Some(head.val);
        } else {
            return None;
        }
    }

    pub fn top(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    pub fn print_stack(&self) {
        let mut iter = self.head.as_ref();

        while let Some(head) = iter {
            print!("{:?} ", head.val);
            iter = head.next.as_ref();
        }
        println!();
    }

    pub fn is_empty(&self) -> bool{
        if let Some(head) = self.head.as_ref() {
            return false;
        } else {
            return true;
        }
    }
}
