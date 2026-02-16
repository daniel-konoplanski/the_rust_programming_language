use std::collections::HashMap;

fn main()
{
    let mut map = HashMap::<String, i32>::new();

    map.insert(String::from("X"), 21);
    map.insert(String::from("Y"), 26);

    {
        let weronika = String::from("Z");
        map.insert(weronika, 23);
    }

    for (name, age) in map
    {
        println!("name: {name}, age: {age}");
    }
}
