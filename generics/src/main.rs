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

fn main()
{
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&number_list));
}
