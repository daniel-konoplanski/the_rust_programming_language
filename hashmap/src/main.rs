use std::collections::HashMap;

fn main()
{
    let mut map = HashMap::new();

    map.insert(String::from("X"), 21);
    map.insert(String::from("Y"), 26);

    {
        let weronika = String::from("Z");
        map.insert(weronika, 23);
    }

    for (name, age) in &map {
        println!("name: {name}, age: {age}");
    }

    let option = map.get("X");

    let Some(age) = option else {
        println!("Nothing found");
        return;
    };

    println!("age: {}", age);

    map.entry(String::from("X")).or_insert(40);

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
}
