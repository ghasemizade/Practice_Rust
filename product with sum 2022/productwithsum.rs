fn main()
{
    println!("Enter first number :");
    let firstinput = get_input ();
    println!("Enter second number :");
    let secondinput = get_input ();
    let mut sum = 0;
    for counter in 0..secondinput 
    {
        sum += firstinput;   
    }
    println!("result :{}", sum);
}


fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}