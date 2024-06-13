mod read_input;
use read_input::read_input;

fn main(){

    let r_name: String = "Muhammad".to_string();
    let user_n: String = read_input("What is your first name: ");
    
    if r_name == user_n {
        println!("Your cool!")
    } else {
        println!("Nice to meet you!")
    }

}
