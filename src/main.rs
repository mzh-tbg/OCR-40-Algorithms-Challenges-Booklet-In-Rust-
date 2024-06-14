mod read_input;

//use capitalize::Capitalize;
use read_input::read_input;
//use std::collections::HashMap;

fn main() {

    let mut lives = 3;
    let mut level = 0;
    let mut points = 0;

    loop {
        if points == 20 || lives == 0 {break;}
        let question: u32 = read_input("Did you:\n1- Lose a life\n2-Gain a point: ")
            .parse().expect("Please enter a sensible positive ineger!");
        
        match question {
            1 => {
                lives -= 1;
                println!("You now have a total of {} lives", lives);
            },
            2 => {
                points += 1;
                if points % 5 == 0 {
                    level +=1;
                    println!("You have gotten up a level!")
                }
                println!("Total points {}", points);
            }
            _ => {
                println!("Enter a number fom 1-2!");
                continue; 
            }
        }


    }
    

    println!("You have gotten to level ({}) with {} lives and {} points!", level, lives, points);

}



 

