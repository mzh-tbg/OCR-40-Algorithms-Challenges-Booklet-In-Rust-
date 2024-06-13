mod read_input;
use read_input::read_input;

fn main(){

    let minutes: f64 = read_input("How many minutes have you used: ").parse().expect("Error make sure the input is a number or the number is too big");
    let texts: f64 = read_input("How many texts have you used: ").parse().expect("Error make sure the input is a number or the number is too big");
    
    println!("Your total cost is: {}", total_cost(minutes, texts));

}

fn total_cost(minutes: f64, texts: f64) -> f64 {
    (minutes*0.10) + (texts*0.05) + 10.00
}