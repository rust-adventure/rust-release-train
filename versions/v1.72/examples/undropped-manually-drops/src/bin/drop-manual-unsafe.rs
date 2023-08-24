use std::mem::ManuallyDrop;

fn drop_numbers(mut value: MyStuff) {
    println!("{:?}", value);
    // trying to drop normally will not compile
    // because of a partial move
    // std::mem::drop(ManuallyDrop::into_inner(value.numbers));

    unsafe {
        ManuallyDrop::drop(&mut value.numbers);
    }
    // This println! shows why ManuallyDrop::drop is unsafe
    // we shouldn't be accessing my_stuff.numbers here
    println!("{:?}", value);
}

fn main() {
    let mut my_stuff = MyStuff {
        name: "my numbers".to_string(),
        numbers: ManuallyDrop::new(MyNumbers(vec![
            0, 1, 2, 3, 4, 5,
        ])),
    };
    my_stuff.numbers.0[2] = 7;
    drop_numbers(my_stuff);
}

#[derive(Debug)]
struct MyStuff {
    name: String,
    numbers: ManuallyDrop<MyNumbers>,
}

#[derive(Debug)]
struct MyNumbers(Vec<i32>);
impl Drop for MyNumbers {
    fn drop(&mut self) {
        println!("Dropping Numbers! {:?}", self);
    }
}
