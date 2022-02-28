fn main() 
{
    println!("Enter the number of star :");
    let star = get_input();
    for counter in 0..star 
    {
        println!("*")   
    }
}
fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}