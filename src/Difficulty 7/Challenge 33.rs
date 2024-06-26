mod read_input;
use std::u32;

//use capitalize::Capitalize;
use read_input::read_input;
//use std::collections::HashMap;

fn main() {

    let user: u32 = read_input("How many GCSE's do you have: ")
        .parse()
        .expect("Please enter a sensible positive integer");

    let mut gcses = 0;
    let mut c = 0;

    loop {
        if c == user {break;}

        let result: u32 = read_input("What is your GCSE result: ")
            .parse()
            .expect("Please enter a sensible positive ineger");

        match result {
            1..=9 => {c+=1; gcses+=result;},
            _ => { 
                println!("Result must be within a range of 1-9");
                break;
            }
        }

    }

    match gcses {
        x if x >=40 => println!("You can go to sixth form!"),
        35..=39 => println!("A discussion is needed"),
        _ => println!("Sorry not enough points!")
    }

    println!("Total points: {}", gcses);

    
    
    


}



 


