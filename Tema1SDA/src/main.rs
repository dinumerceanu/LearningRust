mod utils;
use utils::*;

use crate::utils::double_linked_list::{Search, Showcase, Update};


fn main() {
    let mut train = double_linked_list::List::new();

    train.add_cart('a');
    train.add_cart('b');
    train.add_cart('c');
    train.add_cart('d');
    train.add_cart('e');
    train.add_cart('f');
    train.move_right();
    train.move_right();
    train.move_right();
    train.print_list();
    train.search_right(String::from("a"));
    train.print_list();
    train.search(String::from("bcd"));
    train.print_list();
}
