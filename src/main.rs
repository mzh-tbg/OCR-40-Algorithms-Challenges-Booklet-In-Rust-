mod read_input;
//use capitalize::Capitalize;
use read_input::read_input;

fn main() {

    let mut fruits: Vec<f64> = Vec::new();
    
    let user: u32 = read_input("How many fruits do you have: ")
        .parse()
        .expect("Please enter a sensible number");

    let mut count = 0;

    loop {
        if count == user {break;}

        let fruit_w: i64 = read_input("What is the whight of your fruit to the nearest grams: ")
            .parse()
            .expect("Please enter a sensible number!");

        if fruit_w < 0 {println!("Enter a positive whole number!");continue;}
        else {
            count += 1;
            fruits.push(fruit_w as f64);
        }
    }

    let sum: f64 = fruits.iter().sum();

    println!("Average weight for the fruits is {}", sum/(user as f64));
    
    



}



 

