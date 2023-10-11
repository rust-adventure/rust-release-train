use std::{cell::Cell, thread, time::Duration};

thread_local! {
    static THINGS: Cell<Vec<i32>> = Cell::new(Vec::new());
}

fn main() {
    THINGS.set(vec![5, 10, 15]);

    thread::spawn(|| {
        dbg!(THINGS.take());
        for i in 1..10 {
            THINGS.set(vec![i, i * 2]);
            thread::sleep(Duration::from_millis(1));
        }
        dbg!(THINGS.take());
        dbg!(THINGS.take());
    })
    .join()
    .expect("thread to have completed successfully");

    dbg!(THINGS.take());
}
