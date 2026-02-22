fn largest<T: PartialOrd>(list: &[T]) -> &T
{
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    return largest;
}

struct Point<T>
{
    x: T,
    y: T,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");


    let int_point = Point {x: 5, y: 7};
    let float_point = Point {x: 5.0, y: 7.0};
}