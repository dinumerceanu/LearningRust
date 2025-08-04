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

pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current_node_ref = &mut head;

    loop {
        let (n1, n2) = (list1.as_mut(), list2.as_mut());

        match (n1, n2) {
            (Some(node1), Some(node2)) => {
                if node1.val <= node2.val {
                    let mut node = list1.take().unwrap();
                    list1 = node.next.take();
                    *current_node_ref = Some(node);
                    current_node_ref = &mut current_node_ref.as_mut().unwrap().next;
                } else {
                    let mut node = list2.take().unwrap();
                    list2 = node.next.take();
                    *current_node_ref = Some(node);
                    current_node_ref = &mut current_node_ref.as_mut().unwrap().next;
                }
            },
            (Some(_), None) => {
                *current_node_ref = list1.take();
                break;
            },
            (None, Some(_)) => {
                *current_node_ref = list2.take();
                break;
            },
            (None, None) => break,
        }
    }

    head
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

fn main() {
    let mut list1 = make_list();
    let mut list2 = make_list();
    
    print_list(&merge_two_lists(list1, list2));
}