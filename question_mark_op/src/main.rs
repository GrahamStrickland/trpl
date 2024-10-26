fn main() {
    let lines = "Hello, this is a line of text.\nThis is another one.";

    let char_opt = last_char_of_first_line(lines);

    match char_opt {
        Some(char) => println!("Last char of first line: '{char}'"),
        None => println!("Unable to find last char of first line in '{lines}'"),
    }

    let lines = "\nhi";

    let char_opt = last_char_of_first_line(lines);

    match char_opt {
        Some(char) => println!("Last char of first line: '{char}'"),
        None => println!("Unable to find last char of first line in '{lines}'"),
    }
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
