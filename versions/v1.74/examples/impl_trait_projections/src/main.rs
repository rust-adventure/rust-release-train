#[derive(Debug)]
struct Wrapper<'a, T>(&'a T);

// Opaque return types that mention `Self`:
impl Wrapper<'_, ()> {
    fn impl_trait() -> impl Iterator<Item = Self> {
        vec![Wrapper(&())].into_iter()
    }
}

trait Trait<'a> {
    type Assoc;
    fn new() -> Self::Assoc;
}
impl Trait<'_> for () {
    type Assoc = ();
    fn new() -> () {}
}

// Opaque return types that mention an associated type:
impl<'a, T: Trait<'a>> Wrapper<'a, T> {
    fn a_few_assocs() -> impl Iterator<Item = T::Assoc> {
        vec![].into_iter()
    }
}

fn main() {
    let option_1: Vec<_> =
        Wrapper::<'_, ()>::impl_trait().collect();
    let option_2: Vec<_> =
        Wrapper::<'_, ()>::a_few_assocs().collect();
    dbg!(option_1, option_2);
}
