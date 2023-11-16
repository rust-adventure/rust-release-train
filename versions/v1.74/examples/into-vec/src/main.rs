use std::rc::Rc;
use std::sync::Arc;

fn main() {
    let arr: [char; 10] = ['a'; 10];
    println!("{:?}", arr);

    // let my_vec: Vec<char> = Vec::from(&arr);
    // println!("{:?}", my_vec);

    // this is ambiguous now
    let my_arc = Arc::from(arr);
    let my_arc: Arc<[char]> = Arc::from(arr);
    let my_arc2: Arc<[char; 10]> = Arc::from(arr);
    println!("{:?}", my_arc);
    println!("{:?}", my_arc2);

    // this is ambiguous now
    let my_rc = Rc::from(arr);
    let my_rc: Rc<[char]> = Rc::from(arr);
    let my_rc2: Rc<[char; 10]> = Rc::from(arr);
    println!("{:?}", my_rc);
    println!("{:?}", my_rc2);
}
