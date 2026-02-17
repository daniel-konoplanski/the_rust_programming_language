// Given a list of integers, use a vector and return the median
// (when sorted, the value in the middle position) and mode
// (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn median(vec: &[i32]) -> f32
{
    let len = vec.len();
    let mid = len / 2;

    if len % 2 == 0 {
        (vec[mid - 1] + vec[mid]) as f32 / 2.0
    } else {
        vec[mid] as f32
    }
}

fn mode(vec: &[i32]) -> i32
{
    let mut map = HashMap::new();

    for i in vec {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let (key, _) = map.iter().max_by_key(|(_, v)| *v).unwrap();

    return **key;
}

fn main()
{
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let vec2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let result = median(&vec);
    println!("__________");
    let result2 = median(&vec2);

    println!("median: {result}");
    println!("median2: {result2}");
}
