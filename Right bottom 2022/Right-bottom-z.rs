fn main ()
{
    println! ("Enter a number for draw a triangle :");
    let mut inputuser = get_input ();
    let mut n = inputuser;
    
    for counter in 0..inputuser
    {
        for coutner1 in 0..inputuser
        {
            print! ("* ");
        }
        inputuser -= 1;
        
        if inputuser == 0
        {
            break;
        }
        print! ("\n");
    }
}

fn get_input() -> i32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}