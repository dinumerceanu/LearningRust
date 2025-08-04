use std::io;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn print_list(head: &Option<Box<ListNode>>) {
    let mut current = head;

    while let Some(node) = current {
        print!("{} ", node.val);
        current = &node.next
    }
    println!();
}

fn make_list() -> Option<Box<ListNode>> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Eroare la citire");

    let mut arr: Vec<i32> = input
        .trim()
        .split(' ')
        .map(|num| num.parse::<i32>().unwrap())
        .collect();

    let mut head: Option<Box<ListNode>> = if let Some(first_num) = arr.first().copied() {
        arr.remove(0);
        Some(Box::new(ListNode::new(first_num)))
    } else {
        None
    };

    if head.is_none() {
        println!("Lista este goala.");
        return None;
    }

    let mut current = head.as_mut();
    for num in &arr {
        if let Some(node) = current {
            node.next = Some(Box::new(ListNode::new(*num)));
            current = node.next.as_mut();
        }
    }

    head
}

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut current = &head;
    let mut stack: Vec<i32> = Vec::new();

    while let Some(node) = current {
        stack.push(node.val);
        current = &node.next;
    }

    let mut current = &head;
    while let Some(node) = current {
        let y = stack.pop().unwrap();
        if node.val != y {
            return false;
        }
        current = &node.next;
    }

    true
}

fn main() {
    let mut list1 = make_list();
    
    println!("{}", is_palindrome(list1));
}
