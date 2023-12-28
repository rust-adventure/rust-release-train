use std::{collections::VecDeque, io::BufRead};

fn main() {
    let input = VecDeque::from([
        b'a', b'b', b'\n', b'c', b'd', b'\n', b'e',
    ]);
    for line in input.lines() {
        println!("{}", line.unwrap());
    }
}
