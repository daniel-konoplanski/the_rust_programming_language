fn main()
{
    let greeting_msg = "Hello there";
    let msg_str = greeting_msg.to_string();

    let mut initial = String::from("Some str ref");

    initial = format!("{initial} Ok whats going on here lol");

    println!("{initial}");

    initial.push('X');
    initial.push_str(" Hi again!");

    println!("{initial}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{}", &s3);

    println!("{s1}, {s2}, {s3}");

    let indexing = String::from("Hello there");
    let hello = String::from("Здравствуйте");

    println!("indexing: {}", indexing.len());
    println!("hello: {}", hello.len());

    let chars = hello.chars();
    println!("{:?}", chars);
}
