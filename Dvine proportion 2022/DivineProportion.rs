fn main() 
{
    println!("Enter number :");
    let inputuser = get_input();
    
    println!("The divine ratio of your input number :{}", inputuser / 1.618);
    println!("The divine ratio of your input number :{}", inputuser * 1.618);
}
fn get_input() -> f32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : f32 = line.trim().parse().unwrap();
    return number ;
}