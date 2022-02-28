fn main ()
{
    println! ("Enter a number radius :");
    let inputuser = get_input ();
    
    let mut temp = 2.0 * 3.14 * inputuser;
    let mut variable = temp / inputuser;
    

    println! ("The expansion of pi number is = {}", variable);
}

fn get_input() -> f32
{
    let mut line  = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let number : f32 = line.trim().parse().unwrap();
    return number ;
}