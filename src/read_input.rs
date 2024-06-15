pub fn read_input(prompt: &str) -> String {

    let mut result = String::new();

    println!("{}", prompt);
    
    std::io::stdin()
        .read_line(&mut result)
        .expect("Could not read line!");

    result.trim().to_string()

}