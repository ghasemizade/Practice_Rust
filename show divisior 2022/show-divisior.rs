//check divisior
fn main ()
{
    println! ("Enter a number :");
    let inputuser = get_input ();
    
    let mut newnum = inputuser;
    let mut sum = 0;
    
    for counter in 1..inputuser + 1
    {
        if newnum % counter == 0
        {
            println! ("Divide your number is = {}", counter);
        }
    }
}

fn get_input() -> i32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : i32 = line.trim().parse().unwrap();
    return number ;
}