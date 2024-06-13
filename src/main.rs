mod read_input;


use read_input::read_input;

fn main(){

    let user: u32 = read_input("How many days a week do you work: ").parse().expect("Please enter a number between 1-5");
    
    println!("You get {} days of holidays! ", (holidays(user) as u32));

}

fn holidays(days: u32) -> f64 {
    ((days as f64) / 5.0) * 28.0
}
