const fn do_the_most() {
    loop {}
}

#[allow(long_running_const_eval)]
const NOTHING: () = do_the_most();

pub fn main() {
    println!("here");
}
