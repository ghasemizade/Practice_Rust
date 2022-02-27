fn main()
{
    println!("Enter the first number :");
    let firstinput = get_input ();
    println!("Enter the second number :");
    let secondinput = get_input ();
    let mut product = 1;
    for counter in firstinput..secondinput 
    {
        if counter % 2 == 1 
        {
            product *= counter; 
        }
    }
    println!("odd between product :{}", product)
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}