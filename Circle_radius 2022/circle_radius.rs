use std::io;

fn main(){
    println!("Enter a number for radius of the circle ");
    let inputuser = get_input ();
    
    let number: f32 = 2.0;
 
    println! ("Diameter = {}", inputuser * number);
    println! ("Enviroment = {}", number * 3.14 * inputuser);
    println! ("Area = {}", 3.14 * inputuser * inputuser);
}

fn get_input() -> f32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : f32 = line.trim().parse().unwrap();
    return number ;
}
