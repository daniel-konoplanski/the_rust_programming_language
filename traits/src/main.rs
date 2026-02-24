pub trait Area
{
    fn area(&self) -> f64;
}

struct Rectangle
{
    x: f32,
    y: f32,
}

impl Area for Rectangle
{
    fn area(&self) -> f64
    {
        return (self.x * self.y) as f64;
    }
}

fn main() {
    println!("Hello, world!");
}
