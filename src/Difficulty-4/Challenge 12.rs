mod read_input;
use rand::{self, Rng};

use read_input::read_input;

fn main(){

    let random_num = rand::thread_rng().gen_range(1..=10);
    let user_num: i32 = read_input("Guess a number from 1-10: ").parse().expect("Please enter a number!");

    if user_num == random_num {
        println!("Correct");
    } else {
        println!("Not what I was thinking!");
    }
}
