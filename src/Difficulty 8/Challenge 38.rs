mod read_input;
//use capitalize::Capitalize;
use read_input::read_input;

fn main() {
    let i_amount: f64 = read_input("What is the starting point of your interest amount: ").parse().expect("Please enter a valid number");

    let years: i32 = read_input("How many years: ").parse().expect("Please enter a valid number");

    let mut c_amount: f64 = i_amount;

    println!("{:<5} {:<10} {:<10} {:<10} {:<10}", "Year", "Start", "Initial", "Interest", "End");

    for year in 1..=years {
        let interest = c_amount * 0.10;
        let end_amount = c_amount + interest;

        println!("{:<5} {:<10.2} {:<10.2} {:<10.2} {:<10.2}", year, c_amount, i_amount, interest, end_amount);

        c_amount = end_amount + 100.00; 
    }
}



 

