fn main()
{
    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 5;

    let y: i32 = 1_000_000;

    println!("guess: {}", guess);

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("cat emoji lol: {}", heart_eyed_cat);

    let isThisCalled: bool = false;

    println!("isThisCalled: {}", isThisCalled);

    tuples();
}

fn tuples()
{
    let tp = (4, 5, 6);

    println!("tp: {:?}", tp);

    let (x, y, z) = tp;

    println!("x: {}, y: {}, z: {}", x, y, z);

    println!("tp.1: {}", tp.1);
}
