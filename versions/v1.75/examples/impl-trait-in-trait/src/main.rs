trait Characters {
    fn characters(&self) -> impl Iterator<Item = char>;
}

struct ManyCharacters(String);

impl Characters for ManyCharacters {
    fn characters(&self) -> impl Iterator<Item = char> {
        self.0.chars()
    }
}

struct ManyNumbers(Vec<u8>);

impl Characters for ManyNumbers {
    fn characters(&self) -> impl Iterator<Item = char> {
        self.0.iter().map(|num| char::from(*num))
    }
}

fn main() {
    let value = ManyCharacters("abcdefghijkl".to_string());

    for c in value.characters().enumerate() {
        dbg!(c);
    }

    let value =
        ManyNumbers(vec![90, 83, 59, 200, 123, 100]);

    for d in value.characters().enumerate() {
        dbg!(d);
    }
}
