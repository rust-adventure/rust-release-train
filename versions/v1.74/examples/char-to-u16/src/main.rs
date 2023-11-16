fn main() {
    let my_chars = vec!['0', 'c', 'ÿ', 'Ϣ'];

    for c in my_chars.into_iter() {
        let my_u16 = u16::try_from(c).unwrap();
        dbg!(my_u16);
    }
}
