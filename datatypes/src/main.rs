use std::io;

fn main() {
    let x = 2.0; // f64
    println!("x = {x}");

    let y: f32 = 3.0; // f32
    println!("y = {y}");

    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {difference}");

    // multiplication
    let product = 4 * 30;
    println!("4 * 30 = {product}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    println!("56.7 / 32.2 = {quotient}");
    println!("-5 / 3 = {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {remainder}");

    let t = true;
    println!("t = {t}");

    let f: bool = false;    // with explicit type annotation
    println!("f = {f}");

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c = {c}, z = {z}, heart_eyed_cat = {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    println!("x = {x}, y = {y}, z = {z}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("five_hundred = {five_hundred}, six_point_four = {six_point_four}, one = {one}");

    let a = [1, 2, 3, 4, 5];
    for i in a {
        println!("i = {i}");
    }

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    for month in months {
        println!("month = {month}");
    }

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    for i in a {
        println!("i = {i}");
    }

    let a = [3; 5];
    for i in a {
        println!("i = {i}");
    }

    let a = [1, 2, 3, 4, 5];
    for i in a {
        println!("i = {i}");
    }

    let first = a[0];
    let second = a[1];

    println!("first = {first}, second = {second}");

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
