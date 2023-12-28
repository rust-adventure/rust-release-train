// https://doc.rust-lang.org/stable/core/option/enum.Option.html#method.as_slice
fn main() {
    let something: Option<u32> = Some(3);
    // None results in a sum of 0
    // let something: Option<u32> = None;
    let result = sum_slice(something.as_slice());
    dbg!(result);

    //
    let thing = Thing {
        name: "chris".to_string(),
    };
    let some_str: Option<&Thing> = Some(&thing);
    let result = count_sizes(
        some_str.map_or(&[], std::slice::from_ref),
    );
    dbg!(result, thing.name);
}

fn sum_slice(arr: &[u32]) -> u32 {
    arr.iter().map(|value| value * 5).sum()
}

struct Thing {
    name: String,
}
fn count_sizes(things: &[Thing]) -> Vec<usize> {
    things
        .iter()
        .map(|thing| thing.name.len())
        .collect::<Vec<usize>>()
}
