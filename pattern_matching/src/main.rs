use core::num;

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState
{
    Alabama,
    Alaska,
}

enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8
{
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

fn add_one(number: Option<i32>) -> Option<i32>
{
    let result = match number {
        None => None,
        Some(value) => Some(value + 1),
    };

    return result;
}

fn main()
{
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    println!("{:?}", add_one(Some(10)));
    println!("{:?}", add_one(None));
}
