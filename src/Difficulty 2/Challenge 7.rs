mod read_input;
use read_input::read_input;

fn main(){

    let dist: f64 = read_input("Please enter the distence (meters): ").parse().expect("Number too high or input nota number");
    let time: f64 = read_input("Please enter the time (seconds): ").parse().expect("Number too high or input nota number");

    println!("\nYour object is going at {:.2} m/s", calc_speed(dist, time));

}

fn calc_speed(dist: f64, time: f64) -> f64 {
    dist/time
}