fn main() {
    let vals = vec![2, 44, 11, 22, 5, 7, 88, 63, 2, 15, 94, 100];

    let val = median(&vals);
    match val {
        Some(val) => println!("median of {vals:?}: {val}"),
        None => println!("median of {vals:?} could not be determined"),
    }

    let val = mode(&vals);
    match val {
        Some(val) => println!("mode of {vals:?}: {val}"),
        None => println!("mode of {vals:?} could not be determined"),
    }
}

fn median(vals: &Vec<i32>) -> Option<&i32> {
    let pos = vals.len() / 2;

    let mut sorted = vals.clone();
    sorted.sort();

    let val = sorted.get(pos);

    match val {
        Some(val) => return Some(val),
        None => return None,
    }
}

fn mode(vals: &Vec<i32>) -> Option<&i32> {
    return vals.get(0);
}
