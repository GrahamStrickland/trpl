use rand::Rng;

fn main() {
    let dice_roll = rand::thread_rng().gen_range(1..=12);

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn add_fancy_hat() {
    println!("You've been awarded a fancy hat!");
}

fn remove_fancy_hat() {
    println!("Sorry, your fancy hat was removed.");
}
