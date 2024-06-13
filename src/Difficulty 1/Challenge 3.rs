mod read_input;
use read_input::read_input;

fn main(){

    let surname = read_input("What is your surname: ");
    let name = read_input("What is your first name: ");

    println!("\n\n{}", name);
    println!("{}", surname);

}