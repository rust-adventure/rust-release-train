use std::mem::ManuallyDrop;

#[cfg(feature = "manually-drop")]
#[derive(Debug)]
struct MyStuff {
    name: String,
    numbers: ManuallyDrop<MyNumbers>,
}

fn drop_numbers(value: MyStuff) {
    println!("{:?}", value);
    // unsafe {
    //     ManuallyDrop::drop(&mut value.numbers);
    // }
}

fn main() {
    cfg!(not(feature="manually-drop"))
    let mut my_stuff = MyStuff {
        name: "my numbers".to_string(),
        numbers: ManuallyDrop::new(MyNumbers(vec![
            0, 1, 2, 3, 4, 5,
        ])),
    };
    cfg!(not(feature="manually-drop"))
    let mut my_stuff = MyStuff {
        name: "my numbers".to_string(),
        numbers: ManuallyDrop::new(MyNumbers(vec![
            0, 1, 2, 3, 4, 5,
        ])),
    };
    
    my_stuff.numbers.0[2] = 7;
    drop_numbers(my_stuff);
    // This println! shows why ManuallyDrop::drop is unsafe
    // we shouldn't be accessing my_stuff.numbers here
    // println!("{:?}", my_stuff);
}

#[derive(Debug)]
struct MyNumbers(Vec<i32>);
impl Drop for MyNumbers {
    fn drop(&mut self) {
        println!("Dropping Numbers! {:?}", self);
    }
}
