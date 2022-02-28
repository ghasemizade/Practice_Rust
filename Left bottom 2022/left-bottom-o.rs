//draw triangle
fn main ()
{
    println! ("Enter your number for draw triangle :");
    let mut inputuser = get_input ();
    
    let _n = inputuser;
    for counter in 0..inputuser
    {
        inputuser -= 1;
        for counter1 in 0..counter
        {
            print! ("  ");
        }
        for counter2 in 0..inputuser + 1
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