
fn main()
{
    const TIME: u32 = 60 * 8 * 20;

    println!("{TIME}");

    let x: i32 = 5;
    let y: i32;
    y = 9;

    println!("x: {}", x);
    println!("y: {}", y);

    // ____________________

    let shadow: u64 = 8888888;

    println!("shadow: {shadow}");

    let shadow: i32 = -43443;

    println!("shadow: {shadow}");

    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";

    println!("pangram: {pangram}");
}
