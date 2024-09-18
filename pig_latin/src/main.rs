fn main() {
    let mut word = "first";
    println!("Before: {word}");
    let mut new_word = to_pig_latin(word);
    println!("After: {new_word}");

    word = "apple";
    println!("Before: {word}");
    new_word = to_pig_latin(word);
    println!("After: {new_word}");

    word = "Здравствуйте";
    println!("Before: {word}");
    new_word = to_pig_latin(word);
    println!("After: {new_word}");

    word = "Olá";
    println!("Before: {word}");
    new_word = to_pig_latin(word);
    println!("After: {new_word}");
}

fn to_pig_latin(word: &str) -> String {
    let mut new_word = String::new();
    let mut ending = String::new();
    let mut first_found = false;

    for c in word.chars() {
        if !first_found {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                    ending = String::from("-hay");
                    new_word.push(c);
                }
                _ => ending = format!("-{c}ay"),
            }
            first_found = true;
        } else {
            new_word.push(c);
        }
    }
    new_word.push_str(&ending);

    return new_word;
}
