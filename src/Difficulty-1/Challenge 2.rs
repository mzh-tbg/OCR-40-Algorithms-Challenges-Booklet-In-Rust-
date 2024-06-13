mod read_input;
use read_input::read_input;

fn main(){

    let first_name = read_input("What is your first name: ");

    println!("Your first name is: {}", first_name);

}