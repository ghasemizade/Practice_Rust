fn main ()
{
    //this program about check number perfect number or not!?
    println! ("Enter a number :");
    let intpuuser = get_input ();
    
    
    let mut saveinput = intpuuser;
    let mut sum = 0;
    
    for counter in 1..intpuuser
    {
        if saveinput % counter == 0
        {
            sum += counter;
        }
    }
    
    // output
    if sum == intpuuser
    {
        println! ("Yes");
    }
    else
    {
        println! ("No");
    }
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}