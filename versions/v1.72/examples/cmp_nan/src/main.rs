fn main() {
    let a = 2.3f32;
    if a == f32::NAN {
        println!("true");
    } else {
        println!("false");
    }
}
