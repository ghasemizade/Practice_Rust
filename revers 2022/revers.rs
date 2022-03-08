fn main()
{
    //get input
    println! ("Enter a number for check symmetrical number: ");
    let mut inputuser = get_input();
    
    //define variables
    let mut num;
    let mut revers = 0;
    let save = inputuser;
    
    // the operation
    for counter in 0..inputuser
    {
        num = inputuser % 10;
        inputuser = inputuser / 10;
        revers = revers * 10 + num;
    }
    
    // output
    if save == revers
    {
        println! ("This number is a symmetrical number.");
    }
    else
    {
        println! ("This number isn't a symmetrical number.");
    }
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}