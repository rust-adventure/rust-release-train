fn main() {
    let a = unsafe {
        std::str::from_utf8_unchecked(b"cl\x82ippy")
    };

    let b = std::str::from_utf8(b"ru\x82st");
    dbg!(a, b);
}
