fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    {
        let r1 = &mut s;
    }

    let r2 = &mut s;

    let r1 = &s;
    let r2 = &s;

    println!("{r1} and {r2}");
    
    let r3 = &mut s;
    println!("{r3}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
