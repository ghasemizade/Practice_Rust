fn main ()
{
    println! ("Enter a number for draw triangle :");
    let inputuser = get_input ();
    
    for counter in 0..inputuser + 1
    {
        for counter1 in 0..counter
        {
            print! ("* ");
        }
        print! ("\n");
        
        for counter2 in counter..inputuser + 1
        {
            print! ("  ");
        }
    }
    print! ("\n");
}

fn get_input() -> i32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}