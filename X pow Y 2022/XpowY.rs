fn main() 
{
    println! ("Enter x :");
    let x = get_input ();
    println! ("Enter y :");
    let y = get_input ();
    let mut product = 0;
    for  counter in 0..y 
    {
        product = (x*x);
    }
    println!("{}", product)
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}