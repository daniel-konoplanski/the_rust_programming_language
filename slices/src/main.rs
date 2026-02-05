use std::array;

fn main() {

    let array: [i32; 20] = std::array::from_fn(|i| (i + 1) as i32);

    let slice = &array[10..20];

    println!("array: {:?}", array);
    println!("slice: {:?}", slice);
}
