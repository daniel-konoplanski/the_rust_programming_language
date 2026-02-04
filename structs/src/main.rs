#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle
{
    fn area(&self) -> u32
    {
        return self.width * self.height;
    }

    fn width(&self) -> u32
    {
        return self.width;
    }

    fn can_hold(&self, rec: &Rectangle) -> bool
    {
        if self.width > rec.width && self.height > rec.height
        {
            return true;
        }

        return false;
    }

    fn square(side: u32) -> Self
    {
        return Self { width: side, height: side };
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 60,
    };

    println!("rect2.can_hold(&rect1): {}", rect2.can_hold(&rect1));

    let square = Rectangle::square(5);

    println!("square area: {}", square.area());
}