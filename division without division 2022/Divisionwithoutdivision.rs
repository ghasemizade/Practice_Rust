fn main ()
{
    println! ("Enter a first number :");
    let firstnumber = get_input ();
    println! ("Enter a second number :");
    let secondnumber = get_input ();
    
    
    let mut newfirst = firstnumber;
    let mut division = 0;
    
    for counter in 0..secondnumber
    {
        if newfirst >= secondnumber
        {
            division += 1;
            newfirst -= secondnumber;
        }
        else
        {
            break;
        }
    }
    println! ("Division : {}", division);
}

fn get_input() -> i32{

    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}