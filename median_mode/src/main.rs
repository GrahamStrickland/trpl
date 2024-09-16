use std::collections::HashMap;

fn main() {
    let mut vals = Vec::new();
    print_median(&vals);
    print_mode(&vals);

    vals = vec![1, 2, 3];
    print_median(&vals);
    print_mode(&vals);

    vals = vec![1, 1, 1];
    print_median(&vals);
    print_mode(&vals);

    vals = vec![1, 1, 1, 2];
    print_median(&vals);
    print_mode(&vals);

    vals = vec![1, 20, 11, 2];
    print_median(&vals);
    print_mode(&vals);

    vals = vec![21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10];
    print_median(&vals);
    print_mode(&vals);

    vals = vec![2, 44, 11, 22, 5, 7, 88, 63, 2, 15, 94, 100];
    print_median(&vals);
    print_mode(&vals);
}

fn print_median(vals: &Vec<i32>) {
    let val = median(&vals);
    match val {
        Some(val) => println!("median of {vals:?}: {val}"),
        None => println!("median of {vals:?} could not be determined"),
    }
}

fn print_mode(vals: &Vec<i32>) {
    let val = mode(&vals);
    match val {
        Some(val) => println!("mode of {vals:?}: {val}"),
        None => println!("mode of {vals:?} could not be determined"),
    }
}

fn median(vals: &Vec<i32>) -> Option<f64> {
    let pos = vals.len() / 2;

    let mut sorted = vals.clone();
    sorted.sort();

    if pos == 0 {
        None
    } else if pos % 2 == 0 {
        Some((f64::from(sorted[pos - 1]) + f64::from(sorted[pos])) / 2.0)
    } else {
        Some(f64::from(sorted[pos])) as Option<f64>
    }
}

fn mode(vals: &Vec<i32>) -> Option<i32> {
    let mut counts = HashMap::new();

    if vals.len() == 0 {
        return None;
    }

    for val in vals {
        let count = counts.entry(val).or_insert(0);
        *count += 1;
    }

    let mut max_key = 0;
    let mut max_value = 0;
    for (key, value) in &counts {
        if value > &max_value {
            max_key = **key;
            max_value = *value;
        }
    }

    Some(max_key) as Option<i32>
}
