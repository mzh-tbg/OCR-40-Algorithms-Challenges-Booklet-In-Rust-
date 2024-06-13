mod read_input;
use capitalize::Capitalize;

use read_input::read_input;

fn main(){

    let user: String = read_input("What is the trafic light colour: ").capitalize();

    match user.as_str() {
        "Green" => println!("\nGo!"),
        "Amber" => println!("\nGet Ready"),
        "Red" => println!("\nStop"),
        _ => println!("\nWrong input!")
    }
    

}


