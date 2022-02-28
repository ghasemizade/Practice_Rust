fn main ()
{
    println! ("Enter a number for length of rectangle :");
    let length = get_input ();
    println! ("Enter a number for width of rectangle :");
    let width = get_input ();
    
    // the operation
    for i in 0..width
    {
        for j in 0..length
        {
            print! ("* ");
        }
        print! ("\n");
    }
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}