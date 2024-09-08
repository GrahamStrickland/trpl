fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v2 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v2[100];
    // let does_not_exist = v2.get(100);
    
    let mut v3 = vec![1, 2, 3, 4, 5];

    let first = &v3[0];

    // v3.push(6);

    println!("The first element is: {first}");

    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
        println!("{i}");
    }
}
