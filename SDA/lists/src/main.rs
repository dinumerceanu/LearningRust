use crate::lists::queue::Queue;
use crate::lists::stack::Stack;
use crate::lists::heap::Heap;

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


    // let mut list2 = Stack::new();
    // list2.push(1);
    // list2.print_stack();
    // list2.push(2);
    // list2.print_stack();
    // list2.push(3);
    // list2.print_stack();
    // list2.pop();
    // list2.print_stack();
    // println!("{}", list2.is_empty());
    // list2.pop();
    // println!("{:?}", list2.pop());
    // println!("{}", list2.is_empty());
    // println!("{:?}", list2.top());
    // list2.push(1);
    // list2.push(2);
    // list2.push(3);
    // println!("{:?}", list2.top());

    let mut list3 = Heap::new();
    list3.insert(50);
    list3.insert(43);
    list3.insert(30);
    list3.insert(17);
    list3.insert(40);
    list3.insert(12);
    list3.insert(11);
    list3.insert(7);
    list3.insert(4);
    list3.insert(6);
    list3.insert(13);
    list3.print_heap();

    list3.delete();
    list3.print_heap();

    list3.build_heap(vec![-1, 1, 5, 0, 0, 4, 6, 7]);
    list3.print_heap();

    list3.drop_heap();
    list3.insert(8);
    list3.insert(2);
    list3.insert(5);
    list3.insert(12);
    list3.insert(1);
    list3.print_heap();
}
