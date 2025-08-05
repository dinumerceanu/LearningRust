use crate::lists::queue::Queue;
use crate::lists::stack::Stack;

mod lists;

fn main() {
    // let mut list1 = Queue::new();
    // list1.push(1);
    // list1.push(2);
    // list1.push(3);
    // list1.push(4);
    // list1.print_queue();
    // list1.delete_queue();
    // list1.print_queue();
    // list1.pop();
    // list1.print_queue();
    // list1.pop();
    // list1.pop();
    // list1.print_queue();
    // println!("{:?}", list1.pop());
    // println!("{:?}", list1.pop());


    let mut list2 = Stack::new();
    list2.push(1);
    list2.print_stack();
    list2.push(2);
    list2.print_stack();
    list2.push(3);
    list2.print_stack();
    list2.pop();
    list2.print_stack();
    println!("{}", list2.is_empty());
    list2.pop();
    println!("{:?}", list2.pop());
    println!("{}", list2.is_empty());
    println!("{:?}", list2.top());
    list2.push(1);
    list2.push(2);
    list2.push(3);
    println!("{:?}", list2.top());
}
