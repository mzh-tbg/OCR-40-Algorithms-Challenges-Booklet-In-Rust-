mod read_input;
//use capitalize::Capitalize;

use read_input::read_input;

fn main(){

    let mut dimentions: Vec<f64> = Vec::new();
    let f_bed: f64 = read_input("What is the radius of your turf (meters): ").parse().expect("Please enter a sensible radius");

    for _x in 0..=1{
        let user: f64 = read_input("What is the dimentions of your lawn (in meters)").parse().expect("Please enter sensible dimentions!");
        dimentions.push(user)
    }
    
    println!("You need {:.2} m^2 area of turf!", ( ( dimentions[1] * dimentions[0] ) - ( f_bed.powi(2) * std::f64::consts::PI ) ) );

    
    

}



