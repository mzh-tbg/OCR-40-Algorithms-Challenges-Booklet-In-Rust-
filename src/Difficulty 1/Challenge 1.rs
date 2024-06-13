use read_input::read_input;

mod read_input;

fn main() {

    let name = read_input("What is your name: ");

    let age: i64 = read_input("What is your age: ").parse().expect("Please enter a valid number!");

    let colour = read_input("What is your favoruite colour :");

    print!("\n\nInformation:\nName - {}\nAge - {}\nFavoruite Colour: {}", name,age,colour);
    
}
