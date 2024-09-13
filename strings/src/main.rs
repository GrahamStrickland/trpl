fn main() {
    let mut s = String::new();

    let data = "initial contents";
    println!("{s}");
    s = data.to_string();
    println!("{s}");

    s = "initial_contents".to_string();
    println!("{s}");

    s = String::from("initial_contents");
    println!("{s}");

    s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    s = String::from("lo");
    s.push('l');
    println!("{s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("{s}");

    for c in hello.chars() {
        println!("{c}");
    }

    for b in hello.bytes() {
        println!("{b}");
    }
}
