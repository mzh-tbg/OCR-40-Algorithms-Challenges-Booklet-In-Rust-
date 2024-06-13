mod read_input;
use read_input::read_input;

fn main(){

    let num1: i64 = read_input("Please enter a number: ").parse().expect("Number too high or input nota number");
    let num2: i64 = read_input("Please enter a number: ").parse().expect("Number too high or input nota number");

    println!("\n{} + {} = {}", num1, num2, (num1+num2));
    println!("\n{} * {} = {}", num1, num2, (num1*num2));

}