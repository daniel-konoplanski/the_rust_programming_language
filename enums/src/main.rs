#[derive(Debug)]
enum IpAddrKind {
    V4((u8, u8, u8, u8)),
    V6(String),
}

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting..."),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(text) => println!("Writing: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
        }
    }
}

fn division(number: i32) -> Option<u32>
{
    if number == 0
    {
        return Option::<u32>::None;
    }

    return Option::Some(number.isqrt().try_into().unwrap());
}


fn main() {
    let ip = IpAddrKind::V4((127, 0, 0, 1));

    println!("ip: {:?}", ip);

    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    assert_eq!("127.0.0.1".parse(), Ok(localhost_v4));
    assert_eq!("::1".parse(), Ok(localhost_v6));

    assert_eq!(localhost_v4.is_ipv6(), false);
    assert_eq!(localhost_v4.is_ipv4(), true);

    let message = Message::Move { x: 4, y: 5 };

    // ________
    let m = Message::Write(String::from("hello"));
    m.call();
}
