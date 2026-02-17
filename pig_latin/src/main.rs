// Convert strings to Pig Latin. The first consonant of each word is moved to the end
// of the word and ay is added, so first becomes irst-fay. Words that start with a
// vowel have hay added to the end instead (apple becomes apple-hay).
// Keep in mind the details about UTF-8 encoding!

const VOWELS: [char; 6]= ['a', 'e', 'i', 'o', 'u', 'y'];

fn to_pig_latin(sentence: &str) -> String {
    let words = sentence.split_whitespace();

    let mut pig_latin = String::new();

    for word in words {
        let maybe_first_letter = word.chars().next();

        let Some(first_letter) = maybe_first_letter else {
            continue;
        };

        let is_vowel = VOWELS.contains(&first_letter);

        if is_vowel
        {
            pig_latin += &format!("{word}-hay ");
        }
        else
        {
            pig_latin += &format!("{}-{}ay ", &word[1..], first_letter);
        }
    };

    return pig_latin.trim().to_string();
}

fn main()
{
    let sentence = "hello there what is going on about";

    println!("{}", to_pig_latin(sentence));
}
