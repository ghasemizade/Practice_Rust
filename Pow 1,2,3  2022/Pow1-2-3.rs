use std::io;

fn main() 
{
    println! ("Enter the number :");
    let inputuser = get_input ();

    println! ("Power of 1 = {}", inputuser);
    println! ("Power of 2 = {}", inputuser * inputuser);
    println! ("Power of 3 = {}", inputuser * inputuser * inputuser);
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}