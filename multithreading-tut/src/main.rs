mod my_threads;
use my_threads::*;

mod scoped_threads;
use scoped_threads::*;

mod mutex;
use mutex::*;

mod my_channels;
use my_channels::*;

fn main() {
    // main_thread();

    // spawn_2();

    // named_threads();

    // scoped_threads_demo();

    // test_mutex();

    test_channels();
}
