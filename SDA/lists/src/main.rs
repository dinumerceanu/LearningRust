use crate::lists::queue::Queue;

mod lists;

fn main() {
    let mut list1 = Queue::new();
    list1.push(1);
    list1.push(2);
    list1.push(3);
    list1.push(4);
    list1.print_queue();
    list1.delete_queue();
    list1.print_queue();
    // list1.pop();
    // list1.print_queue();
    // list1.pop();
    // list1.pop();
    // list1.print_queue();
    // println!("{:?}", list1.pop());
    // println!("{:?}", list1.pop());
}
