mod read_input;
use read_input::read_input;

fn main(){

    let user: i32 = read_input("How many numbers are there in the alphabet: ").parse().expect("Please enter a number");

    if user == 26 {
        println!("You are correct!");
    } else {
        println!("You are wrong :(");
    }

}
