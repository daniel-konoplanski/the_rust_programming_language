pub trait Area
{
    fn area(&self) -> f64;
}

struct Rectangle
{
    x: f32,
    y: f32,
}

impl Rectangle
{
    fn new(x: f32, y: f32) -> Rectangle
    {
        return Rectangle{x, y};
    }
}

impl Area for Rectangle
{
    fn area(&self) -> f64
    {
        return (self.x * self.y) as f64;
    }
}

fn print_area(shape: &impl Area)
{
    println!("{:.2}", shape.area());
}

fn print_area2<T: Area>(shape: &T)
{
    println!("{:.2}", shape.area());
}

fn main()
{
    let rec = Rectangle::new(4.0, 5.0);
    print_area(&rec);
    print_area2(&rec);
}
