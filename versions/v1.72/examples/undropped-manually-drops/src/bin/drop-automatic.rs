fn drop_numbers(value: MyStuff) {
    println!("{:?}", value);
}

fn main() {
    let mut my_stuff = MyStuff {
        name: "my numbers".to_string(),
        numbers: MyNumbers(vec![0, 1, 2, 3, 4, 5]),
    };
    my_stuff.numbers.0[2] = 7;
    drop_numbers(my_stuff);
}

#[derive(Debug)]
struct MyStuff {
    name: String,
    numbers: MyNumbers,
}

#[derive(Debug)]
struct MyNumbers(Vec<i32>);
impl Drop for MyNumbers {
    fn drop(&mut self) {
        println!("Dropping Numbers! {:?}", self);
    }
}
