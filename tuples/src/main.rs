use core::fmt;

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3);
    }
}

fn main() {
    let matrix = Matrix(1.0, 2.0, 3.0, 4.0);
    println!("{}", matrix);
}
