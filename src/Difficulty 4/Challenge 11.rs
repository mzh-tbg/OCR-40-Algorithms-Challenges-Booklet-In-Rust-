mod read_input;
use std::cmp::Ordering;

use read_input::read_input;

fn main(){

    let user: i32 = read_input("What is your first number: ").parse().expect("Please enter a number");
    let user2: i32 = read_input("What is your second number: ").parse().expect("Please enter a number");

    match user.cmp(&user2) {
        Ordering::Greater => println!("{} is the greater number!", user),
        Ordering::Less => println!("{} is the greater number!", user2),
        Ordering::Equal => println!("Both numbers are equal!"),
    }

}
