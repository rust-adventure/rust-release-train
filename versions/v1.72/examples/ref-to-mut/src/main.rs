fn main() {
    let n = 5;
    x(&n);
    dbg!(n);
}

#[deny(invalid_reference_casting)]
fn x(r: &i32) {
    unsafe {
        *(r as *const i32 as *mut i32) += 1;
    }
}
