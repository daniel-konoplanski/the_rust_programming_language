fn main() {
    let mut vec = Vec::<i32>::new();

    vec.push(5);

    let vec2 = vec![1, 2, 3, 4, 5, 6];

    println!("{:?}", vec);
    println!("{:?}", vec2);

    let third = &vec2[2];

    let thirdO = vec2.get(2);

    if let Some(result) = thirdO {
        println!("Found result {}", result);
    };

    // let None = thirdO else {
    //     println!("thirdO not equal to None");
    //     return;
    // };

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    let mut vec1 = Vec::<&i32>::new();

    let some_var1 = 32;
    let some_var2 = 33;

    vec1.push(&some_var1);

    let some_var1 = "a";
    let some_var2 = "b";

    println!("{}", vec1[0]);
}
