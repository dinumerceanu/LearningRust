mod my_threads;
use my_threads::*;

fn main() {
    main_thread();

    spawn_2();
}
